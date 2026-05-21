CREATE TABLE shopping_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    quantity REAL,
    unit TEXT,
    store TEXT,
    category TEXT,
    bought BOOLEAN NOT NULL DEFAULT FALSE,
    auto_generated BOOLEAN NOT NULL DEFAULT FALSE, -- von Warnung erzeugt?
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
