use crate::core::SemanticRecord;
use crate::semantic::DataProvider;
use iced::Task;

pub struct StubDB;

impl StubDB {
    pub fn new() -> Self {
        Self
    }

    pub fn save(&self, _record: SemanticRecord) -> Task<std::result::Result<(), String>> {
        Task::none()
    }

    pub fn search(
        &self,
        _query: &str,
        _limit: usize,
    ) -> Task<std::result::Result<Vec<SemanticRecord>, String>> {
        Task::none()
    }

    pub fn get_all(&self) -> Vec<SemanticRecord> {
        Vec::new()
    }
}

impl DataProvider for StubDB {
    fn save(&self, _record: SemanticRecord) -> Task<std::result::Result<(), String>> {
        Task::none()
    }

    fn find(&self, _query: String) -> Task<std::result::Result<Vec<SemanticRecord>, String>> {
        Task::none()
    }

    fn delete(&self, _id: String) -> Task<std::result::Result<(), String>> {
        Task::none()
    }

    fn async_find(
        &self,
        _query: String,
    ) -> iced::futures::future::BoxFuture<'static, std::result::Result<Vec<SemanticRecord>, String>>
    {
        use iced::futures::FutureExt;
        async { Ok(Vec::new()) }.boxed()
    }

    fn async_save(
        &self,
        _record: SemanticRecord,
    ) -> iced::futures::future::BoxFuture<'static, std::result::Result<(), String>> {
        use iced::futures::FutureExt;
        async { Ok(()) }.boxed()
    }

    fn async_find_semantic(
        &self,
        _vector: Vec<f32>,
        _limit: usize,
    ) -> iced::futures::future::BoxFuture<'static, std::result::Result<Vec<SemanticRecord>, String>> {
        use iced::futures::FutureExt;
        async { Ok(Vec::new()) }.boxed()
    }
}
