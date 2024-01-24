#!/bin/bash
# This file will assume there is a running Kubernetes cluster

DEFAULT_PROXY_PORT=8001
CLUSTER_PROXY_IP="localhost:$DEFAULT_PROXY_PORT"

echo "No input proxy port found. Defaulting to $DEFAULT_PROXY_PORT"
echo "Testing cluster operations on $CLUSTER_PROXY_IP"
echo ""
echo "Note: This proxy should only be used to test the result of the Ingress results. The API should ONLY be used for test verification"
echo ""
# During tests, multiple configurations should be published in a future state
# During tests, the configurations should then be verified
# In this, the ingress controller should be configured.
# After this, but before the test, the proxy  should be verified.

# In the test cleanup, the kubernetes cluster should tear down, removing all pods

kubectl cluster-info

kubectl proxy &