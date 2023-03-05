ENDPOINT ?= mainnet.eth.streamingfast.io:443
GRAPH_CONFIG ?= ../graph-node-dev/config/graphman.toml
STOP_BLOCK ?= +10

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONE: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run_db_out_eth
run_db_out_eth: build
	 substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml db_out_eth --start-block 10 --stop-block 100

.PHONY: run_jsonl_out_eth
run_jsonl_out_eth: build
	 substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml jsonl_out_eth --start-block 10 --stop-block 100

.PHONY: run_db_out_sol
run_db_out_sol: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml db_out_sol --start-block 179432154 --stop-block 179432254

.PHONY: run_db_out_sol
run_db_out_sol: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml db_out_sol --start-block 179432154 --stop-block 179432254

.PHONY: map_sa_trades
map_sa_trades: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_sa_trades --start-block 159433267 --stop-block 0

.PHONY: run_db_out_sa
run_db_out_sa: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml db_sa_trades --start-block 159433267 --stop-block 0


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

