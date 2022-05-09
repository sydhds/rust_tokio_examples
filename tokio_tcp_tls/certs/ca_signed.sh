#!/bin/sh

if [ "$#" -ne 1 ]
then
  echo "Usage: Must supply a domain"
  exit 1
fi

DOMAIN=$1

cd ca_signed

# root certificate (CA)
openssl genrsa -out root_ca.key 2048
openssl req -x509 -new -nodes -key root_ca.key -sha256 -days 365 -out root_ca.pem

# server certificate
openssl genrsa -out $DOMAIN.key 2048
openssl req -new -key $DOMAIN.key -out $DOMAIN.csr

cat > $DOMAIN.ext << EOF
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names
[alt_names]
DNS.1 = $DOMAIN
EOF

openssl x509 -req -in $DOMAIN.csr -CA root_ca.pem -CAkey root_ca.key -CAcreateserial \
-out $DOMAIN.crt -days 365 -sha256 -extfile $DOMAIN.ext
