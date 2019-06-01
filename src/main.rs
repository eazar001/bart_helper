mod bart;
mod bart_response;

use lambda_runtime as lambda;
use self::lambda::{lambda, Context, error::HandlerError};
use alexa_sdk::{Request, Response};
use alexa_sdk::response::{Speech, Card};
use alexa_sdk::request::{IntentType};
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;
use serde_json::{Result};
use lazy_static::lazy_static;
use bart::error::BartError;
use bart::error::BartError::{InvalidStation, NoConnection};


lazy_static! {
    static ref STATIONS: HashMap<&'static str, &'static str> = {
        stations()
    };
}

fn stations() -> HashMap<&'static str, &'static str> {[
    ("twelfth street oakland city center", "12th"),
    ("12th street oakland city center", "12th"),
    ("twelfth street", "12th"),
    ("12th street", "12th"),
    ("12 street", "12th"),
    ("12 street oakland", "12th"),
    ("12 street oakland city center", "12th"),
    ("12 street oakland city", "12th"),
    ("twelve street", "12th"),
    ("twelfth street oakland", "12th"),
    ("12th street oakland", "12th"),
    ("oakland city center", "12th"),
    ("16 street mission", "16th"),
    ("sixteenth street mission", "16th"),
    ("16th street mission", "16th"),
    ("sixteenth street", "16th"),
    ("16 street", "16th"),
    ("16th street", "16th"),
    ("nineteenth street oakland", "19th"),
    ("19th street oakland", "19th"),
    ("19 street oakland", "19th"),
    ("19 street", "19th"),
    ("nineteen street", "19th"),
    ("nineteenth street", "19th"),
    ("19th street", "19th"),
    ("24 street mission", "24th"),
    ("24 street", "24th"),
    ("twenty fourth street mission", "24th"),
    ("24th street mission", "24th"),
    ("twenty four street", "24th"),
    ("24th street", "24th"),
    ("ashby", "ashb"),
    ("antioch", "antc"),
    ("balboa park", "balb"),
    ("bay fair", "bayf"),
    ("castro valley", "cast"),
    ("civic center", "civc"),
    ("coliseum", "cols"),
    ("colosseum", "cols"),
    ("the coliseum", "cols"),
    ("the colosseum", "cols"),
    ("oakland coliseum", "cols"),
    ("oakland colosseum", "cols"),
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
    ("mccarthy", "mcar"),
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
    ("west oakland", "woak")].iter().cloned().collect()
}


fn bart_connection_err_msg() -> Response {
    let response = Response::new_simple(
            "Bart Services Connection Issues",
            "Sorry I'm having difficulties connecting to Bart Information Services right now.\n\
            The network may be down. Please try again later."
        );

    response
}

fn http_get(url: &str) -> std::result::Result<String, BartError> {
    let mut response = match reqwest::get(url){
        Ok(r) => r,
        Err(_) => return Err(BartError::NoConnection)
    };

    Ok(response.text().unwrap())
}

fn get_help(_req: &Request) -> std::result::Result<Response, BartError> {
    let response = Response::new(true)
        .card(Card::simple(
            "Usage Help",
            "To get information about BART service advisories and alerts, say: \
            Alexa, ask BART info for updates. To inquire about fares between stations say \
            a command such as: Alexa, ask BART info for the fare from West Oakland to \
            Richmond, or Alexa, ask BART info for the fare from South San Francisco to \
            Lake Merritt. To avoid confusion, try to pronounce the station names clearly."
        ))
        .speech(Speech::ssml(
            "<speak>To get information about <phoneme alphabet='ipa' ph='bɑɹt'>BART</phoneme> \
            service advisories and alerts, say: Alexa, ask \
            <phoneme alphabet='ipa' ph='bɑɹt'>BART</phoneme> info for updates. To inquire about \
            fares between stations say a command such as: Alexa, ask \
            <phoneme alphabet='ipa' ph='bɑɹt'>BART</phoneme> info for the fare from West Oakland \
            to Richmond, or Alexa, ask <phoneme alphabet='ipa' ph='bɑɹt'>BART</phoneme> info for \
            the fare from South San Francisco to Lake Merritt. To avoid confusion, try to \
            prounounce the station names clearly.</speak>"
        ));

    Ok(response)
}

fn get_advisory(_req: &Request) -> std::result::Result<Response, BartError> {
    let payload_text = http_get(
        "https://api.bart.gov/api/bsa.aspx?cmd=bsa&key=MW9S-E7SL-26DU-VV8V&json=y"
    )?;

    let s = &payload_text[..];

    let bsa: Result<bart_response::bsa::Response> = serde_json::from_str(s);
    let mut response_buffer = String::new();

    for e in bsa.unwrap().root.payload {
        response_buffer.push_str(e.description.cdata);
    }

    Ok(
        Response::new_simple(
            "Service Advisories",
            &response_buffer[..]
        )
    )
}

fn invalid_key(s: &str) -> Response {
    let response = Response::new_simple(
        "Invalid station",
        &format!("Sorry I thought I heard you say \"{}\", but that is not a valid station.\n\
        Please try again.", s)
    );

    response
}

