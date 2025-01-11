# yandex-rasp-rs
⚠ Проект находится в разработке
### Библиотека для взаимодействия с API [Яндекс.Расписаний](https://rasp.yandex.ru/) на языке Rust

---

- Основана на Reqwest и Serde
- Асинхронная
- Простое взаимодействие
- Типизация возвращаемых данных
- Обработка ошибок
- Все методы API (в разработке)

### Пример кода

Для работы необходимо получить токен. [Инструкция](https://yandex.ru/dev/rasp/doc/ru/concepts/access)

```rust
use yandex_rasp_rs::enums::TransportTypes;
use yandex_rasp_rs::{YaRaspClient, SearchResponse, StationsListResponse};

#[tokio::main]
async fn main() {
    let client: YaRaspClient = YaRaspClient::new("[ВАШ токен API Яндекс.Расписаний]");

    // Получение списка всех станций из API
    let stations_list: StationsListResponse = client
        .stations_list()
        .send()
        .await
        .unwrap();

    // Поиск маршрута между двумя станциями по их коду
    let from = "s9603402";
    let to = "s9602675";
    let search: SearchResponse = client
        .search(&from, &to)
        .date("2025-01-08")
        .transport_types(TransportTypes::Suburban)
        .send()
        .await
        .unwrap();
}
```

### Доступные методы

- /search
- /schedule
- /stations_list