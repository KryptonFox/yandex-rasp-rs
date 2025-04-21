use once_cell::sync::Lazy;

pub static API_ENDPOINT: &str = "https://api.rasp.yandex.net/v3.0";

pub static SCHEDULE_ENDPOINT: Lazy<String> = Lazy::new(|| format!("{API_ENDPOINT}/schedule"));
pub static SEARCH_ENDPOINT: Lazy<String> = Lazy::new(|| format!("{API_ENDPOINT}/search"));
pub static  STATIONS_LIST_ENDPOINT: Lazy<String> =
    Lazy::new(|| format!("{API_ENDPOINT}/stations_list"));
pub static  THREAD_ENDPOINT: Lazy<String> = Lazy::new(|| format!("{API_ENDPOINT}/thread"));
