#!/usr/bin/env bash

set -exEuo pipefail

# Trap -e errors
trap 'echo "Exit status $? at line $LINENO from: $BASH_COMMAND"' ERR

kustomize build --enable-helm ./cert-manager | kubectl apply --kubeconfig ./.kubeconfig.yaml --filename - --wait=true

kubectl rollout status deployment -n cert-manager cert-manager

kustomize build --enable-helm ./operators/ | kubectl apply --kubeconfig ./.kubeconfig.yaml --filename - --server-side --force-conflicts --wait=true

kubectl rollout status deployment -n scylla-operator webhook-server
kubectl rollout status deployment -n astarte-operator astarte-operator-controller-manager

kustomize build --enable-helm ./astarte/ | kubectl apply --kubeconfig ./.kubeconfig.yaml --filename - --wait=true

echo "Creating credential secrets for RabbitMQ..."
RABBITMQ_PASSWORD=$(
    kubectl get secret rabbitmq-default-user \
        --kubeconfig ./.kubeconfig.yaml \
        -n rabbitmq-system \
        -o jsonpath='{.data.password}' |
        base64 --decode
)
RABBITMQ_USER=$(
    kubectl get secret rabbitmq-default-user \
        --kubeconfig ./.kubeconfig.yaml \
        -n rabbitmq-system \
        -o jsonpath='{.data.username}' |
        base64 --decode
)

echo "apiVersion: v1
kind: Secret
type: Opaque
metadata:
  name: rabbitmq-connection-secret
  namespace: astarte
stringData:
  username: '$RABBITMQ_USER'
  password: '$RABBITMQ_PASSWORD'
" | kubectl apply --filename - --kubeconfig ./.kubeconfig.yaml

# Wait for it to be ready (cluster status must be green), up to 15 minutes
while true; do
    if kubectl wait -n astarte astarte astarte --for 'jsonpath={.status.health}=green'; then
        echo "Astarte cluster reported green status"
        break
    else
        echo "Astarte cluster not ready yet, waiting..."
        echo "Current status: "
        kubectl get pods -n astarte
    fi
done
