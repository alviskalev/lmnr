use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

pub mod in_memory;
pub mod keys;
pub mod redis;

pub use in_memory::InMemoryCache;
pub use redis::RedisCache;

#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("{0}")]
    InternalError(#[from] anyhow::Error),
    #[error("{{0}}")]
    SerDeError(#[from] serde_json::Error),
}

#[enum_dispatch]
pub enum Cache {
    InMemory(InMemoryCache),
    Redis(RedisCache),
}

#[async_trait]
#[enum_dispatch(Cache)]
pub trait CacheTrait: Sync + Send {
    async fn get<T>(&self, key: &str) -> Result<Option<T>, CacheError>
    where
        T: for<'de> Deserialize<'de> + Send + Sync;
    async fn insert<T>(&self, key: &str, value: T) -> Result<(), CacheError>
    where
        T: Serialize + Send + Sync;
    async fn remove(&self, key: &str) -> Result<(), CacheError>;
}
