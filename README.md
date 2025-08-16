# README

DINA = Daily Improvement Navigation Assistant

Daily helper for organizing recipes, weekly meal plan, articles in stock and a shopping list.

## CLI Help

https://rust-cli.github.io/book/index.html


## Reuqirements

Ich möchte eine CLI Anwendung in Rust 2024 schreiben (dina-cli). Es soll ein Alltagshelfer sein, um folgende Dinge zu organisieren: Wöchentlicher Essenplan, Lebensmittelvorräte (Artikelname, Anzahl in der Vorratskammer), Rezepte.

Ich möchte eine CLI Anwendung in Rust 2024 schreiben (dina-cli). Es soll ein Alltagshelfer sein, um folgende Dinge zu organisieren: Wöchentlicher Essenplan, Lebensmittelvorräte (Artikelname, Anzahl in der Vorratskammer), Rezepte und den Einkaufszettel.

### Commands and sub-commands

- dina list [recipes, articles, meals, shopping] // Listet z.b. alle Rezeptnamen mit dina list recipes
- dina show  [recipe, article] // Zeigt z.b. das vollständige Rezept auf der Konsole mit dina show recipe oder den Lagerbestand des Artikels mit dina show article
- dina add [recipe, article, meal, shopping] // fügt ein element hinzu bspw. dina add recipe "recipe name" "the whole recipe " oder dina add article "Nicer Flower Soap" 3 - oder dina add meal monday "Chicken Tikka"
- dina edit [recipe, article, meal, shopping] // öffnet bspw. recipe mit dem Terminal Editor wie vim, nano
- dina delete [recipe, article, meal]

### Examples
- dina list recipes         => List all recipe names, all articles with amount, all meal names, all shopping list items
- dina show reicpe <name>   => Print the recipe on the console (formated, human readable)
- dina show article <name>  => show article status (name, ammount, price, brand, seller, ...)


### Persitence

Save data in database e.g. SQLite


## IDE

Using ZED IDE - s. Keybindings: `~/.config/zed/keymap.json`


## Extensible Structure

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
