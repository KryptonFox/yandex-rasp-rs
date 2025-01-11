pub mod stations_list_response;

use crate::enums::Lang;
use crate::YaRaspClient;
use stations_list_response::StationsListResponse;

pub struct StationsListRequestBuilder {
    ya_rasp_client: YaRaspClient,
    lang: Lang,
}
impl StationsListRequestBuilder {
    pub fn new(ya_rasp_client: YaRaspClient) -> StationsListRequestBuilder {
        Self {
            ya_rasp_client,
            lang: Lang::default(),
        }
    }

    pub async fn send(&self) -> Result<StationsListResponse, Box<dyn std::error::Error>> {
        let response = self
            .ya_rasp_client
            .reqwest_client
            .get("https://api.rasp.yandex.net/v3.0/stations_list/")
            .query(&[
                ("format", "json"),
                ("apikey", &self.ya_rasp_client.api_key),
                ("lang", &self.lang.to_string()),
            ])
            .send()
            .await?;
        if response.status().is_success() {
            Ok(response.json::<StationsListResponse>().await?)
        } else {
            Err(format!(
                "Unable to fetch stations list. Status: {}",
                response.status()
            )
            .into())
        }
    }

    pub fn lang(&mut self, lang: Lang) -> &mut Self {
        self.lang = lang;
        self
    }
}
