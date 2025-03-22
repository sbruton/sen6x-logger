use std::io::{Write, stdout};

use sen6x::MeasuredSample;
use tokio::sync::mpsc::{Sender, channel};

use crate::error::Error;
use crate::output::{OutputFormat, format_csv, format_csv_header, format_json};

pub(super) async fn spawn_writer(
    output_format: OutputFormat,
) -> Result<Sender<MeasuredSample>, Error> {
    let (measured_sample_tx, mut measured_sample_rx) = channel::<MeasuredSample>(100);

    let output_formatter = match output_format {
        OutputFormat::Csv => {
            let header = format_csv_header()?;
            stdout().write_all(&header).map_err(Error::StdoutWrite)?;
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

            if let Err(err) = stdout().write_all(&output) {
                eprintln!("Error writing to stdout: {}", err);
            }

            if let Err(err) = stdout().write(&[b'\n']) {
                eprintln!("Error writing to stdout: {}", err);
            };

            if let Err(err) = stdout().flush() {
                eprintln!("Error flushing stdout: {}", err);
            }
        }
    });

    Ok(measured_sample_tx)
}
