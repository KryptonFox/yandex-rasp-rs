use crate::enums::TransportType;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct ScheduleResponse {
    pub date: String,
    pub pagination: Pagination,
    pub station: Station,
    pub schedule: Vec<Schedule>,
    pub interval_schedule: Vec<IntervalSchedule>,
    pub schedule_direction: ScheduleDirection,
    pub directions: Vec<Direction>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Schedule {
    pub thread: Thread,
    pub is_fuzzy: bool,
    pub days: String,
    pub stops: String,
    pub terminal: Value,
    pub platform: String,
    pub except_days: Option<String>,
    pub arrival: DateTime<Utc>,
    pub departure: DateTime<Utc>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Pagination {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Default, Debug, Deserialize)]
pub struct Station {
    pub code: String,
    pub title: String,
    pub station_type: String,
    pub popular_title: String,
    pub short_title: String,
    pub transport_type: TransportType,
    #[serde(rename = "type")]
    pub station_kind: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Thread {
    pub uid: String,
    pub title: String,
    pub number: String,
    pub short_title: String,
    pub carrier: Carrier,
    pub transport_type: String,
    pub vehicle: Option<String>,
    pub transport_subtype: TransportSubtype,
    pub express_type: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Carrier {
    pub code: i64,
    pub codes: Codes,
    pub title: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Codes {
    pub icao: Option<String>,
    pub sirena: Option<String>,
    pub iata: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct TransportSubtype {
    pub color: Option<String>,
    pub code: Option<String>,
    pub title: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct IntervalSchedule {
    pub except_days: Value,
    pub thread: Thread,
    pub is_fuzzy: bool,
    pub days: String,
    pub stops: String,
    pub terminal: Value,
    pub platform: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Interval {
    pub density: String,
    pub end_time: String,
    pub begin_time: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct ScheduleDirection {
    pub code: String,
    pub title: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Direction {
    pub code: String,
    pub title: String,
}
