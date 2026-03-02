use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, SqlitePool};

use crate::storage::{Repository, Result};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Rate {
    pub id: Option<i64>,
    pub base_currency: String,
    pub target_currency: String,
    pub rate: f64,
    pub timestamp: i64,
    pub source: String,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct RatesRepository {
    pool: SqlitePool,
}

impl RatesRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn find_by_pair(&self, base: &str, target: &str) -> Result<Option<Rate>> {
        let rate = sqlx::query_as::<_, Rate>(
            "SELECT id, base_currency, target_currency, rate, timestamp, source, created_at, updated_at 
             FROM rates 
             WHERE base_currency = ? AND target_currency = ? 
             ORDER BY timestamp DESC 
             LIMIT 1"
        )
        .bind(base)
        .bind(target)
        .fetch_optional(&self.pool)
        .await?;

        Ok(rate)
    }

    pub async fn find_all_by_base(&self, base: &str) -> Result<Vec<Rate>> {
        let rates = sqlx::query_as::<_, Rate>(
            "SELECT id, base_currency, target_currency, rate, timestamp, source, created_at, updated_at 
             FROM rates 
             WHERE base_currency = ? 
             ORDER BY timestamp DESC"
        )
        .bind(base)
        .fetch_all(&self.pool)
        .await?;

        Ok(rates)
    }
}

#[async_trait]
impl Repository for RatesRepository {
    type Entity = Rate;

    async fn create(&self, rate: Self::Entity) -> Result<Self::Entity> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            "INSERT INTO rates (base_currency, target_currency, rate, timestamp, source, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)
             RETURNING id"
        )
        .bind(&rate.base_currency)
        .bind(&rate.target_currency)
        .bind(rate.rate)
        .bind(rate.timestamp)
        .bind(&rate.source)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        let id: i64 = result.get(0);
        Ok(Rate {
            id: Some(id),
            ..rate
        })
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Self::Entity>> {
        let rate = sqlx::query_as::<_, Rate>(
            "SELECT id, base_currency, target_currency, rate, timestamp, source, created_at, updated_at 
             FROM rates WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(rate)
    }

    async fn update(&self, rate: Self::Entity) -> Result<Self::Entity> {
        let id = rate.id.ok_or(sqlx::Error::RowNotFound)?;
        let now = chrono::Utc::now().timestamp();

        sqlx::query(
            "UPDATE rates 
             SET base_currency = ?, target_currency = ?, rate = ?, timestamp = ?, source = ?, updated_at = ?
             WHERE id = ?"
        )
        .bind(&rate.base_currency)
        .bind(&rate.target_currency)
        .bind(rate.rate)
        .bind(rate.timestamp)
        .bind(&rate.source)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(Rate {
            updated_at: now,
            ..rate
        })
    }

    async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM rates WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
