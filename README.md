# rsa_keygen_cli

## how to use:
To install the cli tool, run: 
```Rust
Cargo install rsa_keygen_cli
```

current options:
--help: displays a list of options
--new: generates a 12 word seedphrase and associating rsa keypair in pksc8 pem format.

Example usage:
´´´Rust
rsakeygen --new
´´´

