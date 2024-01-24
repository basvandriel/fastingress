#!/bin/bash
set -o pipefail

DEFAULT_PROXY_PORT=8001
response=$(curl --write-out '%{http_code}' --silent --output /dev/null localhost:$DEFAULT_PROXY_PORT/api)

if [ $response != 200 ]
then
    echo "Proxy doesn't seem to be running on port $DEFAULT_PROXY_PORT. Aborting."
    exit 1;
fi
echo "OK"