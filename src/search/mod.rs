pub mod search_response;

use crate::constants::SEARCH_ENDPOINT;
use crate::enums::{Lang, TransportType};
use crate::errors::YaRaspError;
use crate::handle_response::handle_response;
use crate::YaRaspClient;
use chrono::{Local, NaiveDate};
use chrono_tz::{Tz, UTC};
use search_response::SearchResponse;

pub struct SearchRequestBuilder {
    ya_rasp_client: YaRaspClient,
    from: String,
    to: String,
    lang: Lang,
    date: NaiveDate,
    transport_types: TransportType,
    system: String,
    show_systems: String,
    offset: u32,
    limit: u32,
    add_days_mask: bool,
    result_timezone: Tz,
    transfers: bool,
}

impl SearchRequestBuilder {
    pub fn new(ya_rasp_client: YaRaspClient, from: String, to: String) -> Self {
        Self {
            ya_rasp_client,
            from,
            to,
            lang: Lang::default(),
            date: Local::now().naive_local().date(),
            transport_types: TransportType::default(),
            system: String::from("yandex"),
            show_systems: String::from("yandex"),
            offset: 0,
            limit: 100,
            add_days_mask: false,
            result_timezone: UTC,
            transfers: false,
        }
    }

    /// Отправить запрос
    pub async fn send(&self) -> Result<SearchResponse, YaRaspError> {
        let response = self
            .ya_rasp_client
            .reqwest_client
            .get(SEARCH_ENDPOINT.clone())
            .query(&[
                ("format", "json"),
                ("apikey", &self.ya_rasp_client.api_key),
                ("from", &self.from),
                ("to", &self.to),
                ("lang", &self.lang.to_string()),
                ("date", &self.date.to_string()),
                ("transport_types", &self.transport_types.to_string()),
                ("system", &self.system),
                ("show_systems", &self.show_systems),
                ("offset", &self.offset.to_string()),
                ("limit", &self.limit.to_string()),
                ("add_days_mask", &self.add_days_mask.to_string()),
                ("result_timezone", &self.result_timezone.to_string()),
                ("transfers", &self.transfers.to_string()),
            ])
            .send()
            .await?;

        handle_response::<SearchResponse>(response).await
    }

    pub fn lang(&mut self, lang: Lang) -> &mut Self {
        self.lang = lang;
        self
    }

    pub fn date(&mut self, date: NaiveDate) -> &mut Self {
        self.date = date;
        self
    }

    pub fn date_str(&mut self, date_str: &str) -> &mut Self {
        self.date = date_str.parse().unwrap();
        self
    }

    pub fn transport_types(&mut self, transport_types: TransportType) -> &mut Self {
        self.transport_types = transport_types;
        self
    }
    pub fn system(&mut self, system: &str) -> &mut Self {
        self.system = String::from(system);
        self
    }
    pub fn show_systems(&mut self, show_systems: &str) -> &mut Self {
        self.show_systems = String::from(show_systems);
        self
    }
    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.offset = offset;
        self
    }
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = limit;
        self
    }

    pub fn add_days_mask(&mut self, add_days_mask: bool) -> &mut Self {
        self.add_days_mask = add_days_mask;
        self
    }
    pub fn result_timezone(&mut self, result_timezone: Tz) -> &mut Self {
        self.result_timezone = result_timezone;
        self
    }
    pub fn transfers(&mut self, transfers: bool) -> &mut Self {
        self.transfers = transfers;
        self
    }
}
