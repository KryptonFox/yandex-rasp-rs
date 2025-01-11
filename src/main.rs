use chrono::NaiveTime;
use chrono_tz::Europe;
use yandex_rasp_rs::enums::TransportType;
use yandex_rasp_rs::YaRaspClient;

#[tokio::main]
async fn main() {
    let client = YaRaspClient::new("92993f33-a5cd-4fa6-9f78-8346c460d9a6");
    dbg!(NaiveTime::parse_from_str("15:10:23", "%H:%M:%S").unwrap());

    let from = "s9603402";
    let to = "s9602675";
    match client
        .search(&from, &to)
        .transport_types(TransportType::Suburban)
        .result_timezone(Europe::Moscow)
        .send()
        .await
    {
        Ok(res) => {
            dbg!(res);
        }
        Err(e) => eprintln!("error{:?}", e),
    }

    match client.schedule(from).send().await {
        Ok(res) => {
            dbg!(res);
        }
        Err(e) => eprintln!("error{:?}", e),
    }

    match client.stations_list().send().await {
        Ok(res) => {
            dbg!(res);
        }
        Err(e) => eprintln!("error{:?}", e),
    }
}
