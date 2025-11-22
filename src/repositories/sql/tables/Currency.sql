CREATE TABLE Currency (
	id serial4 NOT NULL,
	symbol varchar(10) NOT NULL,
	"name" varchar(100) NOT NULL,
	kind varchar(20) NOT NULL,
	is_active bool DEFAULT true NOT NULL,
	"precision" int2 NOT NULL,
	coingecko_id varchar(15) NULL,
	CONSTRAINT currency_name_key UNIQUE (name),
	CONSTRAINT currency_pkey PRIMARY KEY (id),
	CONSTRAINT currency_precision_check CHECK ((("precision" >= 0) AND ("precision" <= 18))),
	CONSTRAINT currency_symbol_key UNIQUE (symbol)
);