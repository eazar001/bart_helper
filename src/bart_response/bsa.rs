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
pub struct BsaPayload<'a> {
    #[serde(borrow)]
    #[serde(rename = "@id")]
    id: Option<&'a str>,

    #[serde(borrow)]
    station: &'a str,

    #[serde(borrow)]
    #[serde(rename = "type")]
    advisory_type: Option<&'a str>,

    #[serde(borrow)]
    pub description: Msg<'a>,

    #[serde(borrow)]
    sms_text: Msg<'a>,

    #[serde(borrow)]
    posted: Option<&'a str>,

    #[serde(borrow)]
    expires: Option<&'a str>
}

#[derive(Deserialize, Serialize, Debug)]
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

impl<'a> Response<'a> {
    pub fn payload(&self) -> &Vec<BsaPayload> {
        &self.root.payload
    }
}

impl<'a> BsaPayload<'a> {
    pub fn description(&self) -> &str {
        self.description.cdata
    }
}