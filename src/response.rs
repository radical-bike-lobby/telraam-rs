use std::time::SystemTime;

use geojson::GeoJson;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

use crate::error::Error;

trait Response {
    fn status(&self) -> &Status;
}

#[derive(Deserialize)]
pub struct WelcomeResponse {
    pub msg: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Status {
    pub status_code: usize,
    pub message: String,
}

impl Status {
    fn try_to_error(&self) -> Result<(), Error> {
        if self.status_code <= 299 {
            Ok(())
        } else {
            Err(Error::Non200Response(self.clone()))
        }
    }

    fn try_into_error(self) -> Result<(), Error> {
        if self.status_code <= 299 {
            Ok(())
        } else {
            Err(Error::Non200Response(self))
        }
    }
}

#[derive(Deserialize)]
pub struct TrafficResponse {
    #[serde(flatten)]
    status: Status,
    #[serde(rename = "report")]
    reports: Vec<Report>,
}

impl Response for TrafficResponse {
    fn status(&self) -> &Status {
        &self.status
    }
}

impl TrafficResponse {
    pub fn reports(&self) -> Result<&[Report], Error> {
        self.status.try_to_error()?;
        Ok(&self.reports)
    }

    pub fn take_reports(self) -> Result<Vec<Report>, Error> {
        self.status.try_into_error()?;
        Ok(self.reports)
    }
}

#[derive(Deserialize)]
pub struct Report {
    pub instance_id: isize,
    pub segment_id: isize,
    #[serde(with = "humantime_serde")]
    pub date: SystemTime,
    pub interval: String,
    pub uptime: f32,
    pub heavy: f32,
    pub car: f32,
    pub bike: f32,
    pub pedestrian: f32,
    pub heavy_lft: f32,
    pub heavy_rgt: f32,
    pub car_lft: f32,
    pub car_rgt: f32,
    pub bike_lft: f32,
    pub bike_rgt: f32,
    pub pedestrian_lft: f32,
    pub pedestrian_rgt: f32,
    pub direction: usize,
    pub timezone: String,
    pub car_speed_hist_0to70plus: Vec<f32>,
    pub car_speed_hist_0to120plus: Vec<f32>,
    pub v85: f32,
}

#[derive(Deserialize)]
pub struct TrafficSnapshotResponse {
    #[serde(flatten)]
    status: Status,
    #[serde(flatten)]
    geo: GeoJson,
}

impl Response for TrafficSnapshotResponse {
    fn status(&self) -> &Status {
        &self.status
    }
}

impl TrafficSnapshotResponse {
    pub fn snapshot(&self) -> Result<&GeoJson, Error> {
        self.status.try_to_error()?;
        Ok(&self.geo)
    }

    pub fn take_snapshot(self) -> Result<GeoJson, Error> {
        self.status.try_into_error()?;
        Ok(self.geo)
    }
}

#[derive(Deserialize)]
pub struct AllCamerasResponse {
    #[serde(flatten)]
    status: Status,
    cameras: Vec<Camera>,
}

#[derive(Deserialize)]
pub struct Camera {
    pub instance_id: isize,
    pub mac: usize,
    pub user_id: isize,
    pub segment_id: isize,
    pub direction: bool,
    pub status: String,
    pub manual: bool,
    #[serde(with = "humantime_serde")]
    pub time_added: SystemTime,
    #[serde(with = "humantime_serde")]
    pub time_end: Option<SystemTime>,
    #[serde(with = "humantime_serde")]
    pub last_data_package: SystemTime,
    #[serde(with = "humantime_serde")]
    pub first_data_package: SystemTime,
    pub pedestrians_left: bool,
    pub pedestrians_right: bool,
    pub bikes_left: bool,
    pub bikes_right: bool,
    pub cars_left: bool,
    pub cars_right: bool,
    #[serde(deserialize_with = "from_yes_no")]
    pub is_calibration_done: bool,
}

fn from_yes_no<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct YesNoVisitor;

    impl Visitor<'_> for YesNoVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a 'yes' or 'no' value")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match v {
                "yes" => Ok(true),
                "no" => Ok(false),
                _ => Err(de::Error::unknown_variant(v, &["yes", "no"])),
            }
        }
    }

    deserializer.deserialize_str(YesNoVisitor)
}

impl Response for AllCamerasResponse {
    fn status(&self) -> &Status {
        &self.status
    }
}

impl AllCamerasResponse {
    pub fn cameras(&self) -> Result<&[Camera], Error> {
        self.status.try_to_error()?;
        Ok(&self.cameras)
    }

    pub fn take_cameras(self) -> Result<Vec<Camera>, Error> {
        self.status.try_into_error()?;
        Ok(self.cameras)
    }
}

#[derive(Deserialize)]
pub struct SegmentResponse {
    #[serde(flatten)]
    status: Status,
    #[serde(flatten)]
    segment: GeoJson,
}

impl Response for SegmentResponse {
    fn status(&self) -> &Status {
        &self.status
    }
}

impl SegmentResponse {
    pub fn segments(&self) -> Result<&GeoJson, Error> {
        self.status.try_to_error()?;
        Ok(&self.segment)
    }

