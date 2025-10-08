mod actions;
mod config;

use clap::Parser;
use containerd_client::tonic::Request;
use containerd_client::{
    connect,
    services::v1::{transfer_client::TransferClient, TransferOptions, TransferRequest},
    to_any,
    types::transfer::OciRegistry,
    types::transfer::{ImageStore, UnpackConfiguration},
    types::Platform,
    with_namespace,
};
use std::env::consts;

#[derive(Parser, Debug)]
struct Args {
    /// containerd namespace
    #[arg(short, long, default_value = "nocturne")]
    namespace: String,

    /// containerd socket
    #[arg(short, long, default_value = "/run/containerd/containerd.sock")]
    socket: String,
}

const IMAGE: &str = "docker.io/library/alpine:latest";

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let arch = match consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        _ => consts::ARCH,
    };

    dbg!(&args);

    let channel = connect(&args.socket).await.expect("failed to bind socket");
    let mut client = TransferClient::new(channel.clone());

    let registry = OciRegistry {
        reference: IMAGE.to_string(),
        resolver: Default::default(),
    };

    let platform = Platform {
        os: "linux".to_string(),
        architecture: arch.to_string(),
        variant: "".to_string(),
        os_version: "".to_string(),
    };

    let destination = ImageStore {
        name: IMAGE.to_string(),
        platforms: vec![platform.clone()],
        unpacks: vec![UnpackConfiguration {
            platform: Some(platform),
            ..Default::default()
        }],
        ..Default::default()
    };

    let anys = to_any(&registry);
    let anyd = to_any(&destination);

    println!(
        "Pulling image for linux/{} from source: {:?}",
        arch, registry
    );

    let request = TransferRequest {
        source: Some(anys),
        destination: Some(anyd),
        options: Some(TransferOptions {
            ..Default::default()
        }),
    };
    // let request = Request::new(request);
    let req = with_namespace!(request, args.namespace);

    client
        .transfer(req)
        .await
        .expect("unable to transfer image");
}
