use clap::{Parser, Subcommand};

use telraam::{client::TelraamClient, endpoint, response::Response};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 't', env = "TELRAAM_TOKEN", hide_env_values = true)]
    telraam_token: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Welcome(endpoint::Welcome),
    Traffic(endpoint::Traffic),
    LiveTrafficSnapshot(endpoint::LiveTrafficSnapshot),
    AllAvailableCameras(endpoint::AllAvailableCameras),
    CamerasBySegmentId(endpoint::CamerasBySegmentId),
    CameraByMacId(endpoint::CameraByMacId),
    AllSegments(endpoint::AllSegments),
    SegmentById(endpoint::SegmentById),
}

fn welcome(
    client: &TelraamClient,
    request: &endpoint::Welcome,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.send(request)?;
    println!("msg = {}", response.status().message);
    Ok(())
}

fn traffic(
    client: &TelraamClient,
    request: &endpoint::Traffic,
) -> Result<(), Box<dyn std::error::Error>> {
    let reports = client.send(request)?.take_reports()?;
    println!("{}", serde_json::to_string_pretty(&reports)?);
    Ok(())
}

fn live_traffic_snapshot(
    client: &TelraamClient,
    request: &endpoint::LiveTrafficSnapshot,
) -> Result<(), Box<dyn std::error::Error>> {
    let snapshot = client.send(request)?.take_snapshot()?;
    println!("{}", serde_json::to_string_pretty(&snapshot)?);
    Ok(())
}

fn all_available_cameras(
    client: &TelraamClient,
    request: &endpoint::AllAvailableCameras,
) -> Result<(), Box<dyn std::error::Error>> {
    let cameras = client.send(request)?.take_cameras()?;
    println!("{}", serde_json::to_string_pretty(&cameras)?);
    Ok(())
}

fn cameras_by_segmant_id(
    client: &TelraamClient,
    request: &endpoint::CamerasBySegmentId,
) -> Result<(), Box<dyn std::error::Error>> {
    let cameras = client.send(request)?.take_cameras()?;
    println!("{}", serde_json::to_string_pretty(&cameras)?);
    Ok(())
}

fn camera_by_mac_id(
    client: &TelraamClient,
    request: &endpoint::CameraByMacId,
) -> Result<(), Box<dyn std::error::Error>> {
    let cameras = client.send(request)?.take_cameras()?;
    println!("{}", serde_json::to_string_pretty(&cameras)?);
    Ok(())
}

fn all_segments(
    client: &TelraamClient,
    request: &endpoint::AllSegments,
) -> Result<(), Box<dyn std::error::Error>> {
    let segments = client.send(request)?.take_segments()?;
    println!("{}", serde_json::to_string_pretty(&segments)?);
    Ok(())
}

fn segment_by_id(
    client: &TelraamClient,
    request: &endpoint::SegmentById,
) -> Result<(), Box<dyn std::error::Error>> {
    let segments = client.send(request)?.take_segments()?;
    println!("{}", serde_json::to_string_pretty(&segments)?);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let api_token = &args.telraam_token as &str;
    let client = TelraamClient::new(api_token)?;

    match &args.command {
        Commands::Welcome(welcome_req) => welcome(&client, welcome_req)?,
        Commands::Traffic(traffic_req) => traffic(&client, traffic_req)?,
        Commands::LiveTrafficSnapshot(traffic_req) => live_traffic_snapshot(&client, traffic_req)?,
        Commands::AllAvailableCameras(cameras_req) => all_available_cameras(&client, cameras_req)?,
        Commands::CamerasBySegmentId(cameras_req) => cameras_by_segmant_id(&client, cameras_req)?,
        Commands::CameraByMacId(cameras_req) => camera_by_mac_id(&client, cameras_req)?,
        Commands::AllSegments(segments_req) => all_segments(&client, segments_req)?,
        Commands::SegmentById(segment_req) => segment_by_id(&client, segment_req)?,
    }

    Ok(())
}
