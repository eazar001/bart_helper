use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    #[serde(rename = "?xml")]
    xml: Xml,
    root: Root
}

#[derive(Deserialize, Serialize, Debug)]
struct Xml {
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "@encoding")]
    encoding: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Msg {
    #[serde(rename = "#cdata-section")]
    pub cdata: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Root {
    uri: Msg,
    origin: String,
    destination: String,
    trip: Trip,
    #[serde(rename = "fares")]
    fares: Fares,
    message: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Fares {
    #[serde(rename = "@level")]
    level: String,
    #[serde(rename = "fare")]
    payload: Vec<FarePayLoad>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FarePayLoad {
    #[serde(rename = "@amount")]
    amount: String,
    #[serde(rename = "@class")]
    class: String,
    #[serde(rename = "@name")]
    name: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Trip {
    fare: String,
    discount: Discount
}

#[derive(Deserialize, Serialize, Debug)]
struct Discount {
    clipper: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    co2_emissions: Msg
}

impl Response {
    pub fn payload(&self) -> &Vec<FarePayLoad> {
        &self.root.fares.payload
    }
}

impl FarePayLoad {
    pub fn fare_type(&self) -> &str {
        &self.name
    }

    pub fn amount(&self) -> &str {
        &self.amount
    }
}