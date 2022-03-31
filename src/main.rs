use std::io::Write;
use clap::Parser;
use sha2::{Sha512, Digest};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = "Minimal XOR encryption utility")]
struct Args {
    /// Encryption key
    #[clap(short, long)]
    key: String,
    /// Input file name
    #[clap(short, long)]
    input: String,
    /// Output file name
    #[clap(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let mut hasher = Sha512::new();
    hasher.update(args.key.as_bytes());
    let hashed_key: Vec<u8> = hasher.finalize().to_vec();
    let file_bytes = match std::fs::read(args.input) {
        Ok(bytes) => bytes,
        Err(e) => {
            panic!("{}", e);
        } 
    };

    let path = std::path::Path::new(&args.output);
    let mut file = match std::fs::File::create(path) {
        Ok(f) => f,
        Err(e) => {
            panic!("{}", e);
        }
    };

    for (idx, file_byte) in file_bytes.iter().enumerate() {
        let xor_byte = file_byte ^ hashed_key[idx % hashed_key.len()];
        file.write_all(&[xor_byte]).expect("Unable to write byte");
    }
}
