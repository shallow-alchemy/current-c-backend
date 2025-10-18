-- Add migration script here
CREATE TABLE trades (
    trade_id SERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL
);