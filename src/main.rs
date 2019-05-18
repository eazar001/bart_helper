mod bart_response;

use lambda_runtime as lambda;
use self::lambda::{lambda, Context, error::HandlerError};
use alexa_sdk::{Request,Response};
use alexa_sdk::request::{IntentType};
use std::error::Error;
use std::io::Read;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::hash::Hash;


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

    let bsa: Result<bart_response::bsa::Response> = serde_json::from_str(s);
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

fn handle_fare(_req: &Request) -> std::result::Result<Response,HandlerError> {
    let payload_text = http_get(
        "https://api.bart.gov/api/sched.aspx?cmd=fare&orig=12th&dest=embr&date=today&key=MW9S-E7SL-26DU-VV8V&json=y"
    );

    let s= &payload_text.unwrap()[..];
    let fare: Result<bart_response::fare::Response> = serde_json::from_str(s);
    let mut response_buffer = String::new();

    for e in fare.unwrap().root.fares.payload {
        let response = e.amount;
//        response_buffer.push_str(response);
        response_buffer.push_str(response);
        break;
    }

    Ok(
        Response::new_simple(
            "Fares",
            &response_buffer[..]
        )
    )

}

fn handle_cancel(_req: &Request) -> std::result::Result<Response,HandlerError> {
    Ok(Response::end())
}

fn handler(req: Request, _ctx: Context) -> std::result::Result<Response,HandlerError> {
    let stations: HashMap<&str, &str> = [
        ("12th St. Oakland City Center", "12th"),
        ("16th St. Mission (SF)", "16th"),
        ("19th St. Oakland", "19th"),
        ("24th St. Mission (SF)", "24th"),
        ("Ashby (Berkeley)", "ashb")
        ("Antioch", "antc"),
        ("Balboa Park (SF)", "balb"),
        ("Bay Fair (San Leandro)", "bayf"),
        ("Castro Valley", "cast"),
        ("Civic Center (SF)", "civc"),
        ("Coliseum", "cols"),
        ("Colma", "colm"),
        ("Concord", "conc"),
        ("Daly City", "daly"),
        ("Downtown Berkeley", "dbrk"),
        ("Dublin/Pleasanton", "dubl"),
        ("El Cerrito del Norte", "deln"),
        ("El Cerrito Plaza", "plza"),
        ("Embarcadero (SF)", "embr"),
        ("Fremont", "frmt"),
        ("Fruitvale (Oakland)", "ftvl"),
        ("Glen Park (SF)", "glen"),
        ("Hayward", "hayw"),
        ("Lafayette", "lafy"),
        ("Lake Merritt (Oakland)", "lake"),
        ("MacArthur (Oakland)", "mcar"),
        ("Millbrae", "mlbr"),
        ("Montgomery St. (SF)", "mont"),
        ("North Berkeley", "nbrk"),
        ("North Concord/Martinez", "ncon"),
        ("Oakland Int'l Airport", "oakl"),
        ("Orinda", "orin"),
        ("Pittsburg/Bay Point", "pitt"),
        ("Pittsburg Center", "pctr"),
        ("Pleasant Hill", "phil"),
        ("Powell St. (SF)", "powl"),
        ("Richmond", "rich"),
        ("Rockridge (Oakland)", "rock"),
        ("San Bruno", "sbrn"),
        ("San Francisco Int'l Airport", "sfia"),
        ("San Leandro", "sanl"),
        ("South Hayward", "shay"),
        ("South San Francisco", "ssan"),
        ("Union City", "ucty"),
        ("Warm Springs/South Fremont", "warm"),
        ("Walnut Creek", "wcrk"),
        ("West Dublin", "wdub"),
        ("West Oakland", "woak")
    ].iter().cloned().collect();


    match req.intent() {
        IntentType::Help => handle_help(&req),
        IntentType::User(s) =>
            match &s[..] {
                "advisory" => handle_advisory(&req),
                "fare" => handle_fare(&req),
                _ => handle_cancel(&req)
            }
        _ => handle_cancel(&req)
    }
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    lambda!(handler);

    Ok(())
}
