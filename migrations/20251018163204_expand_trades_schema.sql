-- Add migration script here
ALTER TABLE trades
ADD account_balance DECIMAL(15,2) NOT NULL,
ADD trade_type VARCHAR(4) NOT NULL CHECK (trade_type in ('BUY', 'SELL')),
ADD price DECIMAL(15,6) NOT NULL,
ADD quantity DECIMAL(15,2) NOT NULL,
ADD pip_price DECIMAL(12,8) NOT NULL,
ADD spread DECIMAL(10,6) NOT NULL,
ADD trade_time TIMESTAMPTZ NOT NULL,
ADD notes TEXT;