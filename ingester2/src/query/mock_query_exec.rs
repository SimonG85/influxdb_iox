use async_trait::async_trait;
use data_types::{NamespaceId, TableId};
use parking_lot::Mutex;
use trace::span::Span;

use super::{response::Response, QueryError, QueryExec};

#[derive(Debug, Default)]
pub(crate) struct MockQueryExec {
    response: Mutex<Option<Result<Response, QueryError>>>,
}

impl MockQueryExec {
    pub(crate) fn with_result(self, r: Result<Response, QueryError>) -> Self {
        *self.response.lock() = Some(r);
        self
    }
}

#[async_trait]
impl QueryExec for MockQueryExec {
    async fn query_exec(
        &self,
        _namespace_id: NamespaceId,
        _table_id: TableId,
        _columns: Vec<String>,
        _span: Option<Span>,
    ) -> Result<Response, QueryError> {
        self.response
            .lock()
            .take()
            .unwrap_or(Err(QueryError::NamespaceNotFound(NamespaceId::new(42))))
    }
}