# Tests

| Ebene                 | Ort                        | Was wird getestet                   | Beispiel                        |
| --------------------- | -------------------------- | ----------------------------------- | ------------------------------- |
| **Unit Tests**        | `dina-core/src/.../mod.rs` | Reine Funktionen, Model-Validierung | `recipe.parse_ingredients()`    |
| **Integration Tests** | `dina-core/tests/`         | DB-Queries, Service-Methoden        | `shopping_list.auto_generate()` |
| **E2E Tests**         | `dina-web/tests/`          | HTTP-Endpunkte                      | `POST /recipes` → 201 Created   |
