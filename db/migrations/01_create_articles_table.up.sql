CREATE TABLE articles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    emoji TEXT NOT NULL DEFAULT '📦',
    quantity REAL NOT NULL DEFAULT 0,
    unit TEXT NOT NULL,
    category TEXT NOT NULL DEFAULT 'other',  -- kitchen, drugstore, fridge, pantry
    storage_location TEXT NOT NULL DEFAULT 'pantry', -- fridge, pantry, freezer
    min_quantity REAL,
    warning_active BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, name)
);
