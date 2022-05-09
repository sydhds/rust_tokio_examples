cd self_signed
openssl req -x509 -sha256 -newkey rsa:4096 -keyout server_key.pem -out server_cert.pem -days 365 --subj '/CN=127.0.0.1/'
openssl rsa -in server_key.pem -out server_key_decrypted.pem
