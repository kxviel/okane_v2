CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category_name TEXT NOT NULL UNIQUE,
    category_desc TEXT,
    badge_color TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO categories (id, category_name, category_desc, badge_color, created_at, updated_at) 
VALUES (1, "uncategorized", "default category", "gray-300", CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

CREATE TABLE accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_name TEXT NOT NULL,
    account_type TEXT NOT NULL,
    balance DECIMAL(15, 2) DEFAULT 0.00,
    currency TEXT DEFAULT 'EUR',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO accounts (id, account_name, account_type, balance, currency, is_active, created_at, updated_at) 
VALUES (1, 'Cash', 'cash', 0.00, 'EUR', TRUE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_category_id INTEGER NOT NULL,
    account_id INTEGER NOT NULL,
    amount DECIMAL(15, 2) NOT NULL,
    transaction_name TEXT NOT NULL,
    transaction_desc TEXT,
    transaction_date DATE NOT NULL,
    transaction_type TEXT NOT NULL DEFAULT 'expense',
    merchant_name TEXT,                          
    transaction_location TEXT,
    currency TEXT DEFAULT 'EUR',  
    payment_method TEXT,   
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (account_id) REFERENCES accounts(id),
    FOREIGN KEY (transaction_category_id) REFERENCES categories(id)
);