CREATE TABLE IF NOT EXISTS category (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category_name TEXT NOT NULL UNIQUE,
    category_desc TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO category (id, category_name, category_desc, created_at) VALUES (1, "uncategorized", "default category", DATETIME("now"));

CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_title TEXT NOT NULL UNIQUE,
    transaction_desc TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    transaction_category_id INTEGER,
    FOREIGN KEY (transaction_category_id) REFERENCES category(id)
);