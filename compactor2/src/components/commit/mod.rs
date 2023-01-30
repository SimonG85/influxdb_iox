use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use async_trait::async_trait;
use data_types::{ParquetFileId, ParquetFileParams};

pub mod catalog;
pub mod logging;
pub mod metrics;
pub mod mock;

/// Ensures that the file change (i.e. deletion and creation) are committed to the catalog.
#[async_trait]
pub trait Commit: Debug + Display + Send + Sync {
    /// Commmit deletions and creations in a single transaction.
    ///
    /// Returns the IDs for the created files.
    ///
    /// This method retries. During the retries, no intermediate states (i.e. some files deleted, some created) will be
    /// visible. Commits are always all-or-nothing.
    async fn commit(
        &self,
        delete: &[ParquetFileId],
        create: &[ParquetFileParams],
    ) -> Vec<ParquetFileId>;
}

#[async_trait]
impl<T> Commit for Arc<T>
where
    T: Commit + ?Sized,
{
    async fn commit(
        &self,
        delete: &[ParquetFileId],
        create: &[ParquetFileParams],
    ) -> Vec<ParquetFileId> {
        self.as_ref().commit(delete, create).await
    }
}