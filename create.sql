-- Create the database
CREATE DATABASE TransactionsFeed;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255),
    city_name VARCHAR(255),
);

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    brand_name VARCHAR(255),
    full_product_name VARCHAR(255)
);

CREATE TABLE transaction_types (
    id SERIAL PRIMARY KEY,
    type_name VARCHAR(255)
);

CREATE TABLE prices (
    id SERIAL PRIMARY KEY,
    amount_including_tax DECIMAL(10, 2),
    amount_excluding_tax DECIMAL(10, 2),
    currency VARCHAR(3)
);

CREATE TABLE vote_types (
    id SERIAL PRIMARY KEY,
    type_name VARCHAR(255)
);

CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    transaction_timestamp TIMESTAMP,
    transaction_type_id INT REFERENCES transaction_types(id),
    product_id INT REFERENCES products(id),
    user_id INT REFERENCES users(id),
    price_id INT REFERENCES prices(id),
    rating INT,
    search_string VARCHAR(255),
    image_url TEXT,
    quote TEXT,
    url TEXT
);

CREATE TABLE votes (
    id SERIAL PRIMARY KEY,
    vote_type_id INT REFERENCES vote_types(id),
    user_id INT REFERENCES users(id),
    transaction_id INT REFERENCES transactions(id)
);
