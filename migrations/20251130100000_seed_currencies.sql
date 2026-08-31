-- Seed initial currencies for the application
-- These are the currencies required by the application to function properly

INSERT INTO Currency (symbol, name, kind, is_active, precision, coingecko_id, is_major) VALUES
-- Fiat currencies
('USD', 'US Dollar', 'Fiat', true, 2, NULL, true),
('EUR', 'Euro', 'Fiat', true, 2, NULL, true),
('GBP', 'British Pound', 'Fiat', true, 2, NULL, true),
('JPY', 'Japanese Yen', 'Fiat', true, 0, NULL, true),
('AUD', 'Australian Dollar', 'Fiat', true, 2, NULL, true),
('CHF', 'Swiss Franc', 'Fiat', true, 2, NULL, false),
('CAD', 'Canadian Dollar', 'Fiat', true, 2, NULL, false),
('CNY', 'Chinese Yuan', 'Fiat', true, 2, NULL, false),

-- Crypto currencies
('BTC', 'Bitcoin', 'Crypto', true, 8, 'bitcoin', true),
('ETH', 'Ethereum', 'Crypto', true, 8, 'ethereum', true),
('SOL', 'Solana', 'Crypto', true, 8, 'solana', false),
('ADA', 'Cardano', 'Crypto', true, 8, 'cardano', false),
('DOT', 'Polkadot', 'Crypto', true, 8, 'polkadot', false),
('LINK', 'Chainlink', 'Crypto', true, 8, 'chainlink', false),

-- Stablecoins
('USDT', 'Tether', 'Stablecoin', true, 6, 'tether', false),
('USDC', 'USD Coin', 'Stablecoin', true, 6, 'usd-coin', false),
('DAI', 'Dai', 'Stablecoin', true, 18, 'dai', false)
ON CONFLICT (symbol) DO NOTHING;
