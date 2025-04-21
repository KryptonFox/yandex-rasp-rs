use yandex_rasp_rs::YaRaspClient;

#[tokio::main]
async fn main() {
    let client = YaRaspClient::new("Токен");

    let thread = client.thread("UID ветки").send().await.unwrap();
    println!("{:#?}", thread);
}
