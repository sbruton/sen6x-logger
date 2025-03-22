use std::path::PathBuf;

use sen6x::MeasuredSample;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::mpsc::{Sender, channel};

use crate::error::Error;
use crate::output::{OutputFormat, format_csv, format_csv_header, format_json};

pub(super) async fn spawn_writer(
    output_format: OutputFormat,
    file_path: PathBuf,
) -> Result<Sender<MeasuredSample>, Error> {
    let (measured_sample_tx, mut measured_sample_rx) = channel::<MeasuredSample>(100);

    let file_exists = match tokio::fs::metadata(&file_path).await {
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => false,
        _ => true,
    };

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .await
        .map_err(Error::FileCreate)?;
    let mut file_writer = BufWriter::new(file);

    let output_formatter = match output_format {
        OutputFormat::Csv => {
            if !file_exists {
                let header = format_csv_header()?;
                file_writer
                    .write_all(&header)
                    .await
                    .map_err(Error::FileWrite)?;
                file_writer.flush().await.map_err(Error::FileWrite)?;
            }
            format_csv
        }
        OutputFormat::Json => format_json,
    };

    tokio::spawn(async move {
        while let Some(measured_sample) = measured_sample_rx.recv().await {
            let output = match output_formatter(&measured_sample) {
                Ok(output) => output,
                Err(err) => {
                    eprintln!("Error formatting output: {}", err);
                    continue;
                }
            };

            if let Err(err) = file_writer.write_all(&output).await {
                eprintln!("Error writing to file: {}", err);
            };

            if let Err(err) = file_writer.write(&[b'\n']).await {
                eprintln!("Error writing to file: {}", err);
            };

            if let Err(err) = file_writer.flush().await.map_err(Error::FileWrite) {
                eprintln!("Error flushing file writer: {}", err);
            };
        }
    });

    Ok(measured_sample_tx)
}
