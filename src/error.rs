use clapper::ClapperError;
use linux_embedded_hal::i2cdev::linux::LinuxI2CError;
use sen6x::{MeasuredSample, Sen6xError};
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

#[derive(Debug, Error)]
pub(super) enum Error {
    #[error("Failed to start continuous measurement: {0}")]
    ContinuousMeasurement(#[source] Sen6xError),
    #[error("Failed to serialize CSV: {0}")]
    CsvSerialization(#[source] csv::Error),
    #[error("Failed to write CSV: {0}")]
    CsvWrite(#[source] std::io::Error),
    #[error("Failed to finish writing CSV: {0}")]
    CsvWriteFinish(#[source] csv::IntoInnerError<csv::Writer<Vec<u8>>>),
    #[error("Failed to parse CSV header")]
    CsvHeaderParse,
    #[error("I2C bus error: {0}")]
    I2cBus(#[source] LinuxI2CError),
    #[error("Failed to serialize JSON: {0}")]
    JsonSerialization(#[source] serde_json::Error),
    #[error("Failed to read sample: {0}")]
    ReadSample(#[source] Sen6xError),
    #[error("Failed to write to stdout: {0}")]
    StdoutWrite(#[source] std::io::Error),
    #[error("Failed to stop measurement: {0}")]
    StopMeasurement(#[source] Sen6xError),
    #[error("Failed to write to file: {0}")]
    FileCreate(#[source] std::io::Error),
    #[error("Failed to create file writer: {0}")]
    FileWrite(#[source] std::io::Error),
    #[error("Failed to dispatch sample to output writers: {0}")]
    SampleDispatch(#[source] SendError<MeasuredSample>),
}

impl ClapperError for Error {
    fn exit_code(&self) -> i32 {
        match self {
            Error::ContinuousMeasurement(_) => exitcode::IOERR,
            Error::CsvSerialization(_) => exitcode::SOFTWARE,
            Error::CsvWrite(_) => exitcode::IOERR,
            Error::CsvWriteFinish(_) => exitcode::SOFTWARE,
            Error::CsvHeaderParse => exitcode::SOFTWARE,
            Error::I2cBus(_) => exitcode::IOERR,
            Error::JsonSerialization(_) => exitcode::SOFTWARE,
            Error::ReadSample(_) => exitcode::IOERR,
            Error::StdoutWrite(_) => exitcode::IOERR,
            Error::StopMeasurement(_) => exitcode::IOERR,
            Error::FileCreate(_) => exitcode::IOERR,
            Error::FileWrite(_) => exitcode::IOERR,
            Error::SampleDispatch(_) => exitcode::SOFTWARE,
        }
    }
}
