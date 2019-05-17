use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Response<'a> {
    #[serde(borrow)]
    #[serde(rename = "?xml")]
    xml: Xml<'a>,

    #[serde(borrow)]
    pub root: Root<'a>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Xml<'a> {
    #[serde(borrow)]
    #[serde(rename = "@version")]
    version: &'a str,

    #[serde(borrow)]
    #[serde(rename = "@encoding")]
    encoding: &'a str
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Msg {

    #[serde(rename = "#cdata-section")]
    pub cdata: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Root<'a> {

    uri: Msg,

    #[serde(borrow)]
    origin: &'a str,

    #[serde(borrow)]
    destination: &'a str,

    #[serde(borrow)]
    sched_num: &'a str,

    #[serde(borrow)]
    trip: Trip<'a>,

    #[serde(borrow)]
    #[serde(rename = "fares")]
    pub fares: Fares<'a>,

    message: Message
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Fares<'a> {

    #[serde(borrow)]
    #[serde(rename = "@level")]
    level: &'a str,

    #[serde(borrow)]
    #[serde(rename = "fare")]
    pub payload: Vec<FarePayLoad<'a>>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FarePayLoad<'a> {

    #[serde(borrow)]
    #[serde(rename = "@amount")]
    pub amount: &'a str,

    #[serde(borrow)]
    #[serde(rename = "@class")]
    class: &'a str,

    #[serde(borrow)]
    #[serde(rename = "@name")]
    name: &'a str
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Trip<'a> {

    #[serde(borrow)]
    fare: &'a str,

    #[serde(borrow)]
    discount: Discount<'a>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Discount<'a> {

    #[serde(borrow)]
    clipper: &'a str
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {

    co2_emissions: Msg
}