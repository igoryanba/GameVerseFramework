#!/bin/sh
set -eu
cert=/var/lib/gameverse/identity/server-cert.der
key=/var/lib/gameverse/identity/server-key.der
if [ ! -s "$cert" ] || [ ! -s "$key" ]; then
  rm -f "$cert" "$key"
  gameverse-presence-server-m2 --cert "$cert" --key "$key" --init-identity
fi
exec gameverse-presence-server-m2 --bind 0.0.0.0:30122 --cert "$cert" --key "$key"
