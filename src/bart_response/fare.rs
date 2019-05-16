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
pub struct Msg<'a> {
    #[serde(borrow)]
    #[serde(rename = "#cdata-section")]
    pub cdata: &'a str
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Root<'a> {

    #[serde(borrow)]
    uri: Msg<'a>,

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

    #[serde(borrow)]
    message: Message<'a>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Fares<'a> {

    #[serde(borrow)]
    level: &'a str,

    #[serde(borrow)]
    #[serde(rename = "fare")]
    payload: Vec<FarePayLoad<'a>>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FarePayLoad<'a> {

    #[serde(borrow)]
    #[serde(rename = "@amount")]
    amount: &'a str,

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
pub struct Message<'a> {
    #[serde(borrow)]
    co2_emissions: Msg<'a>
}