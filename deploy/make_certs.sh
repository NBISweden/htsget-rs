#!/bin/sh

set -e

# Install openssl if it isn't available already
if ! openssl 2>/dev/null
then
    apk add openssl
fi

# Set output directory
out_dir="${1:-/certs/}"

#
# Helper functions
#

info() {
    printf "\e[96m%s\e[0m\n" "$*" >&2
}

success() {
    printf "\e[32m%s\e[0m\n" "$*" >&2
}


# Check the output directory for a ca certificate
if [ ! -f "$out_dir/privkey.pem" ] || [ ! -f "$out_dir/fullchain.pem" ]
then
    info "Creating certificates"
    openssl req -x509 -newkey rsa:4096 \
        -keyout "$out_dir/privkey.pem" \
        -out "$out_dir/fullchain.pem" \
        -addext "subjectAltName = DNS:htsget" \
        -sha256 -days 365 -nodes -subj '/CN=localhost'
else
    success "Certificates already exists"
fi


