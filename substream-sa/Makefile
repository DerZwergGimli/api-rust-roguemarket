ENDPOINT ?= mainnet.eth.streamingfast.io:443
GRAPH_CONFIG ?= ../graph-node-dev/config/graphman.toml
STOP_BLOCK ?= +10

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONE: build
build:
	cargo build --target wasm32-unknown-unknown --release


.PHONY: map_sa_trades
map_sa_trades: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_sa_trades --start-block 152317109 --stop-block 171345490


.PHONY: store_sa_trades
store_sa_trades: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml store_sa_trades

.PHONY: sa_trades_db_out
sa_trades_db_out: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml sa_trades_db_out


.PHONY: db_sa_trades
db_sa_trades: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml db_sa_trades --start-block 142317109 --stop-block 0

.PHONY: db_out
db_out: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml db_out


.PHONY: package
package: build
	substreams pack -o substreams.spkg substreams.yaml

.PHONY: db_eth
db_eth: package
	./start_eth.sh

.PHONY: db_sol
db_sol: package
	./start_sol.sh


.PHONY: json_eth
json_eth: package
	./start_eth_json.sh

