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
    cdata: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BsaPayload {
    #[serde(rename = "@id")]
    id: Option<String>,
    station: String,
    #[serde(rename = "type")]
    advisory_type: Option<String>,
    description: Msg,
    sms_text: Msg,
    posted: Option<String>,
    expires: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
struct Root {
    #[serde(rename = "@id")]
    id: String,
    uri: Msg,
    date: String,
    time: String,
    #[serde(rename = "bsa")]
    payload: Vec<BsaPayload>,
    message: String
}

impl Response {
    pub fn payload(&self) -> &Vec<BsaPayload> {
        &self.root.payload
    }
}

impl BsaPayload {
    pub fn description(&self) -> &str {
        &self.description.cdata
    }
}