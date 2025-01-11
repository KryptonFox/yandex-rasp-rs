use crate::enums::TransportType;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct SearchResponse {
    pub pagination: Pagination,
    pub interval_segments: Vec<IntervalSegment>,
    pub segments: Vec<Segment>,
    pub search: Search,
}

#[derive(Default, Debug, Deserialize)]
pub struct Segment {
    pub arrival: DateTime<Utc>,
    pub from: RFrom,
    pub thread: Thread,
    pub departure_platform: String,
    pub departure: DateTime<Utc>,
    pub stops: String,
    pub departure_terminal: Option<String>,
    pub to: To,
    pub has_transfers: Option<bool>,
    pub tickets_info: Option<TicketsInfo>,
    pub duration: f64,
    pub arrival_terminal: Option<String>,
    pub start_date: NaiveDate,
    pub arrival_platform: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Pagination {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Default, Debug, Deserialize)]
pub struct IntervalSegment {
    pub from: RFrom,
    pub thread: Thread,
    pub departure_platform: String,
    pub stops: String,
    pub departure_terminal: Option<String>,
    pub to: To,
    pub has_transfers: bool,
    pub tickets_info: TicketsInfo,
    pub duration: f64,
    pub arrival_terminal: String,
    pub start_date: NaiveDate,
    pub arrival_platform: String,
}
#[derive(Default, Debug, Deserialize)]
pub struct Thread {
    pub uid: String,
    pub title: String,
    pub interval: Option<Interval>,
    pub number: String,
    pub short_title: String,
    pub thread_method_link: String,
    pub carrier: Carrier,
    pub transport_type: TransportType,
    pub vehicle: Option<String>,
    pub transport_subtype: TransportSubtype,
    pub express_type: Option<String>,
}
#[derive(Default, Debug, Deserialize)]
pub struct RFrom {
    pub code: String,
    pub title: String,
    pub popular_title: String,
    pub short_title: String,
    pub transport_type: TransportType,
    #[serde(rename = "type")]
    pub type_field: String,
    pub station_type: String,
    pub station_type_name: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct To {
    pub code: String,
    pub title: String,
    pub popular_title: String,
    pub short_title: String,
    pub transport_type: TransportType,
    #[serde(rename = "type")]
    pub type_field: String,
    pub station_type: String,
    pub station_type_name: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Interval {
    pub density: String,
    pub end_time: String,
    pub begin_time: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Carrier {
    pub code: i64,
    pub contacts: String,
    pub url: String,
    pub logo_svg: Option<String>,
    pub title: String,
    pub phone: String,
    pub codes: Codes,
    pub address: String,
    pub logo: String,
    pub email: Option<String>,
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
pub struct TicketsInfo {
    pub et_marker: bool,
    pub places: Vec<Place>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Place {
    pub currency: String,
    pub price: Price,
    pub name: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Price {
    pub cents: i64,
    pub whole: i64,
}

#[derive(Default, Debug, Deserialize)]
pub struct Search {
    pub date: Option<NaiveDate>,
    pub to: To,
    pub from: RFrom,
}
