use std::net::SocketAddr;
use std::time::Duration;

use rumqttc::{AsyncClient as MqttClient, MqttOptions, QoS as MqttQoS};
use sen6x::MeasuredSample;
use tokio::sync::mpsc::{Sender, channel};

use crate::error::Error;
use crate::output::{OutputFormat, format_csv, format_json};

pub(super) async fn spawn_writer(
    output_format: OutputFormat,
    broker: SocketAddr,
    topic: String,
    retain: bool,
) -> Result<Sender<MeasuredSample>, Error> {
    let (measured_sample_tx, mut measured_sample_rx) = channel::<MeasuredSample>(100);

    let output_formatter = match output_format {
        OutputFormat::Csv => format_csv,
        OutputFormat::Json => format_json,
    };

    let mut mqttoptions = MqttOptions::new(
        env!("CARGO_BIN_NAME"),
        broker.ip().to_string(),
        broker.port(),
    );

    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = MqttClient::new(mqttoptions.clone(), 100);

    tokio::spawn(async move {
        loop {
            if let Err(err) = eventloop.poll().await {
                eprintln!("MQTT connection error: {}", err);
            }
        }
    });

    tokio::spawn(async move {
        while let Some(measured_sample) = measured_sample_rx.recv().await {
            let output = match (output_formatter)(&measured_sample) {
                Ok(output) => output,
                Err(err) => {
                    eprintln!("Error formatting output: {}", err);
                    continue;
                }
            };
            if let Err(err) = client
                .publish(topic.clone(), MqttQoS::AtLeastOnce, retain, output)
                .await
            {
                eprintln!("Error publishing to MQTT: {}", err);
            };
        }
    });

    Ok(measured_sample_tx)
}
