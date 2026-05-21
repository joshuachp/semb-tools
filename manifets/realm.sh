#!/usr/bin/env bash

set -exEuo pipefail

# Trap -e errors
trap 'echo "Exit status $? at line $LINENO from: $BASH_COMMAND"' ERR

if [[ ! -f .astarte/test_private.pem ]]; then
    mkdir -p .astarte

    pushd .astarte

    astartectl utils gen-keypair test

    popd
fi

kubectl --kubeconfig ./.kubeconfig.yaml -n \
    astarte get secret astarte-housekeeping-private-key \
    -o 'jsonpath={.data.private-key}' |
    base64 -d >./.astarte/housekeeping-private.pem

astartectl housekeeping realms create test \
    --non-interactive \
    --ignore-ssl-errors \
    -u https://api.astarte.localhost \
    --housekeeping-key .astarte/housekeeping-private.pem \
    --realm-private-key .astarte/test_private.pem
