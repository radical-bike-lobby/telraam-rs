//! All Response types from the Telraam API

use std::time::SystemTime;

use geojson::GeoJson;
use serde::{
    de::{self, DeserializeOwned, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::error::Error;

/// All Responses have a status and must be Deserializable
pub trait Response: DeserializeOwned {
    /// Return the status from the request
    fn status(&self) -> &Status;
}

/// Status contains an error and possibly any associated message from the request
#[derive(Clone, Debug, Deserialize)]
pub struct Status {
    /// HTTP type error code, 200 is good
    #[serde(default)]
    pub status_code: usize,
    /// Message returned by Telraam
    #[serde(alias = "msg")]
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

/// The simple response from the Welcome API
#[derive(Deserialize)]
pub struct WelcomeResponse {
    #[serde(flatten)]
    status: Status,
}

impl Response for WelcomeResponse {
    fn status(&self) -> &Status {
        &self.status
    }
}

/// Response from [`crate::endpoint::Traffic`]
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
    /// Get a reference to all the reports returned
    pub fn reports(&self) -> Result<&[Report], Error> {
        self.status.try_to_error()?;
        Ok(&self.reports)
    }

    /// Take the reports from the response
    pub fn take_reports(self) -> Result<Vec<Report>, Error> {
        self.status.try_into_error()?;
        Ok(self.reports)
    }
}

/// Report data returned from the [`crate::endpoint::Traffic`] request
#[derive(Deserialize, Serialize)]
pub struct Report {
    /// the instance identifier for "instance" level calls ("-1" for "segment" level calls)
    pub instance_id: isize,
    /// the segment identifier for "segment" level calls (in the future when "instance" calls are implemented too, this will read "-1")
    pub segment_id: isize,
    /// ISO timeflag (date and UTC time) of the reporting interval (beginning of the interval)
    #[serde(with = "humantime_serde")]
    pub date: SystemTime,
    /// can be "hourly" or "daily" for hourly or daily aggregate data, respectively
    pub interval: String,
    /// between 0 and 1, represents the portion of the reporting interval (hour or day) that was actively spent counting the traffic (background calculation intervals in hourly periods, and the night time in daily periods contribute to values being less than 1)
    pub uptime: f32,
    /// the number of heavy vehicles (called lorry in older APIs, but all stand for the same: anything larger than car) on this day (and in this hour)
    pub heavy: f32,
    /// the number of cars
    pub car: f32,
    /// the number of two-wheelers (mainly cyclists and motorbikes)
    pub bike: f32,
    /// the number of pedestrians
    pub pedestrian: f32,
    /// heavy count from left
    pub heavy_lft: f32,
    /// heavy count from right
    pub heavy_rgt: f32,
    /// car count from left
    pub car_lft: f32,
    /// car count from right
    pub car_rgt: f32,
    /// bike count from left
    pub bike_lft: f32,
    /// bike count from right
    pub bike_rgt: f32,
    /// pedestrian count from left
    pub pedestrian_lft: f32,
    /// pedestrian count from right
    pub pedestrian_rgt: f32,
    /// "1" - disregard, this is an internal consistency value making sure that when multiple cameras on different sides of the street are aggregated then the left and right directions are handled properly
    pub direction: usize,
    /// The name of the Time zone where the segment can be found, which can be used to convert reported UTC timestamps to local times
    pub timezone: String,
    /// the estimated car speed distribution in 10 km/h bins from 0 to 70+ km/h (in percentage of the total 100%)
    pub car_speed_hist_0to70plus: Vec<f32>,
    /// the estimated car speed distribution in 5 km/h bins from 0 to 120+ km/h (in percentage of the total 100%)
    pub car_speed_hist_0to120plus: Vec<f32>,
    /// the estimated car speed limit in km/h that 85% of all cars respect (15% of drivers drive faster than this limit). Just like all other speed related measurements, the accuracy of this value is likely not better than +/-10%.
    pub v85: f32,
}

/// Response from [`crate::endpoint::LiveTrafficSnapshot`]
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
    /// Get a reference to the GeoJson data from the response
    pub fn snapshot(&self) -> Result<&GeoJson, Error> {
        self.status.try_to_error()?;
        Ok(&self.geo)
    }

    /// Take the GeoJson data from the response
    pub fn take_snapshot(self) -> Result<GeoJson, Error> {
        self.status.try_into_error()?;
        Ok(self.geo)
    }
}

/// Response from [`crate::endpoint::AllAvailableCameras`], [`crate::endpoint::CamerasBySegementId`], and [`crate::endpoint::CameraByMacId`]
#[derive(Deserialize)]
pub struct CamerasResponse {
    #[serde(flatten)]
    status: Status,
    #[serde(alias = "camera")]
    cameras: Vec<Camera>,
}

/// Details on the Telraam Camera
#[derive(Deserialize, Serialize)]
pub struct Camera {
    /// The unique identifier of the camera instance
    pub instance_id: isize,
    /// The unique identifier of the camera
    pub mac: usize,
    /// The unique identifier of the user who has registered this camera
    pub user_id: isize,
    /// The unique identifier of the street segment where the camera is installed
    pub segment_id: isize,
    /// The Boolean (false or true) that encodes the side of road (relative to the direction of the segment defined by its coordinate chain) on which the camera is installed
    pub direction: bool,
    /// The status of the camera (active, sending good data / non_active, not sending data / problematic, active but not sending good data)
    pub status: String,
    /// Boolean (false or true) encoding some additional internally used information
    pub manual: bool,
    /// The registration date and time of the instance (UTC)
    #[serde(with = "humantime_serde")]
    pub time_added: SystemTime,
    /// null for active instances, or the date and time of the last active moment for the instance (UTC)
    #[serde(with = "humantime_serde")]
    pub time_end: Option<SystemTime>,
    /// The date and time of the last transferred data packet, if available (UTC)
    #[serde(with = "humantime_serde")]
    pub last_data_package: SystemTime,
    /// The date and time of the first transferred data packet, if available (UTC)
    #[serde(with = "humantime_serde")]
    pub first_data_package: SystemTime,
    /// Boolean (true or false) that encodes if pedestrians are expected to be seen by the field of view of the camera in the left direction (user input)
    pub pedestrians_left: bool,
    /// Boolean (true or false) that encodes if pedestrians are expected to be seen by the field of view of the camera in the right direction (user input)
    pub pedestrians_right: bool,
    /// Boolean (true or false) that encodes if bikes are expected to be seen by the field of view of the camera in the left direction (user input)
    pub bikes_left: bool,
    /// Boolean (true or false) that encodes if bikes are expected to be seen by the field of view of the camera in the right direction (user input)
    pub bikes_right: bool,
    /// Boolean (true or false) that encodes if cars are expected to be seen by the field of view of the camera in the left direction (user input)
    pub cars_left: bool,
    /// Boolean (true or false) that encodes if cars are expected to be seen by the field of view of the camera in the right direction (user input)
    pub cars_right: bool,
    /// A "yes" or "no" field that gives information about the status of the calibration process that is neccesary to distinguish heavy vehicles from cars. When the field is "yes", heavy vehicles will be counted separately, when it is "no", heavy vehicles are still counted as cars.
    #[serde(deserialize_with = "from_yes_no", serialize_with = "to_yes_no")]
    pub is_calibration_done: bool,
}

fn from_yes_no<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct YesNoVisitor;

    impl Visitor<'_> for YesNoVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

fn to_yes_no<S: Serializer>(value: &bool, serializer: S) -> Result<S::Ok, S::Error> {
    if *value {
        serializer.serialize_str("yes")
    } else {
        serializer.serialize_str("no")
    }
}

impl Response for CamerasResponse {
    fn status(&self) -> &Status {
        &self.status
    }
}

impl CamerasResponse {
    /// Get a reference to all the cameras in the response
    pub fn cameras(&self) -> Result<&[Camera], Error> {
        self.status.try_to_error()?;
        Ok(&self.cameras)
    }

    /// Take the cameras from the response
    pub fn take_cameras(self) -> Result<Vec<Camera>, Error> {
        self.status.try_into_error()?;
        Ok(self.cameras)
    }
}

/// Response from [`crate::endpoint::AllSegments`] and [`crate::endpoint::SegmentById`]
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
    /// Get a reference to the GeoJSON data for the segements
    pub fn segments(&self) -> Result<&GeoJson, Error> {
        self.status.try_to_error()?;
        Ok(&self.segment)
    }

    /// Take the GeoJSON data from the segements
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
        assert_eq!(
            "hello! Telraam server 2.0 is up and running",
            welcome.status.message
        );
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
    fn test_deserialize_traffic_snapshot_2023_12_9() {
        let mut json = String::new();

        std::fs::File::open("tests/data/traffic_snapshot_live_2023_12_9.json")
            .expect("failed to open test data")
            .read_to_string(&mut json)
            .expect("failed to read test data");

        serde_json::from_str::<TrafficSnapshotResponse>(&json).expect("failed to parse json");
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

        let cameras = serde_json::from_str::<CamerasResponse>(json).expect("failed to parse json");
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
