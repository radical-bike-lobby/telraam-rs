//! All Endpoints are intended to be used with the [`TelraamClient`]

use std::{collections::HashMap, time::SystemTime};

#[cfg(feature = "clap")]
use clap::{Args, Parser, ValueEnum};
use reqwest::Method;
use serde::{Serialize, Serializer};

use crate::response::{
    CamerasResponse, Response, SegmentResponse, TrafficResponse, TrafficSnapshotResponse,
    WelcomeResponse,
};

/// Endpoint is a trait that defines the shape of all the API endpoints in Telraam
///
/// An instance of the Endpoint will contain the parameters to be used for any subsequent queries.
pub trait Endpoint {
    /// The endpoint base path, e.g. the `reports/traffic` part of `https://telraam-api.net/v1/reports/traffic`
    const PATH: &'static str;
    /// Method used for this endpoint, `GET` or `POST`
    const METHOD: Method;

    /// The response expected from the API, this will be deserialized from the JSON response data
    type Response: Response;
    /// If a `POST` request, this is the associated payload to be sent
    type Request: Serialize;

    /// Payload should only be associated for `POST` requests
    fn payload(&self) -> Option<&Self::Request> {
        None
    }

    /// Parameters to add to the request
    fn params(&self) -> HashMap<String, Option<String>> {
        HashMap::new()
    }

    /// Path params additional parameters to add to the path
    fn path_params(&self) -> Option<&str> {
        None
    }
}

/// This is a simple GET call that can be used to check if the Telraam API is alive and well.
#[derive(Debug)]
#[cfg_attr(feature = "clap", derive(Parser))]
pub struct Welcome;

impl Endpoint for Welcome {
    const PATH: &'static str = "";
    const METHOD: Method = Method::GET;

    type Response = WelcomeResponse;
    type Request = ();
}

/// This HTTP POST request method can be used to retrieve the observed traffic statistics for a given segment for a given time interval (maximum 3 months at a time). Parameters for the API call can be provided in the body portion of the call, see [`TrafficRequest`].
#[derive(Debug)]
#[cfg_attr(feature = "clap", derive(Parser))]
pub struct Traffic {
    #[cfg_attr(feature = "clap", command(flatten))]
    request: TrafficRequest,
}

impl Endpoint for Traffic {
    const PATH: &'static str = "reports/traffic";
    const METHOD: Method = Method::POST;

    type Response = TrafficResponse;
    type Request = TrafficRequest;

    fn payload(&self) -> Option<&Self::Request> {
        Some(&self.request)
    }
}

/// Request for observed traffic, see [`Traffic`]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "clap", derive(Args))]
pub struct TrafficRequest {
    /// the main use case is "segments" ("instance" is another option), denoting that the statistics are calculated on segment (and not individual camera, a.k.a. "instance") level
    pub level: TrafficLevel,
    /// can only be "hourly", resulting in hourly aggregated traffic
    pub format: String,
    /// the segment (or instance) identifier in question (can be found in the address of the segment from the Telraam website, e.g.: https://telraam.net/nl/location/348917)
    pub id: String,
    /// The beginning of the requested time interval (UTC)
    #[serde(serialize_with = "format_rfc3339_millis")]
    #[cfg_attr(feature = "clap", arg(value_parser = humantime::parse_rfc3339_weak))]
    pub time_start: SystemTime,
    /// The end of the requested time interval (UTC, note: the time interval is closed-open, so the end time is not included anymore in the request)
    #[serde(serialize_with = "format_rfc3339_millis")]
    #[cfg_attr(feature = "clap", arg(value_parser = humantime::parse_rfc3339_weak))]
    pub time_end: SystemTime,
}

fn format_rfc3339_millis<S: Serializer>(
    time: &SystemTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let time = humantime::format_rfc3339_millis(*time);
    serializer.serialize_str(&time.to_string())
}

/// How detailed the calculations should be
#[derive(Clone, Debug, Default, Serialize)]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum TrafficLevel {
    /// denoting that the statistics are calculated on segment
    #[default]
    #[serde(rename = "segments")]
    Segments,
    /// denoting that the statistics are calculated on an individual camera
    #[serde(rename = "instance")]
    Instance,
}

/// This HTTP GET call is the live version of the traffic snapshot API (see documentation there). The returned GeoJSON is compiled and cached on our servers every 5 minutes, meaning that this API performs much faster than the original live option under the traffic snapshot API.
#[derive(Debug)]
#[cfg_attr(feature = "clap", derive(Parser))]
pub struct LiveTrafficSnapshot;

impl Endpoint for LiveTrafficSnapshot {
    const PATH: &'static str = "reports/traffic_snapshot_live";
    const METHOD: Method = Method::GET;

