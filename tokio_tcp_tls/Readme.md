# Introduction

# Self-signed certificate

## Setup

* cd certs
* ./self_signed.sh

Notes:

* server_cert.pem
* server_key.pem: private key (encrypted, [pkcs8](https://en.wikipedia.org/wiki/PKCS_8))
* server_key_decrypted: private key (not encrypted, pkcs8)

## Run server

`
cargo run --example server_self_signed 127.0.0.1:6161 certs/self_signed/server_cert.pem certs/self_signed/server_key_decrypted.pem
`

## Run client

`
openssl s_client -showcerts -connect 127.0.0.1:6161
`

## Run client 2:

`
cargo run --example client_self_signed 127.0.0.1:6161
`

# Certificate signed with local CA

## Setup

* cd certs
* ./ca_signed.sh mydomain.com

## Run server

* cargo run --example server_ca_signed 127.0.0.1:6161 certs/ca_signed/mydomain.com.crt certs/ca_signed/mydomain.com.key

## Run client

`
openssl s_client -showcerts -connect 127.0.0.1:6161
`

## Run client 2:

`
cargo run --example client_ca_signed -- 127.0.0.1:6161 certs/ca_signed/root_ca.pem mydomain.com
`

# Certificated signed with local CA + client auth ([mTLS](https://en.wikipedia.org/wiki/Mutual_authentication#mTLS))

## Setup

* cd certs
* ./ca_signed_client_auth.sh mydomain2.org

## Run server

* cargo run 127.0.0.1:6161 certs/ca_signed_client_auth/mydomain2.org.crt certs/ca_signed_client_auth/mydomain2.org.key

## Run client

`
cargo run --example client_ca_signed_client_auth -- 127.0.0.1:6161 certs/ca_signed_client_auth/root_ca.pem mydomain2.org certs/ca_signed_client_auth/client1.crt certs/ca_signed_client_auth/client1.key
`

Note that the following client will be refused:

`
cargo run --example client_ca_signed -- 127.0.0.1:6161 certs/ca_signed_client_auth/root_ca.pem mydomain2.org
`

with: CerticateRequired alert
