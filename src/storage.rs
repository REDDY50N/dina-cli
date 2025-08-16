use crate::models::{Recipe, Article, MealPlan, ShoppingItem};

// Trait für einheitliche Schnittstelle
trait Storage {
    fn save(&self);
    fn load() -> Self;
}

// Konkrete Implementierungen
pub struct JsonStorage;
pub struct SqliteStorage;

impl Storage for JsonStorage {
    // JSON-Implementierung
}

impl Storage for SqliteStorage {
    // SQLite-Implementierung
}

// Öffentliche Schnittstelle
pub fn list_recipes() {
    let storage = get_storage_impl();
    // ...
}

// Helper zur Auswahl der Speichermethode
fn get_storage_impl() -> impl Storage {
    // Konfiguration aus Datei oder Umgebungsvariable
    SqliteStorage // oder JsonStorage
}