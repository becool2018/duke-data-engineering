use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// ---------------------------------------------------------------------------
// LEARNING NOTE: Why Arc<Mutex<T>>?
//
// We need to share the client list across multiple threads. In Rust, you
// cannot share a plain Vec across threads because the compiler enforces that
// only one owner exists at a time (ownership rules).
//
//   Arc  = Atomically Reference Counted. Lets multiple threads hold a pointer
//          to the same data. Cloning an Arc just bumps a counter - it does NOT
//          copy the underlying data.
//
//   Mutex = Mutual Exclusion. Only one thread can "lock" it at a time.
//           Trying to lock when another thread holds it → your thread sleeps
//           until it's released. This prevents data races.
//
// Together: Arc<Mutex<T>> is the "safe shared mutable state" pattern in Rust.
// You will use this constantly. Learn to love it and fear it equally.
// ---------------------------------------------------------------------------

// Each connected client gets a handle so we can write back to them.
// We wrap TcpStream in Arc<Mutex<>> so multiple threads can write to it.
type ClientHandle = Arc<Mutex<TcpStream>>;

// The shared list of all connected clients.
// Arc lets every client thread hold a reference to this same list.
type ClientList = Arc<Mutex<Vec<ClientHandle>>>;

fn main() -> std::io::Result<()> {
    // Create a TCP listener on localhost:8080
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr)?;
    println!("Server listening on {}", addr);

    // Create the shared client list. This single instance will be shared
    // (via Arc clones) with every client thread we spawn.
    let clients: ClientList = Arc::new(Mutex::new(Vec::new()));

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                // Get the peer address before we move the stream.
                let peer = stream.peer_addr()?;
                println!("[server] New connection from {}", peer);

                // Wrap the stream so it can be shared across threads.
                let client_handle = Arc::new(Mutex::new(
                    // TcpStream::try_clone gives us a second OS-level handle
                    //to the same socket. We keep one for writing (in the
                    // client list) and use the original for reading below.
                    //
                    // LEARNING NOTE: We can't just clone TcpStream directly
                    // (it doesn't implement Clone). try_clone() is how you
                    // get a second handle to the same socket. This is an
                    // important TCP/OS concept - the OS socket itself is
                    // reference counted at the kernel level.
                    stream.try_clone().expect("Failed to clone stream"),
                ));

                // Register this client in the shared list.
                // Lock → push → drop the lock immediately.
                // LEARNING NOTE: Hold locks for the shortest time possible.
                // Holding a lock while doing I/O is a classic mistake that
                // causes all other threads to stall waiting.
                {
                    let mut list = clients.lock().unwrap();
                    list.push(Arc::clone(&client_handle));
                }

                // Clone the Arc (not the data) so the new thread gets its
                // own referene to the shared client list.
                let clients_clone = Arc::clone(&clients);

                thread::spawn(move || {
                    handle_client(stream, peer.to_string(), clients_clone, client_handle);
                });
            }
            Err(e) => {
                eprintln!("[server] Accept error: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(
    stream: TcpStream,
    peer: String,
    clients: ClientList,
    // We need our own handle so we can remove ourselves from the list on exit.
    my_handle: ClientHandle,
) {
    // BufReader wraps the stream so we can read line-by-line efficiently.
    // Without buffering, we'd read one byte at a time - very slow.
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(msg) => {
                let outgoing = format!("[{}]:{}\n", peer, msg);
                print!("{}", outgoing);
                // Broadcast to all connected clients.
                // LEARNING NOTE: We lock the list to iterate it, but we
                // release the individual client lock after each write.
                // If we held the list lock AND tried to lock each client,
                // and another thread was doing the same in the other order,
                // we'd have a DEADLOCK. Always acquire locks in a consistent
                // order to avoid this.
                broadcast(&clients, &outgoing, &my_handle);
            }
            Err(e) => {
                eprintln!("[server] Error reading from {}: {}", peer, e);
                break;
            }
        }
    }
    // Client disconnected. Remove them from the shared list.
    // LEARNING NOTE: If you don't do this, the list grows forever with dead
    // handles, and every broadcast will try (and fail) to write to them.
    // This is a classic "stale handle" / resource leak bug in chat servers.
    cleanup(&clients, &my_handle, &peer);
}

fn broadcast(clients: &ClientList, message: &str, sender: &ClientHandle) {
    // Lock the list for the duration of the iteration.
    let list = clients.lock().unwrap();

    for client in list.iter() {
        // Skip sending the message back to the sender.
        // Arc::ptr_eq checks if two Arcs point to the exact same allocation.
        if Arc::ptr_eq(client, sender) {
            continue;
        }
        // Lock this specific client's stream and write to it.
        // if the write fails (client disconnected), we just skip them.
        // They'll be cleaned up when their own read loop exists
        match client.lock().unwrap().write_all(message.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("[server] Error writing to client: {}", e);
            }
        }
    }
    // list lock is released here automatically (Drop trait).
}

fn cleanup(clients: &ClientList, my_handle: &ClientHandle, peer: &str) {
    println!("[server] {} disconnected. Cleaning up.", peer);

    let mut list = clients.lock().unwrap();

    // retain() keeps only elements for which the closure returns true.
    // We remove ourself by pointer comparison.
    list.retain(|c| !Arc::ptr_eq(c, my_handle));
    println!("[server] Active connections: {}", list.len());
}
