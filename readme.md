# Rogue Market backend-stack

This repo contains the codebase to create a fast and reliable data aggregation and storage solution for the RogueMarket
with optimizations for TradingView-Charts.

Its fetching history data using Substreams and writing it into a PostgreSQL-Database.

## Docker Images

### API

- [derzwerggimli/api-roguemarket-worker](derzwerggimli/api-roguemarket-worker)

```dotenv
RUST_LOG: info
MONGOURL: 'mongodb+srv://<user>:<password>@<host>'
```

### Substream-Worker

You may spawn up to instances of the worker:

1. Keep up to recent changes
2. Secound will be used to SYNC and can have multiple threads to speed things up!

