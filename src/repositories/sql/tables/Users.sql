/* VARCHAR(255) or VARCHAR are managed the same, also in terms of performance, but we try to prevent the possibility that someone submit a username of 10MB. */

CREATE TABLE users (
    id bpchar(36) NOT NULL, 
    username public.citext NOT NULL, 
    hashed_password varchar NOT NULL, 
    currency_id int4 NOT NULL,
    creation_date timestamptz NOT NULL, 
    "role" varchar(100) NOT NULL, 
      
    CONSTRAINT users_pkey PRIMARY KEY (id), 
    CONSTRAINT users_username_unique UNIQUE (username));

-- public.users foreign keys

ALTER TABLE public.users ADD CONSTRAINT users_currency_id_fkey FOREIGN KEY (currency_id) REFERENCES currency(id);