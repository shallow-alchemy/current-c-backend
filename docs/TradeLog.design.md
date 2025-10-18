# FX Trading Tracker - Backend Design For Trade Logs

## Database Schema

### trades table
```sql
CREATE TABLE trades (
    trade_id SERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    account_balance DECIMAL(15,2) NOT NULL,
    trade_type VARCHAR(4) NOT NULL CHECK (type IN ('BUY', 'SELL')),
    price DECIMAL(15,6) NOT NULL,
    quantity DECIMAL(15,2) NOT NULL,
    pip_price DECIMAL(10,6) NOT NULL,
    spread DECIMAL(10,6) NOT NULL,
    trade_time TIMESTAMPTZ NOT NULL,
    notes TEXT
);
```

### positions table
```sql
CREATE TABLE positions (
    position_id SERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    balance DECIMAL(15,2) NOT NULL,
    is_open BOOLEAN NOT NULL DEFAULT true,
    position_type VARCHAR(5) NOT NULL CHECK (type IN ('LONG', 'SHORT')),
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
```

### position_trades table (enhanced many-to-many)
```sql
CREATE TABLE position_trades (
    position_id INTEGER REFERENCES positions(position_id),
    trade_id INTEGER REFERENCES trades(trade_id),
    quantity_allocated DECIMAL(15,2) NOT NULL, -- how much of the trade applies to this position
    trade_action VARCHAR(10) NOT NULL CHECK (trade_action IN ('OPEN', 'ADD', 'REDUCE', 'CLOSE')),
    PRIMARY KEY (position_id, trade_id)
);
```

## REST API Endpoints

### Trades
```
POST   /api/trades              # Create new trade
GET    /api/trades              # List all trades (with pagination/filters)
GET    /api/trades/{id}         # Get specific trade
GET    /api/trades/{id}/positions # Get all positions affected by this trade
PUT    /api/trades/{id}         # Update trade
DELETE /api/trades/{id}         # Delete trade
```

### Positions
```
GET    /api/positions           # List all positions
GET    /api/positions/open      # List only open positions
GET    /api/positions/closed    # List only closed positions
GET    /api/positions/{id}      # Get specific position
GET    /api/positions/{id}/trades # Get all trades for a position with allocation details
```

### Analytics
```
GET    /api/symbols             # List all traded symbols
GET    /api/pnl/summary         # Overall P&L summary
GET    /api/pnl/by-symbol       # P&L breakdown by symbol
GET    /api/stats/win-rate      # Win/loss statistics
GET    /api/allocations         # Full audit trail of trade allocations
```

## Core Business Logic

### Position Calculation Scenarios

1. **Simple position opening**: Trade creates new position (`trade_action='OPEN'`)
2. **Adding to position**: Same direction trade increases position size (`trade_action='ADD'`)
3. **Partial close**: Opposite direction trade reduces position (`trade_action='REDUCE'`)
4. **Full close**: Position closed completely (`trade_action='CLOSE'`)
5. **Position flip**: Single trade closes existing position AND opens new opposite position

### Trade Processing Algorithm

When processing a trade:
- **No existing position**: Create new position with full trade quantity
- **Same direction as existing**: Add to position, update weighted average entry price
- **Opposite direction, partial**: Reduce position quantity, calculate realized P&L
- **Opposite direction, exact match**: Close position completely
- **Opposite direction, exceeds position**: Close existing position, open new position in opposite direction with remaining quantity

### Position Calculations

- **Entry Price**: Weighted average when adding to position
- **PIP Diff**: `close_price - entry_price` (LONG), `entry_price - close_price` (SHORT)
- **Profit/Loss**: `pip_diff * quantity * pip_price`
- **Win/Loss**: "WIN" if `profit_loss > 0`, "LOSS" if `profit_loss <= 0`

### Trade-Position Mapping

- Trade `type=BUY` → Position `type=LONG`
- Trade `type=SELL` → Position `type=SHORT`

## Key Benefits

1. **Complete audit trail**: Every trade shows exactly how it affected positions via `quantity_allocated` and `trade_action`
2. **Position flip support**: Single trade can close one position and open another through multiple allocation records
3. **Flexible P&L tracking**: Calculate realized P&L for each portion of a trade based on allocations
4. **Data integrity**: Foreign key constraints ensure consistency between trades and positions
5. **Query flexibility**: Easy to get trades for a position or positions for a trade with allocation context
6. **Granular analysis**: Track exactly how each trade contributes to position building and P&L realization

## Next Immediate Steps

1. **Create the three migration files** for all tables with proper constraints and indexes
2. **Implement the Rust models** with proper serialization and the enhanced `PositionTrade` struct
3. **Build the trade processing service** with position flip logic and allocation creation
4. **Create the trade creation endpoint** that handles all scenarios (open, add, reduce, close, flip)
5. **Add position query endpoints** that show allocation details and trade history
6. **Implement P&L calculation service** for real-time position valuation and realized gains