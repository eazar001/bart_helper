mod bart_response;

use lambda_runtime as lambda;
use self::lambda::{lambda, Context, error::HandlerError};
use alexa_sdk::{Request,Response};
use alexa_sdk::request::{IntentType};
use std::error::Error;
use std::io::Read;
use std::collections::HashMap;
use regex::RegexSet;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::hash::Hash;


fn stations() -> HashMap<&'static str, &'static str> {
    [
        ("twelfth street oakland city center", "12th"),
        ("twelfth street", "12th"),
        ("twelfth street oakland", "12th"),
        ("oakland city center", "12th"),
        ("sixteenth street mission", "16th"),
        ("sixteenth street", "16th"),
        ("nineteenth street oakland", "19th"),
        ("nineteenth street", "19th"),
        ("twenty fourth street mission", "24th"),
        ("ashby", "ashb"),
        ("antioch", "antc"),
        ("balboa park", "balb"),
        ("bay fair", "bayf"),
        ("castro valley", "cast"),
        ("civic center", "civc"),
        ("coliseum", "cols"),
        ("oakland coliseum", "cols"),
        ("colma", "colm"),
        ("coma", "colm"),
        ("concord", "conc"),
        ("daly city", "daly"),
        ("downtown berkeley", "dbrk"),
        ("dublin pleasanton", "dubl"),
        ("east dublin", "dubl"),
        ("el cerrito del norte", "deln"),
        ("el cerrito plaza", "plza"),
        ("embarcadero", "embr"),
        ("fremont", "frmt"),
        ("fruitvale", "ftvl"),
        ("glen park", "glen"),
        ("hayward", "hayw"),
        ("lafayette", "lafy"),
        ("lake merritt", "lake"),
        ("macarthur", "mcar"),
        ("millbrae", "mlbr"),
        ("montgomery street", "mont"),
        ("north berkeley", "nbrk"),
        ("north concord martinez", "ncon"),
        ("north concord", "ncon"),
        ("martinez", "ncon"),
        ("oakland international airport", "oakl"),
        ("oakland airport", "oakl"),
        ("oaktown airport", "oakl"),
        ("oak", "oakl"),
        ("o.a.k", "oakl"),
        ("orinda", "orin"),
        ("pittsburg bay point", "pitt"),
        ("bay point", "pitt"),
        ("pittsburg center", "pctr"),
        ("pleasant hill", "phil"),
        ("powell street", "powl"),
        ("powell", "powl"),
        ("richmond", "rich"),
        ("rockridge", "rock"),
        ("san bruno", "sbrn"),
        ("san francisco international airport", "sfia"),
        ("san francisco airport", "sfia"),
        ("frisco airport", "sfia"),
        ("sfo", "sfia"),
        ("s.f.o", "sfia"),
        ("san leandro", "sanl"),
        ("south hayward", "shay"),
        ("south san francisco", "ssan"),
        ("south frisco", "ssan"),
        ("union city", "ucty"),
        ("warm springs south fremont", "warm"),
        ("warm springs", "warm"),
        ("south fremont", "warm"),
        ("walnut creek", "wcrk"),
        ("west dublin", "wdub"),
        ("west oakland", "woak")
    ].iter().cloned().collect()
}

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

fn handle_fare(req: &Request, stations: &HashMap<&str, &str>) -> std::result::Result<Response,HandlerError> {
    let daily_re = Regex::new(r"(daily) ").unwrap();


    let origin_lower = req.slot_value("origin").unwrap().to_lowercase();
    let dest_lower = req.slot_value("dest").unwrap().to_lowercase();

    let origin_key = daily_re.replace_all(&origin_lower, "daly ");
    let dest_key = daily_re.replace_all(&dest_lower, "daly ");

    let origin = stations.get(&origin_key[..]).unwrap();
    let dest = stations.get(&dest_key[..]).unwrap();

    let url = format!("https://api.bart.gov/api/sched.aspx?cmd=fare&\
        orig={}&dest={}&date=today&key=MW9S-E7SL-26DU-VV8V&json=y", origin, dest);

    let payload_text = http_get(&url);

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
    let stations = stations();

    match req.intent() {
        IntentType::Help => handle_help(&req),
        IntentType::User(s) =>
            match &s[..] {
                "advisory" => handle_advisory(&req),
                "fare" => handle_fare(&req, &stations),
                _ => handle_cancel(&req)
            }
        _ => handle_cancel(&req)
    }
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    lambda!(handler);

    Ok(())
}
