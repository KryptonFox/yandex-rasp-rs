use serde::Deserialize;

#[derive(Default, Debug)]
pub enum TransportType {
    #[default]
    All,
    Plane,
    Train,
    Suburban,
    Bus,
    Water,
    Helicopter,
}

impl TransportType {
    pub fn to_string(&self) -> String {
        match self {
            &TransportType::All => String::from("all"),
            &TransportType::Plane => String::from("plane"),
            &TransportType::Train => String::from("train"),
            &TransportType::Suburban => String::from("suburban"),
            &TransportType::Bus => String::from("bus"),
            &TransportType::Water => String::from("water"),
            &TransportType::Helicopter => String::from("helicopter"),
        }
    }
}

// Implement Deserialize for TransportTypes
impl<'de> Deserialize<'de> for TransportType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        match s {
            "all" => Ok(TransportType::All),
            "plane" => Ok(TransportType::Plane),
            "train" => Ok(TransportType::Train),
            "suburban" => Ok(TransportType::Suburban),
            "bus" => Ok(TransportType::Bus),
            "water" => Ok(TransportType::Water),
            "helicopter" => Ok(TransportType::Helicopter),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown transport type: {}",
                s
            ))),
        }
    }
}

// impl Serialize for TransportTypes {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let s = self.to_string();
//         serializer.serialize_str(&s)
//     }
// }
