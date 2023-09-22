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
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_sa_trades --start-block 218775170 --stop-block 218775172

.PHONY: map_sa_trades_long
map_sa_trades_long: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_sa_trades --start-block 194492072 --stop-block 206661222


.PHONY: map_sa_trades_double
map_sa_trades_double: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_sa_trades --start-block 151993612 --stop-block 151993613


.PHONY: map_sa_trades_single
map_sa_trades_single: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_sa_trades --start-block 181695702 --stop-block 181695704



.PHONY: map_sa_trades_test
map_sa_trades_test: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_sa_trades --start-block 185686863 --stop-block 0


.PHONY: store_sa_trades
store_sa_trades: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml store_sa_trades

.PHONY: sa_trades_db_out
sa_trades_db_out: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml sa_trades_db_out


.PHONY: db_sa_trades
db_sa_trades: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml db_sa_trades --start-block 151993612 --stop-block 0

.PHONY: db_out
db_out: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml db_out


.PHONY: package
package: build
	substreams pack -o substreams.spkg substreams.yaml && cp ./substreams.spkg ../substream-psql/substreams.spkg

.PHONY: db_eth
db_eth: package
	./start_eth.sh

.PHONY: db_sol
db_sol: package
	./start_sol.sh


.PHONY: json_eth
json_eth: package
	./start_eth_json.sh

