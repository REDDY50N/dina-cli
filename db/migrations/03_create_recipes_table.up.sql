CREATE TABLE recipes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    emoji TEXT NOT NULL DEFAULT '🍽️',
    ingredients TEXT NOT NULL,      -- JSON: [{"name":"Mehl","amount":500,"unit":"g"}]
    instructions TEXT NOT NULL,
    prep_time INTEGER,              -- Minuten
    tags TEXT,                      -- JSON-Array: ["vegetarisch","schnell"]
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, name)
);
