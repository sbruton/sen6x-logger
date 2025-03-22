use std::{net::SocketAddr, path::PathBuf};

use clapper::prelude::*;

use crate::output::OutputFormat;

#[derive(ArgParser)]
#[command(group(
    ArgGroup::new("output")
        .args(["stdout_output_format", "mqtt_output_format", "file_output_format"])
        .multiple(true)
        .required(true)
))]
pub(super) struct Args {
    pub(super) i2c_bus: PathBuf,
    #[clap(long = "stdout")]
    pub(super) stdout_output_format: Option<OutputFormat>,
    #[clap(
        long = "mqtt-format",
        requires = "mqtt_broker",
        requires = "mqtt_topic"
    )]
    pub(super) mqtt_output_format: Option<OutputFormat>,
    #[arg(long = "mqtt-broker")]
    pub(super) mqtt_broker: Option<SocketAddr>,
    #[arg(long = "mqtt-topic")]
    pub(super) mqtt_topic: Option<String>,
    #[arg(long = "mqtt-retain", default_value_t = false, action = ArgAction::SetTrue)]
    pub(super) mqtt_retain: bool,
    #[clap(long = "file-format", requires = "file_output_path")]
    pub(super) file_output_format: Option<OutputFormat>,
    #[arg(long = "file")]
    pub(super) file_output_path: Option<PathBuf>,
    #[clap(long = "stabilization-secs", default_value_t = 60)]
    pub(super) stabilization_secs: u64,
    #[clap(short = 'i', long = "measurement-secs", default_value_t = 1)]
    pub(super) measurement_secs: u64,
}
