# DINA CLI 🚀🏡

**DINA** (Daily Improvement Navigation Assistant) is a modern, lightweight CLI tool designed to streamline your household planning and management. It helps you organize recipes, meal plans, manage household articles, and generate shopping lists — all in one simple, easy-to-use interface. ✨📝

---

## Features 🌟

- **Recipe Management 🍽️:** Add, view, and organize your recipes.
- **Household Articles (ERP) 🧴🛒:** Track your current stock of household items, set minimum quantities, and automatically update shopping lists.
- **Shopping List 📝🛍️:** Create, view, and manage shopping items.
- **Meal Planning 📅🍲:** Plan weekly meals (future feature).
- **Persistence 💾:** Save your data in TOML files or a database (e.g., SQLite).
- **Sync 🔄:** Optional synchronization with remote Git repositories for backup and sharing.

---

## Getting Started 🚀

### Installation 🛠️

Build and run from source:

```bash
git clone https://github.com/reddy50n/dina-cli.git
cd dina-cli
cargo install --path .
```

Alternatively, download pre-built binaries from the releases page.

---

## Usage 🧭

### Basic Commands 📝

```bash
dina list [recipes|articles|shopping]
dina show [recipe|article] <name>
dina add [recipe|article|shopping] <name> [additional options]
dina edit [recipe|article|shopping] <name>
dina delete [recipe|article|meal] <name>
```

### Managing Articles (Household ERP) 🧴🧹

```bash
# List all articles
dina list articles

# Add a new article
dina add article "Soap" 3 --unit "1 kg" --category "drugstore" --min-quantity 2

# Update existing article
dina edit article "Soap" --quantity 5

# Remove an article
dina delete article "Soap"
```

### Managing Shopping List 🛒📝

```bash
# Add items to shopping list
dina add shopping "Salt" 1 --unit "kg" --store "Aldi"
dina add shopping "Flour" 2 --unit "kg" --store "Lidl"

# View shopping list
dina list shopping

# Remove an item from shopping list
dina delete shopping 2
```

### Recipes 🍳

```bash
# List all recipes
dina list recipes

# Show a specific recipe
dina show recipe "Pancakes"

# Add a new recipe
dina add recipe "Pancakes" --ingredients "Flour, Eggs, Milk" --instructions "Mix and cook"

# Edit an existing recipe
dina edit recipe "Pancakes"

# Delete a recipe
dina delete recipe "Pancakes"
```

### Synchronization with Remote Git Repository 🔗

Ensure `git` is installed and SSH keys are configured.

```bash
# Initialize synchronization
dina sync init git@github.com:yourusername/dina-database.git

# Push local changes
dina sync push -m "Updated recipes"

# Pull latest data
dina sync pull

# Check status
dina sync status
```

---

## Data Persistence 💾

- **TOML Files:** Stored at `.local/share/dina-cli/dina-db.toml` (Linux)
- **Database:** Optionally, use SQLite for more complex storage

---

## Extensible Architecture 🧱

The project is structured for easy extension:

```
src/
├── main.rs
├── models.rs        # Data structures (Recipe, Article, MealPlan, ShoppingItem, Database)
├── cli.rs           # Command-line interface definitions
├── storage.rs       # Persistence logic
├── git_sync.rs      # Git synchronization
└── handlers/        # Command handlers (list.rs, add.rs, delete.rs, show.rs, edit.rs, sync.rs)
```

---

## Inspiration & Resources 📚

- Rust CLI best practices and examples: [rust-command-line-utilities](https://github.com/sts10/rust-command-line-utilities)
- CLI color management recommendations: [rust-cli-recommendations](https://rust-cli-recommendations.sunshowers.io/managing-colors-in-rust.html)

---

## Testing 🧪

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run specific test
cargo test test_handle_add_recipe

# With code coverage
cargo tarpaulin --lib --tests
```

---

## License 📄

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Contact ✉️

For issues, feature requests, or contributions, please open an issue on GitHub.

---

*Happy household planning with DINA!* 😊
