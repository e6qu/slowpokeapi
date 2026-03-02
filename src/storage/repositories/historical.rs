use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, SqlitePool};

use crate::storage::{Repository, Result};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HistoricalRate {
    pub id: Option<i64>,
    pub date: String,
    pub base_currency: String,
    pub target_currency: String,
    pub rate: f64,
    pub source: String,
    pub created_at: i64,
}

pub struct HistoricalRepository {
    pool: SqlitePool,
}

impl HistoricalRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn find_by_date_and_pair(
        &self,
        date: &str,
        base: &str,
        target: &str,
    ) -> Result<Option<HistoricalRate>> {
        let rate = sqlx::query_as::<_, HistoricalRate>(
            "SELECT id, date, base_currency, target_currency, rate, source, created_at
             FROM historical_rates 
             WHERE date = ? AND base_currency = ? AND target_currency = ?",
        )
        .bind(date)
        .bind(base)
        .bind(target)
        .fetch_optional(&self.pool)
        .await?;

        Ok(rate)
    }

    pub async fn find_by_date(&self, date: &str) -> Result<Vec<HistoricalRate>> {
        let rates = sqlx::query_as::<_, HistoricalRate>(
            "SELECT id, date, base_currency, target_currency, rate, source, created_at
             FROM historical_rates 
             WHERE date = ?",
        )
        .bind(date)
        .fetch_all(&self.pool)
        .await?;

        Ok(rates)
    }

    pub async fn find_by_date_range(
        &self,
        start_date: &str,
        end_date: &str,
        base: &str,
    ) -> Result<Vec<HistoricalRate>> {
        let rates = sqlx::query_as::<_, HistoricalRate>(
            "SELECT id, date, base_currency, target_currency, rate, source, created_at
             FROM historical_rates 
             WHERE date BETWEEN ? AND ? AND base_currency = ?
             ORDER BY date",
        )
        .bind(start_date)
        .bind(end_date)
        .bind(base)
        .fetch_all(&self.pool)
        .await?;

        Ok(rates)
    }
}

#[async_trait]
impl Repository for HistoricalRepository {
    type Entity = HistoricalRate;

    async fn create(&self, rate: Self::Entity) -> Result<Self::Entity> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            "INSERT INTO historical_rates (date, base_currency, target_currency, rate, source, created_at)
             VALUES (?, ?, ?, ?, ?, ?)
             ON CONFLICT(date, base_currency, target_currency) DO UPDATE SET
             rate = excluded.rate,
             source = excluded.source
             RETURNING id"
        )
        .bind(&rate.date)
        .bind(&rate.base_currency)
        .bind(&rate.target_currency)
        .bind(rate.rate)
        .bind(&rate.source)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        let id: i64 = result.get(0);
        Ok(HistoricalRate {
            id: Some(id),
            created_at: now,
            ..rate
        })
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Self::Entity>> {
        let rate = sqlx::query_as::<_, HistoricalRate>(
            "SELECT id, date, base_currency, target_currency, rate, source, created_at
             FROM historical_rates WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(rate)
    }

    async fn update(&self, rate: Self::Entity) -> Result<Self::Entity> {
        let id = rate.id.ok_or(sqlx::Error::RowNotFound)?;

        sqlx::query(
            "UPDATE historical_rates 
             SET date = ?, base_currency = ?, target_currency = ?, rate = ?, source = ?
             WHERE id = ?",
        )
        .bind(&rate.date)
        .bind(&rate.base_currency)
        .bind(&rate.target_currency)
        .bind(rate.rate)
        .bind(&rate.source)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(rate)
    }

    async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM historical_rates WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
