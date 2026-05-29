use sqlx::{SqlitePool, Row};
use crate::models::Article;
use anyhow::{Result, bail};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

// =============================================
// ARTICLES - CREATE
// =============================================

impl Database {
    pub async fn add_article(
        &self,
        user_id: i64,
        name: &str,
        emoji: &str,
        quantity: f64,
        unit: &str,
        category: &str,
        storage_location: &str,
        min_quantity: Option<f64>,
    ) -> Result<Article> {
        let id = sqlx::query(
            r#"
            INSERT INTO articles (
                user_id, name, emoji, quantity, unit, category,
                storage_location, min_quantity, warning_active
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
            .bind(user_id)
            .bind(name)
            .bind(emoji)
            .bind(quantity)
            .bind(unit)
            .bind(category)
            .bind(storage_location)
            .bind(min_quantity)
            .bind(false)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

        self.get_article_by_id(id).await
    }
}

// =============================================
// ARTICLES - READ
// =============================================

impl Database {
    pub async fn list_articles(&self, user_id: i64) -> Result<Vec<Article>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, user_id, name, emoji, quantity, unit, category,
                storage_location, min_quantity, warning_active
            FROM articles
            WHERE user_id = ?
            ORDER BY name
            "#
        )
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let articles = rows.into_iter().map(row_to_article).collect();
        Ok(articles)
    }

    pub async fn get_article_by_id(&self, id: i64) -> Result<Article> {
        let row = sqlx::query(
            r#"
            SELECT
                id, user_id, name, emoji, quantity, unit, category,
                storage_location, min_quantity, warning_active
            FROM articles
            WHERE id = ?
            "#
        )
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row_to_article(row))
    }

    pub async fn get_article_by_name(
        &self,
        user_id: i64,
        name: &str,
    ) -> Result<Article> {
        let row = sqlx::query(
            r#"
            SELECT
                id, user_id, name, emoji, quantity, unit, category,
                storage_location, min_quantity, warning_active
            FROM articles
            WHERE user_id = ? AND name = ?
            "#
        )
            .bind(user_id)
            .bind(name)
            .fetch_one(&self.pool)
            .await?;

        Ok(row_to_article(row))
    }

    pub async fn list_articles_by_storage(
        &self,
        user_id: i64,
        storage_location: &str,
    ) -> Result<Vec<Article>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, user_id, name, emoji, quantity, unit, category,
                storage_location, min_quantity, warning_active
            FROM articles
            WHERE user_id = ? AND storage_location = ?
            ORDER BY name
            "#
        )
            .bind(user_id)
            .bind(storage_location)
            .fetch_all(&self.pool)
            .await?;

        let articles = rows.into_iter().map(row_to_article).collect();
        Ok(articles)
    }

    pub async fn list_articles_with_warnings(&self, user_id: i64) -> Result<Vec<Article>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, user_id, name, emoji, quantity, unit, category,
                storage_location, min_quantity, warning_active
            FROM articles
            WHERE user_id = ? AND warning_active = TRUE
            ORDER BY name
            "#
        )
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let articles = rows.into_iter().map(row_to_article).collect();
        Ok(articles)
    }
}

// =============================================
// ARTICLES - UPDATE
// =============================================

impl Database {
    pub async fn update_article_quantity(
        &self,
        user_id: i64,
        name: &str,
        new_quantity: f64,
    ) -> Result<()> {
        let result = sqlx::query(
            "UPDATE articles SET quantity = ? WHERE user_id = ? AND name = ?"
        )
            .bind(new_quantity)
            .bind(user_id)
            .bind(name)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            bail!("Article '{}' not found", name);
        }

        self.refresh_warnings(user_id, name).await?;

        Ok(())
    }

    pub async fn update_article(
        &self,
        user_id: i64,
        name: &str,
        emoji: Option<&str>,
        quantity: Option<f64>,
        unit: Option<&str>,
        category: Option<&str>,
        storage_location: Option<&str>,
        min_quantity: Option<f64>,
    ) -> Result<()> {
        let article = self.get_article_by_name(user_id, name).await?;

        let new_emoji = emoji.unwrap_or(&article.emoji);
        let new_quantity = quantity.unwrap_or(article.quantity);
        let new_unit = unit.unwrap_or(&article.unit);
        let new_category = category.unwrap_or(&article.category);
        let new_storage = storage_location.unwrap_or(&article.storage_location);
        let new_min = min_quantity.or(article.min_quantity);

        let result = sqlx::query(
            r#"
            UPDATE articles
            SET emoji = ?, quantity = ?, unit = ?, category = ?,
                storage_location = ?, min_quantity = ?
            WHERE user_id = ? AND name = ?
            "#
        )
            .bind(new_emoji)
            .bind(new_quantity)
            .bind(new_unit)
            .bind(new_category)
            .bind(new_storage)
            .bind(new_min)
            .bind(user_id)
            .bind(name)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            bail!("Article '{}' not found", name);
        }

        self.refresh_warnings(user_id, name).await?;

        Ok(())
    }
}

// =============================================
// ARTICLES - DELETE
// =============================================

impl Database {
    pub async fn delete_article(&self, user_id: i64, name: &str) -> Result<()> {
        let result = sqlx::query(
            "DELETE FROM articles WHERE user_id = ? AND name = ?"
        )
            .bind(user_id)
            .bind(name)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            bail!("Article '{}' not found", name);
        }

        Ok(())
    }
}

// =============================================
// ARTICLES - WARNING LOGIC
// =============================================

impl Database {
    pub async fn check_all_warnings(&self, user_id: i64) -> Result<Vec<Article>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, user_id, name, emoji, quantity, unit, category,
                storage_location, min_quantity, warning_active
            FROM articles
            WHERE user_id = ?
              AND min_quantity IS NOT NULL
              AND quantity < min_quantity
            ORDER BY name
            "#
        )
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let articles = rows.into_iter().map(row_to_article).collect();
        Ok(articles)
    }

    pub async fn refresh_warnings(&self, user_id: i64, name: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE articles
            SET warning_active = (
                min_quantity IS NOT NULL AND quantity < min_quantity
            )
            WHERE user_id = ? AND name = ?
            "#
        )
            .bind(user_id)
            .bind(name)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn refresh_all_warnings(&self, user_id: i64) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE articles
            SET warning_active = (
                min_quantity IS NOT NULL AND quantity < min_quantity
            )
            WHERE user_id = ?
            "#
        )
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

// =============================================
// HELPER: Row -> Article
// =============================================

fn row_to_article(row: sqlx::sqlite::SqliteRow) -> Article {
    Article {
        id: row.get("id"),
        user_id: row.get("user_id"),
        name: row.get("name"),
        emoji: row.get("emoji"),
        quantity: row.get("quantity"),
        unit: row.get("unit"),
        category: row.get("category"),
        storage_location: row.get("storage_location"),
        min_quantity: row.get("min_quantity"),
        warning_active: row.get("warning_active"),
    }
}