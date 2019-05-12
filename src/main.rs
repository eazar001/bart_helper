extern crate lambda_runtime as lambda;
extern crate alexa_sdk;
extern crate reqwest;
extern crate serde_json;

use lambda::{lambda, Context, error::HandlerError};
use alexa_sdk::{Request,Response};
use alexa_sdk::request::{IntentType};
use std::error::Error;
use std::io::Read;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};


#[derive(Deserialize, Serialize)]
struct Bsa<'a> {
    #[serde(borrow)]
    #[serde(rename = "?xml")]
    xml: Xml<'a>,

    #[serde(borrow)]
    root: Root<'a>
}

#[derive(Deserialize, Serialize)]
struct Xml<'a> {
    #[serde(borrow)]
    #[serde(rename = "@version")]
    version: &'a str,

    #[serde(borrow)]
    #[serde(rename = "@encoding")]
    encoding: &'a str
}

#[derive(Deserialize, Serialize)]
struct Msg<'a> {
    #[serde(borrow)]
    #[serde(rename = "#cdata-section")]
    cdata: &'a str
}

#[derive(Deserialize, Serialize)]
struct BsaPayload<'a> {
    #[serde(borrow)]
    #[serde(rename = "@id")]
    id: &'a str,

    #[serde(borrow)]
    station: &'a str,

    #[serde(borrow)]
    #[serde(rename = "type")]
    advisory_type: &'a str,

    #[serde(borrow)]
    description: Msg<'a>,

    #[serde(borrow)]
    sms_text: Msg<'a>,

    #[serde(borrow)]
    posted: &'a str,

    #[serde(borrow)]
    expires: &'a str
}

#[derive(Deserialize, Serialize)]
struct Root<'a> {
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
    payload: Vec<BsaPayload<'a>>,

    #[serde(borrow)]
    message: &'a str
}

//#[derive(Deserialize, Serialize)]
//struct Fare<'a> {
//    #[serde(borrow)]
//    xml: Xml<'a>,
//
//
//}



fn http_get(url: &str) -> reqwest::Result<String> {
    let mut response = reqwest::get(url)?;
    let mut buffer = String::new();

    response.read_to_string(&mut buffer);

    Ok(buffer)
}

fn handle_help(_req: &Request) -> std::result::Result<Response,HandlerError> {
    Ok(
        Response::new_simple(
            "Usage Help",
            "To get information about BART service advisories and alerts, say:\
            Alexa, ask BART info for updates."
        )
    )
}

fn handle_advisory(_req: &Request) -> std::result::Result<Response,HandlerError> {
    let payload_text = http_get(
        "https://api.bart.gov/api/bsa.aspx?cmd=bsa&key=MW9S-E7SL-26DU-VV8V&json=y"
    );

    let s = &payload_text.unwrap()[..];

    let bsa: Result<Bsa> = serde_json::from_str(s);
    let mut response_buffer = String::new();

    for e in bsa.unwrap().root.payload {
        let response = e.description.cdata;
        response_buffer.push_str(response);
    }

    Ok(
        Response::new_simple(
            "Service Advisories",
            &response_buffer[..]
        )
    )
}

fn handle_cancel(_req: &Request) -> std::result::Result<Response,HandlerError> {
    Ok(Response::end())
}

fn handler(req: Request, _ctx: Context) -> std::result::Result<Response,HandlerError> {

    match req.intent() {
        IntentType::Help => handle_help(&req),
        IntentType::User(s) =>
            match &s[..] {
                "advisory" => handle_advisory(&req),
                _ => handle_cancel(&req)
            }
        _ => handle_cancel(&req)
    }
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    lambda!(handler);

    Ok(())
}
