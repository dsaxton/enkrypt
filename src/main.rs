use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = "Minimal symmetric encryption utility")]
struct Args {
    /// Encryption key
    #[clap(short, long)]
    key: String,
    /// File name
    #[clap(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    println!("Key: {}", args.key);
    println!("File: {}", args.file);

    match std::fs::read(args.file) {
        Ok(bytes) => {
            println!("{:?}", bytes);
        }
        Err(e) => {
            panic!("{}", e)
        }
    }
}