fn dollar_amount(s: &str) -> String {
    let price: f32 = s.parse().unwrap();
    let dollars = price.floor();
    let cents = price - dollars;
    let one_dollar = dollars == 1.00;

    if dollars == 0.0 {
        let cents = &format!("{:.2} cents", cents)[2..];

        match &cents.to_string()[..2] {
            "00" => return String::from("0 dollars"),
            "01" => return String::from("1 penny"),
            "02" => return String::from("2 cents"),
            "03" => return String::from("3 cents"),
            "04" => return String::from("4 cents"),
            "05" => return String::from("5 cents"),
            "06" => return String::from("6 cents"),
            "07" => return String::from("7 cents"),
            "08" => return String::from("8 cents"),
            "09" => return String::from("9 cents"),
            _ => return cents.to_string()
        }
    }

    let dollars = &match one_dollar {
        true => format!("{} dollar", (dollars as u32).to_string()),
        _ => format!("{} dollars", (dollars as u32).to_string())
    };

    let cents = &format!("{:.2} cents", cents)[2..];
    let mut price: String = String::new();

    let small_change = match cents.chars().next().unwrap() {
        '0' => true,
        _ => false
    };

    price = match &cents.to_string()[..2] {
        "00" => {
            price.push_str(dollars);
            price
        },

        _ => {
            if small_change {
                let cents = &format!("{:.2}cents", &cents.to_string()[1..]);
                price.push_str(&format!("{} {}", dollars, cents));
            } else {
                price.push_str(&format!("{} {}", dollars, cents));
            }

            price
        }
    };

    price
}

fn get_station(station: &str) -> std::result::Result<&str, BartError> {
    match STATIONS.get(station) {
        Some(s) => Ok(s),
        None => Err(InvalidStation(String::from(station)))
    }
}

fn get_fare(req: &Request) -> std::result::Result<Response, BartError> {
    let daily_re = Regex::new(r"(daily) ").unwrap();

    let origin_lower = req.slot_value("origin")
        .map(|c| c.to_lowercase())
        .unwrap();

    let dest_lower = req.slot_value("dest")
        .map(|c| c.to_lowercase())
        .unwrap();

    let origin_key = daily_re.replace_all(&origin_lower, "daly ");
    let dest_key = daily_re.replace_all(&dest_lower, "daly ");

    let origin = get_station(&origin_key[..])?;
    let dest = get_station(&dest_key[..])?;

    let url = format!("https://api.bart.gov/api/sched.aspx?cmd=fare&\
        orig={}&dest={}&date=today&key=MW9S-E7SL-26DU-VV8V&json=y", origin, dest);

    let payload_text = http_get(&url)?;

    let s= &payload_text[..];
    let fare: Result<bart_response::fare::Response> = serde_json::from_str(s);
    let mut response_buffer = String::new();
    let mut response = String::new();
    let mut card_response = String::new();

    for e in fare.unwrap().root.fares.payload {
        let payment_method = match e.name {
            "Senior/Disabled Clipper" => "Senior Disabled Clipper",
            _ => e.name
        };

        response.push_str(&format!("{}, paying by {}.\n", dollar_amount(e.amount), payment_method));
        card_response.push_str(&format!("{}: ${}\n", e.name, e.amount));
    }

    response_buffer.push_str(&response);

    let response = Response::new(true)
        .speech(Speech::plain(&response_buffer))
        .card(Card::simple("Fares", &card_response));

    Ok(response)
}

fn handler(req: Request, _ctx: Context) -> std::result::Result<Response, HandlerError> {
    let result = match req.intent() {
        IntentType::Help => get_help(&req),
        IntentType::User(s) =>
            match &s[..] {
                "advisory" => get_advisory(&req),
                "fare" => get_fare(&req),
                _ => get_help(&req)
            }
        _ => get_help(&req)
    };

    match result {
        Ok(response) => Ok(response),
        Err(InvalidStation(station)) => Ok(invalid_key(&station)),
        Err(NoConnection) => Ok(bart_connection_err_msg()),
        Err(e) => Err(HandlerError::new(e))
    }
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    lambda!(handler);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;


    fn test_request(request_file: &str) {
        let f = File::open(format!("tests/{}", request_file)).unwrap();
        let bart_request= serde_json::from_reader(f).unwrap();
        let response = handler(bart_request, Context::default()).unwrap();

        let f = File::open(format!("tests/expected_output/{}", request_file)).unwrap();
        let expected_response: Response = serde_json::from_reader(f).unwrap();

        let response = serde_json::to_string(&response).unwrap();
        let expected_response = serde_json::to_string(&expected_response).unwrap();

        println!("{:?}", expected_response);
        println!("{:?}", response);
        assert_eq!(response, expected_response);
    }

    #[test]
    fn test_help_request() {
        test_request("help_request.json")
    }

    #[test]
    fn test_fare_colma_concord_request() {
        test_request("fare_colma_concord_request.json")
    }

    #[test]
    fn test_service_advisory() {
        test_request("service_advisory.json")
    }

    #[test]
    fn test_invalid_station() {
        test_request("invalid_station.json")
    }
}