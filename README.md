# rsa_keygen_cli

## how to use:
To install the cli tool, run: 
```Rust
Cargo install rsa_keygen_cli
```

current options:
--help: displays a list of options
--new: generates a 12 word seedphrase and associating rsa keypair in pksc8 pem format.
--fromseed: generates rsa keypair from already known seedphrase
--newseed: generates a new seedphrase without rsa keypair

Example usage:
´´´
rsakeygen --new
´´´

generating keypair from seed:
```
rsakeygen --fromseed "brother above used lounge afraid cash half universe spatial casual notable hope"
```

