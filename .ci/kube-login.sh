#!/bin/sh -e
echo "$KUBE_CRT" > /tmp/kube-ca.crt
kubectl config set-cluster k8s \
    --server $KUBE_URL \
    --certificate-authority=/tmp/kube-ca.crt \
    --embed-certs=true \

kubectl config set-credentials $KUBE_SERVICE_ACCOUNT \
    --token "$KUBE_TOKEN"

kubectl config set-context k8s \
    --cluster k8s \
    --user=$KUBE_SERVICE_ACCOUNT
kubectl config use-context k8s

kubectl -n $NAMESPACE get deployments
