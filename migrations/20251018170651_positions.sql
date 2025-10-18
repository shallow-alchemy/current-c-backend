-- Add migration script here
CREATE TABLE positions (
    position_id SERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    balance DECIMAL(15,2) NOT NULL,
    is_open BOOLEAN NOT NULL DEFAULT true,
    position_type VARCHAR(5) NOT NULL CHECK (position_type IN ('LONG', 'SHORT')),
    entry_price DECIMAL(15,6) NOT NULL,
    close_price DECIMAL(15,6),
    quantity DECIMAL(15,2) NOT NULL,
    pip_price DECIMAL(10,6) NOT NULL,
    pip_diff DECIMAL(10,6),
    profit_loss DECIMAL(15,2),
    win_loss VARCHAR(4) CHECK (win_loss IN ('WIN', 'LOSS')),
    open_time TIMESTAMPTZ NOT NULL,
    close_time TIMESTAMPTZ,
    notes TEXT
);