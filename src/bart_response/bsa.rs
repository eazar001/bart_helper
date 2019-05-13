extern crate serde_json;

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct Response<'a> {
    #[serde(borrow)]
    #[serde(rename = "?xml")]
    xml: Xml<'a>,

    #[serde(borrow)]
    pub root: Root<'a>
}

#[derive(Deserialize, Serialize)]
pub struct Xml<'a> {
    #[serde(borrow)]
    #[serde(rename = "@version")]
    version: &'a str,

    #[serde(borrow)]
    #[serde(rename = "@encoding")]
    encoding: &'a str
}

#[derive(Deserialize, Serialize)]
pub struct Msg<'a> {
    #[serde(borrow)]
    #[serde(rename = "#cdata-section")]
    pub cdata: &'a str
}

#[derive(Deserialize, Serialize)]
pub struct BsaPayload<'a> {
    #[serde(borrow)]
    #[serde(rename = "@id")]
    id: &'a str,

    #[serde(borrow)]
    station: &'a str,

    #[serde(borrow)]
    #[serde(rename = "type")]
    advisory_type: &'a str,

    #[serde(borrow)]
    pub description: Msg<'a>,

    #[serde(borrow)]
    sms_text: Msg<'a>,

    #[serde(borrow)]
    posted: &'a str,

    #[serde(borrow)]
    expires: &'a str
}

#[derive(Deserialize, Serialize)]
pub struct Root<'a> {
    #[serde(borrow)]
    #[serde(rename = "@id")]
    id: &'a str,

    #[serde(borrow)]
    uri: Msg<'a>,

    #[serde(borrow)]
    date: &'a str,

    #[serde(borrow)]
    time: &'a str,

    #[serde(borrow)]
    #[serde(rename = "bsa")]
    pub payload: Vec<BsaPayload<'a>>,

    #[serde(borrow)]
    message: &'a str
}