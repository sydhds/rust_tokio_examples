# Introduction

# Self-signed certificate

Setup:

* cd certs
* ./self_signed.sh

Notes:

* server_cert.pem
* server_key.pem: private key (encrypted, [pkcs8](https://en.wikipedia.org/wiki/PKCS_8))
* server_key_decrypted: private key (not encrypted, pkcs8)

Run server:

`
cargo run --example server_self_signed 127.0.0.1:6161 certs/self_signed/server_cert.pem certs/self_signed/server_key_decrypted.pem
`

Run client:

`
openssl s_client -showcerts -connect 127.0.0.1:6161
`

Run client 2:

`
cargo run --example client_self_signed 127.0.0.1:6161
`

# Certificate signed with local CA

TODO

# Certificated signed with local CA + client auth ([mTLS]())

