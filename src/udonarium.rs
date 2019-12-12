#[derive(Debug, Serialize)]
struct Character {
    #[serde(rename = "location.name")]
    location_name: String,
    #[serde(rename = "location.x")]
    location_x: String,
    #[serde(rename = "location.y")]
    location_y: String,
    #[serde(rename = "posZ")]
    pos_z: String,
    #[serde(rename = "rotate")]
    rotate: String,
    #[serde(rename = "roll")]
    roll: String,
}

impl Character {
    pub fn new() -> Self {
        Character {
            location_name: String::from("table"),
            location_x: String::from("0"),
            location_y: String::from("0"),
            pos_z: String::from("0"),
            rotate: String::from("0"),
            roll: String::from("0"),
        }
    }
}
