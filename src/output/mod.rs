use sen6x::MeasuredSample;
use strum::EnumString;
use tokio::sync::mpsc::{Sender, channel};

use crate::error::Error;

mod file;
mod mqtt;
mod stdout;

#[derive(Clone, Copy, EnumString)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum OutputFormat {
    Csv,
    Json,
}

pub(super) async fn spawn_writers(
    args: crate::args::Args,
) -> Result<Sender<MeasuredSample>, Error> {
    let (measured_sample_tx, mut measured_sample_rx) = channel::<MeasuredSample>(100);

    let mut output_writers = vec![];

    if let Some(output_format) = args.stdout_output_format {
        output_writers.push(stdout::spawn_writer(output_format).await?);
    }

    if let (Some(output_format), Some(broker), Some(topic)) =
        (args.mqtt_output_format, args.mqtt_broker, args.mqtt_topic)
    {
        output_writers
            .push(mqtt::spawn_writer(output_format, broker, topic, args.mqtt_retain).await?);
    }

    if let (Some(output_format), Some(file_path)) = (args.file_output_format, args.file_output_path)
    {
        output_writers.push(file::spawn_writer(output_format, file_path).await?);
    }

    tokio::spawn(async move {
        while let Some(measured_sample) = measured_sample_rx.recv().await {
            for output_writer_tx in &output_writers {
                if let Err(err) = output_writer_tx.send(measured_sample.clone()).await {
                    eprintln!("Error dispatching sample to output writer: {}", err);
                }
            }
        }
    });

    Ok(measured_sample_tx)
}

fn format_csv_header() -> Result<Vec<u8>, Error> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(vec![]);
    wtr.serialize(&MeasuredSample {
        pm1: 0.0,
        pm2_5: 0.0,
        pm4: 0.0,
        pm10: 0.0,
        humidity: 0.0,
        temperature: 0.0,
        co2: 0,
        voc: 0.0,
        nox: 0.0,
    })
    .map_err(Error::CsvSerialization)?;
    wtr.flush().map_err(Error::CsvWrite)?;
    let buf = wtr.into_inner().map_err(Error::CsvWriteFinish)?;
    let buf = String::from_utf8_lossy(&buf);
    let mut buf = buf
        .split('\n')
        .next()
        .ok_or(Error::CsvHeaderParse)?
        .as_bytes()
        .to_vec();
    buf.push(b'\n');
    Ok(buf)
}

fn format_csv(measured_sample: &MeasuredSample) -> Result<Vec<u8>, Error> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(vec![]);
    wtr.serialize(measured_sample)
        .map_err(Error::CsvSerialization)?;
    wtr.flush().map_err(Error::CsvWrite)?;
    let mut buf = wtr.into_inner().map_err(Error::CsvWriteFinish)?;
    if buf.last() == Some(&b'\n') {
        buf.pop();
    }
    Ok(buf)
}

fn format_json(measured_sample: &MeasuredSample) -> Result<Vec<u8>, Error> {
    let output = serde_json::to_vec(measured_sample).map_err(Error::JsonSerialization)?;
    Ok(output)
}
