use std::{
    io::Read,
    time::{Duration, SystemTime},
};

use geojson::GeoJson;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{Number, Value};

#[derive(Deserialize)]
pub struct WelcomeResponse {
    pub msg: String,
}

#[test]
fn test_deserialize_welcome() {
    let json = r#"
    {
        "msg": "hello! Telraam server 2.0 is up and running"
    }
    "#;

    let welcome = serde_json::from_str::<WelcomeResponse>(json).expect("failed to parse json");
    assert_eq!("hello! Telraam server 2.0 is up and running", welcome.msg);
}

#[derive(Deserialize)]
pub struct TrafficResponse {
    status_code: usize,
    message: String,
    report: Vec<Report>,
}

#[derive(Deserialize)]
pub struct Report {
    instance_id: isize,
    segment_id: isize,
    #[serde(with = "humantime_serde")]
    date: SystemTime,
    interval: String,
    uptime: f32,
    heavy: f32,
    car: f32,
    bike: f32,
    pedestrian: f32,
    heavy_lft: f32,
    heavy_rgt: f32,
    car_lft: f32,
    car_rgt: f32,
    bike_lft: f32,
    bike_rgt: f32,
    pedestrian_lft: f32,
    pedestrian_rgt: f32,
    direction: usize,
    timezone: String,
    car_speed_hist_0to70plus: Vec<f32>,
    car_speed_hist_0to120plus: Vec<f32>,
    v85: f32,
}

#[test]
fn test_deserialize_traffic() {
    let json = r#"
      {
        "status_code": 200,
        "message": "ok",
        "report": [
          {
            "instance_id": -1,
            "segment_id": 348917,
            "date": "2020-10-30T07:00:00.000Z",
            "interval": "hourly",
            "uptime": 0.73,
            "heavy": 78.0821917808,
            "car": 619.1780821918,
            "bike": 263.0136986301,
            "pedestrian": 72.602739726,
            "heavy_lft": 52.0547945205,
            "heavy_rgt": 26.0273972603,
            "car_lft": 202.7397260274,
            "car_rgt": 416.4383561644,
            "bike_lft": 156.1643835616,
            "bike_rgt": 106.8493150685,
            "pedestrian_lft": 41.095890411,
            "pedestrian_rgt": 31.5068493151,
            "direction": 1,
            "timezone": "Europe/Brussels",
            "car_speed_hist_0to70plus": [
              15.9292035398,
              44.2477876106,
              29.203539823,
              5.9734513274,
              1.5486725664,
              0.4424778761,
              0.6637168142,
              1.9911504425
            ],
            "car_speed_hist_0to120plus": [
              5.7522123894,
              10.1769911504,
              19.6902654867,
              24.5575221239,
              18.1415929204,
              11.0619469027,
              4.6460176991,
              1.3274336283,
              1.5486725664,
              0,
              0.4424778761,
              0,
              0.4424778761,
              0.2212389381,
              0.2212389381,
              0.6637168142,
              0.2212389381,
              0,
              0.2212389381,
              0,
              0.2212389381,
              0,
              0,
              0,
              0.4424778761
            ],
            "v85": 25.5
          },
          {
            "instance_id": -1,
            "segment_id": 348917,
            "date": "2020-10-30T08:00:00.000Z",
            "interval": "hourly",
            "uptime": 0.7738888889,
            "heavy": 64.6087580761,
            "car": 462.5987078248,
            "bike": 95.6209619526,
            "pedestrian": 14.2139267767,
            "heavy_lft": 50.3948312994,
            "heavy_rgt": 14.2139267767,
            "car_lft": 164.1062455133,
            "car_rgt": 298.4924623116,
            "bike_lft": 46.5183058148,
            "bike_rgt": 49.1026561378,
            "pedestrian_lft": 7.7530509691,
            "pedestrian_rgt": 6.4608758076,
            "direction": 1,
            "timezone": "Europe/Brussels",
            "car_speed_hist_0to70plus": [
              10.8938547486,
              23.7430167598,
              51.1173184358,
              12.5698324022,
              1.3966480447,
              0.2793296089,
              0,
              0
            ],
            "car_speed_hist_0to120plus": [
              5.5865921788,
              5.3072625698,
              4.748603352,
              18.9944134078,
              30.4469273743,
              20.6703910615,
              9.217877095,
              3.3519553073,
              1.3966480447,
              0,
              0,
              0.2793296089,
              0,
              0,
              0,
              0,
              0,
              0,
              0,
              0,
              0,
              0,
              0,
              0,
              0
            ],
            "v85": 27.5
          }
        ]
      }
    "#;

    let traffic = serde_json::from_str::<TrafficResponse>(json).expect("failed to parse json");
    assert_eq!(200, traffic.status_code);
    assert_eq!("ok", traffic.message);
    assert_eq!(2, traffic.report.len());
    assert_eq!(-1, traffic.report[0].instance_id);
    assert_eq!(
        SystemTime::UNIX_EPOCH + Duration::from_secs(1604041200),
        traffic.report[0].date
    )
}

#[derive(Deserialize)]
pub struct TrafficSnapshot {
    status_code: usize,
    message: String,
    #[serde(flatten)]
    geo: GeoJson,
}

#[test]
fn test_deserialize_traffic_snapshot() {
    let mut json = String::new();

    std::fs::File::open("tests/data/traffic_snapshot.json")
        .expect("failed to open test data")
        .read_to_string(&mut json)
        .expect("failed to read test data");

    let traffic = serde_json::from_str::<TrafficSnapshot>(&json).expect("failed to parse json");
    assert_eq!(200, traffic.status_code);
    assert_eq!("ok", traffic.message);

    let collection = if let GeoJson::FeatureCollection(collection) = &traffic.geo {
        collection
    } else {
        panic!("should have been a feature collection")
    };
    assert!(collection.bbox.is_none());
    assert!(collection.foreign_members.is_none());
    assert_eq!(3675, collection.features.len());

    let feature = &collection.features[0];
    assert!(feature.bbox.is_none());
    assert!(feature.id.is_none());
    assert!(feature.foreign_members.is_none());
    assert_eq!(
        Value::Number(Number::from(24948)),
        feature.properties.as_ref().expect("properties missing")["segment_id"]
    );
    assert!(feature.geometry.is_some());

    let geomentry = feature.geometry.as_ref().unwrap();
    assert!(geomentry.bbox.is_none());
    assert!(geomentry.foreign_members.is_none());

    let multi = if let geojson::Value::MultiLineString(ref multi) = geomentry.value {
        multi
    } else {
        panic!("expected MultiLineString")
    };

    assert_eq!(
        &[4.47577215954854, 51.3021139617358],
        multi[0][0].as_slice()
    );
}
