use std::process::CommandArgs;

#[derive(clap::Parser)]
struct Cli {
    /// The pattern to look for
    #[clap{subcommand}]
    command: Option<Commands>,
}

#[derive(clap::Parser)]
enum Commands {
    #[clap(version  = "1.0", author = "Don", about = "Marco polo game   ")]
    Play {
        /// The name to play with
        #[clap(short, long)]
        name: String,
    },
}
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
           let result = hello_marco::marco_polo(&name); 
              println!("{}", result);
        }
        None => {
            println!("No command provided.");
        }
    }
}
