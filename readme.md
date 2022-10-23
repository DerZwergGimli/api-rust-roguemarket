# Rogue Market backend-stack

This repo contains the codebase to create a fast and reliable data aggregation and storage solution for the RogueMarket with optimizations for TradingView-Charts.

## Docker Images

### API
- [derzwerggimli/api-roguemarket-worker](derzwerggimli/api-roguemarket-worker)

```dotenv
RUST_LOG: info
MONGOURL: 'mongodb+srv://<user>:<password>@<host>'
```

### Workers
Worker do have 2 Modes:
1. `loop` for continuous aggregation of new data
2. `sync` for fetching history of older trades

This repo contains 2 diffrent implementations one in Rust and the other one in JavaScript/TypeScript
#### Rust
- [derzwerggimli/api-roguemarket-api](derzwerggimli/api-roguemarket-api)

```dotenv
RUST_LOG: info
MODE: loop
RPCCLIENT: 'https://api.mainnet-beta.solana.com'
MONGOURL: 'mongodb+srv://<user>:<password>@<host>'
LASTSIG:
```

#### JavaScript
- [derzwerggimli/api-roguemarket-worker-js](derzwerggimli/api-roguemarket-worker-js)

```dotenv
MODE: sync
SLEEP: 1000
RPC: https://api.mainnet-beta.solana.com
DB_CONN_STRING: "mongodb+srv://<user>:<password>@<host>"
DB_NAME: trades_GM
EXCHANGE_COLLECTION: processExchange
COUNTER_COLLECTION:  initializeOpenOrdersCounter
CREATE_COLLECTION: createExchange
CANCEL_COLLECTION: cancelExchange
```