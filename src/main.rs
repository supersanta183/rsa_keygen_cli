use clap::{Command, Parser};
use rsa_keygen::{ generate_seedphrase_and_keypair, pkcs8_pem_from_priv_key, pkcs8_pem_from_pub_key, keypair_from_seedphrase, generate_seedphrase };
use rsa::{pkcs8::der::zeroize::Zeroize, RsaPrivateKey, RsaPublicKey};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct AppArgs {
    /// generate seedphrase and associated keypair
    #[clap(short, long)]
    new: bool,

    /// generate keypair from seedphrase
    #[clap(short, long)]
    fromseed: Option<String>,

    /// generate new seedphrase
    #[clap(short, long)]
    newseed: bool
}

fn main() {
    let args = AppArgs::parse();

    if args.new {
        println!("Generating new seedphrase and keypair \n");
        display_seedphrase_and_keypair();
    } else if let Some(seedphrase) = args.fromseed {
        println!("Generating keypair from seedphrase {}", seedphrase);
        derive_keypair_from_seedphrase(&seedphrase);
    } else if args.newseed {
        println!("Generating seedphrase");
        gen_seedphrase();
    }
}

fn display_seedphrase_and_keypair() {
    let (seedphrase, keypair) = match generate_seedphrase_and_keypair() {
        Ok((seedphrase, keypair)) => (seedphrase, keypair),
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    println!("Seedphrase: {} \n", seedphrase);
    print_keypair(keypair);
}

fn derive_keypair_from_seedphrase(seedphrase: &String) {
    println!("hej");
    println!("{}", seedphrase);
    let keypair = keypair_from_seedphrase(seedphrase).expect("failed to generate keypair from seedphrase");
    print_keypair(keypair);
}

fn gen_seedphrase() {
    let seedphrase = generate_seedphrase();
    println!("seedphrase: {}", seedphrase);
}

fn print_keypair(keypair: (RsaPrivateKey, RsaPublicKey)) {
    let (priv_key, pub_key) = keypair;
    let pem_pub_key = pkcs8_pem_from_pub_key(&pub_key).unwrap();
    let mut pem_priv_key = pkcs8_pem_from_priv_key(&priv_key).unwrap();
    println!("{}", &pem_priv_key.as_str());
    pem_priv_key.zeroize();
    println!("{}", pem_pub_key);
}
