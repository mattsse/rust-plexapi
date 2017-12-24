#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct Genre {
    pub tag: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct Writer {
    pub tag: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct Director {
    pub tag: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct Country {
    pub tag: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct Role {
    pub tag: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct Actor {
    pub tag: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct Similar {
    pub id: String,
    pub filter: String,
    pub tag: String,
}

