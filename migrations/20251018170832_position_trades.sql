-- Add migration script here
CREATE TABLE position_trades (
  position_id INTEGER REFERENCES positions(position_id),
  trade_id INTEGER REFERENCES trades(trade_id),
  quantity_allocated DECIMAL(15,2) NOT NULL,
  trade_action VARCHAR(10) NOT NULL CHECK(trade_action IN ('OPEN', 'ADD', 'REDUCE', 'CLOSE')),
  PRIMARY KEY (position_id, trade_id)  
);