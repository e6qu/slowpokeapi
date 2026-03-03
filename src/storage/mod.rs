pub mod api_keys;
pub mod repositories;
pub mod sqlite;

pub use api_keys::ApiKeyStore;
pub use sqlite::SqlitePool;

use async_trait::async_trait;

pub type Result<T> = std::result::Result<T, sqlx::Error>;

#[async_trait]
pub trait Repository: Send + Sync {
    type Entity;

    async fn create(&self, entity: Self::Entity) -> Result<Self::Entity>;
    async fn find_by_id(&self, id: i64) -> Result<Option<Self::Entity>>;
    async fn update(&self, entity: Self::Entity) -> Result<Self::Entity>;
    async fn delete(&self, id: i64) -> Result<()>;
}
