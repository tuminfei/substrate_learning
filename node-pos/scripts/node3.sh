#!/usr/bin/env bash
./target/release/node-stencil \
  --base-path /tmp/node3 \
  --chain ./terry-staging-raw.json \
  --bootnodes  /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWLDMRcDKskBjac7ZMDCPcV1JRouvXXtfehW5Jm1nZZpBQ \
  --port 30335 \
  --ws-port 9947 \
  --rpc-port 9935 \
  --validator \
  --name node3
    
