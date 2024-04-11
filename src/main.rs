use clap::{Command, Parser};
use rsa_keygen::{ generate_seedphrase_and_keypair, pkcs8_pem_from_priv_key, pkcs8_pem_from_pub_key };

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct AppArgs {
    /// generate seedphrase and associated keypair
    #[clap(short, long)]
    new: bool,
}

fn main() {
    let args = AppArgs::parse();

    if args.new {
        println!("Generating new seedphrase and keypair \n");
        display_seedphrase_and_keypair();
    }
    

}

fn display_seedphrase_and_keypair() {
    let (seedphrase, (priv_key, pub_key)) = match generate_seedphrase_and_keypair() {
        Ok((seedphrase, keypair)) => (seedphrase, keypair),
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    println!("Seedphrase: {} \n", seedphrase);
    let pem_pub_key = pkcs8_pem_from_pub_key(&pub_key).unwrap();
    let pem_priv_key = pkcs8_pem_from_priv_key(&priv_key).unwrap();
    println!("{}", pem_priv_key.as_str());
    println!("{}", pem_pub_key);
}