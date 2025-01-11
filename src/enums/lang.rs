#[derive(Debug, Default)]
pub enum Lang {
    #[default]
    RU,
    UA,
}

impl Lang {
    pub fn to_string(&self) -> &'static str {
        match self {
            Lang::RU => "ru_RU",
            Lang::UA => "ua_UA",
        }
    }
}
