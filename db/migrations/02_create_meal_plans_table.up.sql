CREATE TABLE meal_plans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    week_start DATE NOT NULL,       -- Montag der Woche
    day_of_week INTEGER NOT NULL CHECK(day_of_week BETWEEN 0 AND 6),
    recipe_id INTEGER REFERENCES recipes(id) ON DELETE SET NULL,
    notes TEXT,
    UNIQUE(user_id, week_start, day_of_week)
);
