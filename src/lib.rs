pub mod enums;
pub mod errors;
mod handle_response;
mod schedule;
mod search;
mod stations_list;
mod thread;
mod constants;

pub use crate::schedule::schedule_response::ScheduleResponse;
pub use crate::schedule::ScheduleRequestBuilder;
pub use crate::search::search_response::SearchResponse;
pub use crate::search::SearchRequestBuilder;
pub use crate::stations_list::stations_list_response::StationsListResponse;
pub use crate::stations_list::StationsListRequestBuilder;
use crate::thread::ThreadRequestBuilder;

/// Основной клиент для взаимодействия с API
#[derive(Clone)]
pub struct YaRaspClient {
    api_key: String,
    reqwest_client: reqwest::Client,
}

impl YaRaspClient {
    /// Создать новый экзепляр клиента, используя переданный API ключ
    pub fn new(api_key: &str) -> YaRaspClient {
        Self {
            api_key: String::from(api_key),
            reqwest_client: reqwest::Client::new(),
        }
    }

    /// Возвращает конструктор запроса на поиск расписания между станциями, код которых был передан
    pub fn search(&self, from: &str, to: &str) -> SearchRequestBuilder {
        SearchRequestBuilder::new(self.clone(), from.to_string(), to.to_string())
    }

    /// Возвращает конструктор запроса на поиск расписания для станции, чей код был передан
    pub fn schedule(&self, station: &str) -> ScheduleRequestBuilder {
        ScheduleRequestBuilder::new(self.clone(), station.to_string())
    }

    pub fn thread(&self, thread_uid: &str) -> ThreadRequestBuilder {
        ThreadRequestBuilder::new(self.clone(), thread_uid)
    }

    /// Возвращает конструктор запроса на получения списка всех станций из API.
    /// Можно использовать для поиска кода станции по её названию
    pub fn stations_list(&self) -> StationsListRequestBuilder {
        StationsListRequestBuilder::new(self.clone())
    }
}