    type Response = TrafficSnapshotResponse;
    type Request = ();
}

/// This HTTP GET request method is meant to retrieve all available camera instances from the server. An instance is defined by the mac_id of the connected camera, the user_id of its owner, the segment_id of its location, and its direction relative to the road segment (this translates to the left or right side of the road), if any of these parameters changes - because for example the camera is being moved to another road segment -, then there will be a new instance created for this new situation, and the old instance will be closed by adding a time_end value to it.
#[derive(Debug)]
#[cfg_attr(feature = "clap", derive(Parser))]
pub struct AllAvailableCameras;

impl Endpoint for AllAvailableCameras {
    const PATH: &'static str = "cameras";
    const METHOD: Method = Method::GET;

    type Response = CamerasResponse;
    type Request = ();
}

/// This HTTP GET request method retrieves all camera instances from the server that are associated with the given segment_id identifier. The returned parameters are the same as in the all available cameras API. Some identifiers will return a single entry, but some will have multiple entries, for example when there are multiple cameras on the same segment, or in case some property of the camera was changed resulting in a new instance, e.g., the camera was replaced (new mac_id), the direction of the camera was changed, etc In the latter case both archive and active instances will be returned.
#[derive(Debug)]
#[cfg_attr(feature = "clap", derive(Parser))]
pub struct CamerasBySegmentId {
    /// Id that represents the segment, (can be found in the address of the segment from the Telraam website, e.g.: https://telraam.net/nl/location/348917)
    pub segment_id: String,
}

impl Endpoint for CamerasBySegmentId {
    const PATH: &'static str = "cameras/segment";
    const METHOD: Method = Method::GET;

    type Response = CamerasResponse;
    type Request = ();

    fn path_params(&self) -> Option<&str> {
        Some(&self.segment_id)
    }
}

/// This HTTP GET request method retrieves all camera instances from the server that are associated with the given mac_id identifier. The returned structure is the same as in the cameras by segment id call.
#[derive(Debug)]
#[cfg_attr(feature = "clap", derive(Parser))]
pub struct CameraByMacId {
    /// MAC id from the individual device, returned in other API requests, like [`CamerasBySegmentId`]
    mac_id: String,
}

impl Endpoint for CameraByMacId {
    const PATH: &'static str = "cameras";
    const METHOD: Method = Method::GET;

    type Response = CamerasResponse;
    type Request = ();

    fn path_params(&self) -> Option<&str> {
        Some(&self.mac_id)
    }
}

/// This HTTP GET request method is used to retrieve all road segments from the server in GeoJSON format (coordinate pair strings). The retreived data has the same structure as in the active segments API, except that only the oidn property is returned for each coordinate list, this identifier is used as segment_id in some other API calls. Also, the returned coordinates are in EPSGS 31370 format.
#[derive(Debug)]
#[cfg_attr(feature = "clap", derive(Parser))]
pub struct AllSegments;

impl Endpoint for AllSegments {
    const PATH: &'static str = "segments/all";
    const METHOD: Method = Method::GET;

    type Response = SegmentResponse;
    type Request = ();
}

/// This HTTP GET request method is used to retrieve a single segments from the server in GeoJSON format.
#[derive(Debug)]
#[cfg_attr(feature = "clap", derive(Parser))]
pub struct SegmentById {
    /// Id that represents the segment, (can be found in the address of the segment from the Telraam website, e.g.: https://telraam.net/nl/location/348917)
    segment_id: String,
}

impl Endpoint for SegmentById {
    const PATH: &'static str = "segments/id";
    const METHOD: Method = Method::GET;

    type Response = SegmentResponse;
    type Request = ();

    fn path_params(&self) -> Option<&str> {
        Some(&self.segment_id)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[test]
    fn test_serialize_traffic() {
        let request = TrafficRequest {
            level: TrafficLevel::Segments,
            format: String::from("per-hour"),
            id: "348917".to_string(),
            time_start: humantime::parse_rfc3339_weak("2020-10-30 07:00:00Z").unwrap(),
            time_end: humantime::parse_rfc3339_weak("2020-10-30 09:00:00Z").unwrap(),
        };
        let json = serde_json::to_string_pretty(&request).expect("failed to serialize");
        let parsed = serde_json::from_str::<Value>(&json).expect("failed to parse");

        assert_eq!("segments", parsed["level"]);
        assert_eq!("per-hour", parsed["format"]);
        assert_eq!("348917", parsed["id"]);
        // hopefully this formatting isn't a problem, notice the spaces in the original
        assert_eq!("2020-10-30T07:00:00.000Z", parsed["time_start"]);
        assert_eq!("2020-10-30T09:00:00.000Z", parsed["time_end"]);
    }
}
