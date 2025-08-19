# README

DINA = Daily Improvement Navigation Assistant

Daily helper for organizing recipes, weekly meal plan, articles in stock and a shopping list.



### Persitence

Save data in toml file
Save data in database e.g. SQLite


## IDE

Using ZED IDE - s. Keybindings: `~/.config/zed/keymap.json`

## Inspiration

### Rust CLI apps
- List of CLI apps: https://github.com/sts10/rust-command-line-utilities
- Rust CLI recommandations: https://rust-cli-recommendations.sunshowers.io/managing-colors-in-rust.html

## Extensible Structure

```
src/
├── main.rs         # CLI-Parsing und Routing
├── cli.rs          # Clap-Definitionen
├── models.rs       # Datenstrukturen
├── storage/
│   ├── mod.rs      # Persistenz-Schnittstelle
│   ├── json.rs     # JSON-Implementierung
│   └── sqlite.rs   # SQLite-Implementierung
├── commands/
│   ├── mod.rs      # Befehlslogik
│   ├── list.rs     # list-Implementierung
│   └── show.rs     # show-Implementierung
└── editor.rs       # Editor-Integration
```



## Commands and sub-commands
List all entries in a list of recipes, food stock arictles, shopping list
- dina list [recipes, articles, shopping]

Show a whole recipe or single arcticle
- dina show  [recipe, article]

Add an element to the list / database:
- dina add [recipe, article, meal, shopping]

Open file in a (terminal) editor like vim to edit
- dina edit [recipe, article, meal, shopping]

Delete article from database
- dina delete [recipe, article, meal]

### Examples
- dina list recipes         => List all recipe names, all articles with amount, all meal names, all shopping list items
- dina show reicpe <name>   => Print the recipe on the console (formated, human readable)
- dina show article <name>  => show article status (name, ammount, price, brand, seller, ...)


## Usage

### Articles (Household ERP)

```sh
dina list articles
dina add article "Soap" 3 --unit "1 kg" --category "drugstore" --min-quantity 2
dina add article "Flour 1050" 2

```

### Shopping list

```sh
dina add shopping "Salz" 1
dina add shopping "Mehl" 2 --unit "kg" --store "Aldi Süd"
dina add shopping "Milch" 1 --unit "Liter" --store "Rewe"
dina list shopping
dina delete shopping 2
```

### Sync database with remote git repo

Needs git installed on your system and SSH access to your remote git.

- Init repo: `dina sync init git@github.com:yourusername/dina-database.git`
- Commit: `dina sync push -m "Initial database"`
- Pull: `dina sync pull`
- Push: `dina sync push -m "Added new recipes`
- Status: `dina sync status`

Linux: `.local/share/dina-cli/dina-db.toml`
