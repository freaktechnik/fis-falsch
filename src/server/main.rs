#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate actix_web;
extern crate futures;
extern crate csv;
extern crate bytes;
extern crate actix;

use actix_web::*;
use futures::future::{Future, ok};
use std::str::FromStr;
use bytes::Bytes;
use std::cell::RefCell;

#[derive(Deserialize)]
struct StationQuery {
    q: String,
}

struct AppState {
    stations: RefCell<Vec<String>>,
}

#[derive(Deserialize)]
struct StationList {
    #[serde(rename = "StationID")]
    station_id: u32,
    #[serde(rename = "Station")]
    station: String,
}
#[warn(non_snake_case)]

fn extract_stations(raw_csv: &[u8]) -> Result<Vec<String>, Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .quote(b'"')
        .from_reader(raw_csv);
    let mut stations = Vec::new();
    for result in rdr.deserialize::<StationList>() {
        //TODO somehow this gets confused with the CSV
        match result {
            Err(e) => {
                println!("Saw error {}", e);
            }
            Ok(record) => {
                match get_name(record.station) {
                    Some(val) => stations.push(val),
                    None => (),
                };
            }
        };

    }

    Ok(stations)
}

fn fetch_stations() -> impl Future<Item=Vec<String>, Error=Error> {
    let req = client::get("https://opentransportdata.swiss/dataset/695c7af6-d486-4cde-9bf0-a92fdd581a4e/resource/be85ce3e-374f-49b1-ad93-8b5cc3f17b12/download/bahnhof.csv").finish().unwrap();
    req.send()
        .from_err::<Error>()
        .and_then(move |response| {
            response.body()
                .limit(1024 * 1024)
                .from_err()
        })
        .from_err::<Error>()
        .and_then(move |body: Bytes| extract_stations(&body[..]))
}

fn update_state(state: &AppState) -> FutureResponse<Vec<String>> {
    let stations = state.stations.borrow();
    if stations.len() > 0 {
        Box::new(ok(stations.clone()))
    }
    else {
        Box::new(fetch_stations())
    }
}

fn get_stations((query, state): (Query<StationQuery>, State<AppState>)) -> FutureResponse<HttpResponse> {
    update_state(&state as &AppState).join(ok(state))
        .then(move |result| {
            match result {
                Err(e) => {
                    println!("Got error {}", e);
                    Ok(HttpResponse::InternalServerError().finish())
                },
                Ok((stations, my_state)) => {
                    let mut results = Vec::new();
                    my_state.stations.replace(stations.clone());
                    let pattern = query.q.as_str();
                    let my_stations = stations.clone();
                    for station in my_stations {
                        if station.as_str().starts_with(pattern) {
                            results.push(station);
                        }
                    }
                    Ok(HttpResponse::Ok().json(results))
                }
            }
        })
        .responder()
}

enum SubmissionType {
    Train,
    Station,
}

enum DefectType {
    Incorrect,
    Defect,
}

#[derive(Deserialize)]
struct Submission {
    #[serde(rename = "type")]
    sub_type: String,
    train: String,
    station: String,
    #[serde(rename = "defectType")]
    defect_type: String,
}

fn submit_outage(data: Json<Submission>) -> HttpResponse {
    HttpResponse::Ok().into()
}

fn get_name(station_info: String) -> Option<String> {
    let parts = station_info.split("$");
    let mut name = "";
    let mut context: u32 = 0;
    for s in parts {
        if context == 0 {
            name = s;
            context = 1;
        }
        else {
            context = 0;
            match s {
                "<1>" => return Some(name.to_string()),
                _ => (),
            }
        }
    }
    None
}

fn main() {
    server::new(|| {
        App::with_state(AppState {
            stations: RefCell::new(Vec::new())
        })
            .resource("/api/get-stations", |r| r.get().with_async(get_stations))
            .resource("/api/submit-outage", |r| r.post().with(submit_outage))
            .handler("/", fs::StaticFiles::new("./dist").unwrap())
    }).bind("127.0.0.1:8088")
        .unwrap()
        .run();
}

impl FromStr for SubmissionType {
    type Err = ();

    fn from_str(s: &str) -> Result<SubmissionType, ()> {
        match s {
            "train" => Ok(SubmissionType::Train),
            "station" => Ok(SubmissionType::Station),
            _ => Err(()),
        }
    }
}

impl FromStr for DefectType {
    type Err = ();

    fn from_str(s: &str) -> Result<DefectType, ()> {
        match s {
            "incorrect" => Ok(DefectType::Incorrect),
            "defect" => Ok(DefectType::Defect),
            _ => Err(()),
        }
    }
}