    pub fn take_segments(self) -> Result<GeoJson, Error> {
        self.status.try_into_error()?;
        Ok(self.segment)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::Read,
        time::{Duration, SystemTime},
    };

    use geojson::GeoJson;
    use serde_json::{Number, Value};

    use super::*;

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
        assert_eq!(200, traffic.status.status_code);
        assert_eq!("ok", traffic.status.message);
        assert_eq!(2, traffic.reports.len());
        assert_eq!(-1, traffic.reports[0].instance_id);
        assert_eq!(
            SystemTime::UNIX_EPOCH + Duration::from_secs(1604041200),
            traffic.reports[0].date
        )
    }

    #[test]
    fn test_deserialize_traffic_snapshot() {
        let mut json = String::new();

        std::fs::File::open("tests/data/traffic_snapshot.json")
            .expect("failed to open test data")
            .read_to_string(&mut json)
            .expect("failed to read test data");

        let traffic =
            serde_json::from_str::<TrafficSnapshotResponse>(&json).expect("failed to parse json");
        assert_eq!(200, traffic.status.status_code);
        assert_eq!("ok", traffic.status.message);

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

    #[test]
    fn test_deserialize_all_cameras() {
        let json = r#"
    {
        "status_code": 200,
        "message": "ok",
        "cameras": [
          {
            "instance_id": 1692,
            "mac": 202481587145269,
            "user_id": 414,
            "segment_id": 348917,
            "direction": true,
            "status": "non_active",
            "manual": false,
            "time_added": "2019-10-02T19:42:54.343Z",
            "time_end": null,
            "last_data_package": "2021-05-02T10:56:15.402Z",
            "first_data_package": "1970-01-01T00:00:00.000Z",
            "pedestrians_left": false,
            "pedestrians_right": true,
            "bikes_left": true,
            "bikes_right": true,
            "cars_left": true,
            "cars_right": true,
            "is_calibration_done": "yes"
          },
          {
            "instance_id": 1691,
            "mac": 202481587145269,
            "user_id": 414,
            "segment_id": 348917,
            "direction": false,
            "status": "non_active",
            "manual": false,
            "time_added": "2019-06-26T07:00:37.546Z",
            "time_end": "2019-10-02T19:42:54.343Z",
            "last_data_package": "2021-01-05T08:33:37.913Z",
            "first_data_package": "1970-01-01T00:00:00.000Z",
            "pedestrians_left": false,
            "pedestrians_right": true,
            "bikes_left": true,
            "bikes_right": true,
            "cars_left": true,
            "cars_right": true,
            "is_calibration_done": "no"
          }
        ]
      }
    "#;

        let cameras =
            serde_json::from_str::<AllCamerasResponse>(json).expect("failed to parse json");
        assert_eq!(200, cameras.status.status_code);
        assert_eq!("ok", cameras.status.message);
        assert!(cameras.cameras[0].is_calibration_done);
        assert!(!cameras.cameras[1].is_calibration_done);
    }

    #[test]
    fn test_deserialize_segment() {
        let json = r#"
          {
            "status_code": 200,
            "message": "ok",
            "type": "FeatureCollection",
            "features": [
              {
                "type": "Feature",
                "geometry": {
                  "type": "MultiLineString",
                  "coordinates": [
                    [
                      [
                        4.71129799121917,
                        50.8643967118925
                      ],
                      [
                        4.71131689135236,
                        50.8642862877756
                      ],
                      [
                        4.71132373881259,
                        50.8642670067756
                      ],
                      [
                        4.71137253782927,
                        50.8641296300352
                      ],
                      [
                        4.71143685119098,
                        50.8639917424206
                      ],
                      [
                        4.71151928065087,
                        50.8638146687918
                      ],
                      [
                        4.7116242705432,
                        50.8636377263723
                      ],
                      [
                        4.71172539021883,
                        50.8635120762697
                      ],
                      [
                        4.71176427433283,
                        50.8633690745834
                      ],
                      [
                        4.71188626758468,
                        50.8627563858191
                      ],
                      [
                        4.71190872556415,
                        50.8626435939865
                      ]
                    ]
                  ]
                },
                "properties": {
                  "oidn": 348917,
                  "first_data_package": "2019-06-26T11:00:00.000Z",
                  "last_data_package": "2021-01-25T09:35:43.725Z",
                  "speed": 50,
                  "oneway": false,
                  "road_type": "",
                  "road_speed": "",
                  "pedestrian": 13.1787675411836,
                  "bike": 32.9469188529591,
                  "car": 237.217815741306,
                  "lorry": 70.2867602196461,
                  "speed_histogram": [
                    19.7681513117755,
                    30.7504575960952,
                    133.9841366687,
                    46.1256863941428,
                    4.39292251372788,
                    2.19646125686394
                  ],
                  "speed_buckets": [
                    0,
                    1,
                    2,
                    3,
                    4,
                    5
                  ]
                }
              }
            ]
          }
        "#;

        let segment = serde_json::from_str::<SegmentResponse>(json).expect("failed to parse json");
        assert_eq!(200, segment.status.status_code);
        assert_eq!("ok", segment.status.message);
    }
}
