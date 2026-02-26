use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// ---------------------------------------------------------------------------
// LEARNING NOTE: The client has a classic concurrency problem.
//
// We need to do TWO things simultaneously:
//   1. Read lines from stdin and send them to the server
//   2. Read messages from the server and print them
//
// These can't happen sequentially (if we wait for user input, we miss server
// messages; if we wait for server messages, we can't type).
//
// Solution: split into two threads.
//   - Main thread: reads stdin → writes to server
//   - Spawned thread: reads server → prints to stdout
//
// TcpStream::try_clone() lets both threads share the same socket without
// needing Arc<Mutex<>> because we split read/write responsibilities cleanly.
// One thread only reads, one only writes. No contention = no Mutex needed.
// This is the "split ownership" pattern and is much cleaner than sharing.
// ---------------------------------------------------------------------------

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080";
    let stream = TcpStream::connect(addr)?;

    // Set a read timeout of 15 seconds
    const TIME_OUT_SECS: u64 = 15;
    stream.set_read_timeout(Some(Duration::from_secs(TIME_OUT_SECS)))?;
    println!("[client] Connected to {}", addr);
    println!("[client] Type a message and press Enter to send. Ctrl+C to quit.");

    // Clone the stream. reader_stream is for the background thread,
    // writer_stream stays in main.
    let reader_stream = stream.try_clone()?;

    let (tx, rx) = mpsc::channel();

    // Spawn a background thread to handle incoming messages from the server.
    // 'move' transfers ownership of reader_stream into the closure.
    let receiver = thread::spawn(move || {
        let reader = BufReader::new(reader_stream);
        for line in reader.lines() {
            match line {
                Ok(msg) => {
                    // \r clears the current input line before printing,
                    // so the server message doesn't appear mid-sentence.
                    print!("\r{}\n> ", msg);
                    io::stdout().flush().ok();
                }
                Err(e) => {
                    match e.kind() {
                        io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut => {
                            println!("\n[client] Read timeout (no data for 15s)");
                            // Could retry or keep waiting. For now, just ignore and keep waiting.
                            break;
                        }
                        _ => {
                            eprintln!("\n[client] Read error: {}", e);
                            println!("\n[client] Server disconnected.");
                            let _ = tx.send(()); // Signal main thread to exit.
                            break;
                        }
                    }
                }
            }
        }
    });

    // Main thread handles sending.
    let stdin = io::stdin();
    let mut writer = stream;

    print!("> ");
    io::stdout().flush()?;

    for line in stdin.lock().lines() {
        // Check if the receiver thread has signaled to exit.
        if rx.try_recv().is_ok() {
            println!("\n[client] Exiting due to server disconnect.");
            break;
        }

        match line {
            Ok(msg) => {
                if msg.trim().is_empty() {
                    print!("> ");
                    io::stdout().flush()?;
                    continue;
                }

                // Append \n because the server reads line-by-line.
                // LEARNING NOTE: TCP is a byte stream, not a message stream.
                // You must define your own message framing. Here we use
                // newlines. Real protocols use length-prefixed frames or
                // delimiters like HTTP's \r\n\r\n.
                let to_send = format!("{}\n", msg);
                if let Err(e) = writer.write_all(to_send.as_bytes()) {
                    eprintln!("[client] Send error: {}", e);
                    break;
                }

                print!("> ");
                io::stdout().flush()?;
            }
            Err(e) => {
                eprintln!("[client] Stdin error: {}", e);
                break;
            }
        }
    }

    println!("[client] Disconnecting...");
    let _ = receiver.join();

    Ok(())
}

// ---------------------------------------------------------------------------
// PHASE 1 CLIENT EXERCISES:
//
// 1. Did this one.
//    GRACEFUL SHUTDOWN: If the server disconnects, the receiver thread exits
//    but the main thread is still blocking on stdin. How do you signal main
//    to also exit? Look into std::sync::atomic::AtomicBool as a shared flag,
//    or a channel (std::sync::mpsc).
//
// 2. USERNAME: Send your username as the first line right after connecting,
//    before entering the read loop. The server will use it to label messages.
//
// 3. RECONNECT: If the connection drops, try to reconnect with exponential
//    backoff (wait 1s, then 2s, then 4s, etc.). Use std::thread::sleep.
//
// 4. TIMEOUT: Use TcpStream::set_read_timeout() to add a timeout.
//    What error do you get when it fires? How do you distinguish a timeout
//    from a real disconnect?
// ---------------------------------------------------------------------------
