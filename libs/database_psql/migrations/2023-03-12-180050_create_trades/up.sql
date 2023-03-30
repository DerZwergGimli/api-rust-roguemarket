CREATE TABLE trades
(
    pk                     TEXT PRIMARY KEY NOT NULL,
    signature              TEXT             NOT NULL,
    symbol                 TEXT             NOT NULL,
    block                  int8             NOT NULL,
    timestamp              int8             NOT NULL,
    timestamp_ts           TIMESTAMP        NOT NULL,
    order_taker            TEXT             NOT NULL,
    order_initializer      TEXT             NOT NULL,
    currency_mint          TEXT             NOT NULL,
    asset_mint             TEXT             NOT NULL,
    asset_receiving_wallet TEXT             NOT NULL,
    asset_change           float8           NOT NULL,
    market_fee             float8           NOT NULL,
    total_cost             float8           NOT NULL,
    price                  float8           NOT NULL
)