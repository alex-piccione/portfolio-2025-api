alter table Currency
Add is_major boolean NOT NULL DEFAULT false;

-- Update existing major currencies to is_major = true
update Currency
set is_major = false
where symbol in ('USD', 'EUR', 'JPY', 'GBP', 'AUD', 'BTC', 'ETH');


-- Remove default constraint if not needed
alter table Currency
alter column is_major drop default; 