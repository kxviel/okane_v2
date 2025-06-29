CREATE TABLE IF NOT EXISTS category (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category_name TEXT NOT NULL UNIQUE,
    category_desc TEXT,
    badge_color TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO category (id, category_name, category_desc, badge_color, created_at, updated_at) VALUES (1, "uncategorized", "default category", "green-200", CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_title TEXT NOT NULL UNIQUE,
    transaction_desc TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    transaction_category_id INTEGER,
    FOREIGN KEY (transaction_category_id) REFERENCES category(id)
);