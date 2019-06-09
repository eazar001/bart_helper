use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Response<'a> {
    #[serde(borrow)]
    #[serde(rename = "?xml")]
    xml: Xml<'a>,

    #[serde(borrow)]
    root: Root<'a>
}

#[derive(Deserialize, Serialize, Debug)]
struct Xml<'a> {
    #[serde(borrow)]
    #[serde(rename = "@version")]
    version: &'a str,

    #[serde(borrow)]
    #[serde(rename = "@encoding")]
    encoding: &'a str
}

#[derive(Deserialize, Serialize, Debug)]
struct Msg {
    #[serde(rename = "#cdata-section")]
    cdata: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BsaPayload<'a> {
    #[serde(borrow)]
    #[serde(rename = "@id")]
    id: Option<&'a str>,

    #[serde(borrow)]
    station: &'a str,

    #[serde(borrow)]
    #[serde(rename = "type")]
    advisory_type: Option<&'a str>,

    description: Msg,

    sms_text: Msg,

    #[serde(borrow)]
    posted: Option<&'a str>,

    #[serde(borrow)]
    expires: Option<&'a str>
}

#[derive(Deserialize, Serialize, Debug)]
struct Root<'a> {
    #[serde(borrow)]
    #[serde(rename = "@id")]
    id: &'a str,

    uri: Msg,

    #[serde(borrow)]
    date: &'a str,

    #[serde(borrow)]
    time: &'a str,

    #[serde(borrow)]
    #[serde(rename = "bsa")]
    payload: Vec<BsaPayload<'a>>,

    #[serde(borrow)]
    message: &'a str
}

impl<'a> Response<'a> {
    pub fn payload(&self) -> &Vec<BsaPayload> {
        &self.root.payload
    }
}

impl<'a> BsaPayload<'a> {
    pub fn description(&self) -> &str {
        &self.description.cdata
    }
}