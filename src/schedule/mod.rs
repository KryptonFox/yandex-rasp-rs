pub mod schedule_response;

use crate::enums::{Lang, ScheduleEvent, TransportType};
use crate::errors::YaRaspError;
use crate::handle_response::handle_response;
use crate::YaRaspClient;
use chrono::{Local, NaiveDate};
use chrono_tz::Tz;
use chrono_tz::Tz::UTC;
use schedule_response::ScheduleResponse;
use crate::constants::SCHEDULE_ENDPOINT;

pub struct ScheduleRequestBuilder {
    ya_rasp_client: YaRaspClient,
    station: String,
    lang: Lang,
    date: NaiveDate,
    transport_types: TransportType,
    event: ScheduleEvent,
    system: String,
    show_systems: String,
    direction: String,
    result_timezone: Tz,
}

impl ScheduleRequestBuilder {
    pub fn new(ya_rasp_client: YaRaspClient, station: String) -> Self {
        Self {
            ya_rasp_client,
            station,
            lang: Lang::default(),
            date: Local::now().naive_local().date(),
            transport_types: TransportType::default(),
            system: String::from("yandex"),
            event: ScheduleEvent::default(),
            show_systems: String::from("yandex"),
            direction: String::from("на Оредеж"),
            result_timezone: UTC,
        }
    }

    /// Отправить запрос
    pub async fn send(&self) -> Result<ScheduleResponse, YaRaspError> {
        let response = self
            .ya_rasp_client
            .reqwest_client
            .get(SCHEDULE_ENDPOINT.clone())
            .query(&[
                ("format", "json"),
                ("apikey", &self.ya_rasp_client.api_key),
                ("station", &self.station),
                ("lang", &self.lang.to_string()),
                ("date", &self.date.to_string()),
                ("transport_types", &self.transport_types.to_string()),
                ("system", &self.system),
                ("event", &self.event.to_string()),
                ("show_systems", &self.show_systems),
                ("direction", &self.direction),
                ("result_timezone", &self.result_timezone.to_string()),
            ])
            .send()
            .await?;
        handle_response::<ScheduleResponse>(response).await
    }

    pub fn lang(&mut self, lang: Lang) -> &mut Self {
        self.lang = lang;
        self
    }

    pub fn date(&mut self, date: NaiveDate) -> &mut Self {
        self.date = date;
        self
    }

    pub fn transport_types(&mut self, transport_type: TransportType) -> &mut Self {
        self.transport_types = transport_type;
        self
    }

    pub fn system(&mut self, system: String) -> &mut Self {
        self.system = system;
        self
    }

    pub fn direction(&mut self, direction: String) -> &mut Self {
        self.direction = direction;
        self
    }

    pub fn show_systems(&mut self, show_systems: String) -> &mut Self {
        self.show_systems = show_systems;
        self
    }

    pub fn result_timezone(&mut self, result_timezone: Tz) -> &mut Self {
        self.result_timezone = result_timezone;
        self
    }
}
