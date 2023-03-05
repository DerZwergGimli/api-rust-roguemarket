#!/usr/bin/bash
RUST_LOG=info cargo run \
 --package substream-mongodb \
 --bin substream-mongodb  \
 mongodb://root:root@localhost:27017 \
 https://mainnet.sol.streamingfast.io:443 \
  ../substream-sa/substreams.spkg \
  db_sa_trades \
  trades \
  159433267 \
  0 \
  1000