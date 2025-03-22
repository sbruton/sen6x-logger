use std::sync::atomic::Ordering;
use std::time::Duration;

use clapper::ClapperResult;
use indicatif::{ProgressBar, ProgressStyle};
use linux_embedded_hal::I2cdev;
use sen6x::blocking::Sen6x;
use tokio::time::sleep;

mod args;
mod error;
mod output;

use args::Args;
use error::Error;

#[clapper::main]
async fn main(args: Args, terminated: Arc<AtomicBool>) -> ClapperResult<Error> {
    let mut delay = linux_embedded_hal::Delay;

    let i2c = I2cdev::new(&args.i2c_bus).map_err(Error::I2cBus)?;

    let mut sen6x = Sen6x::new(&mut delay, i2c);

    sen6x
        .start_continuous_measurement()
        .map_err(Error::ContinuousMeasurement)?;

    let bar = ProgressBar::new(args.stabilization_secs)
        .with_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} {wide_msg} {bar:50.cyan/blue} {eta}")
                .unwrap(),
        )
        .with_message("Waiting for sensor readings to stabilize...");

    for _ in 0..args.stabilization_secs {
        if terminated.load(Ordering::Relaxed) {
            break;
        }
        sleep(Duration::from_secs(1)).await;
        bar.inc(1);
    }

    let measurement_secs = args.measurement_secs;

    let measured_sample_tx = output::spawn_writers(args).await?;

    loop {
        if terminated.load(Ordering::Relaxed) {
            break;
        }

        let measured_sample = sen6x.get_sample().map_err(Error::ReadSample)?;

        measured_sample_tx
            .send(measured_sample)
            .await
            .map_err(Error::SampleDispatch)?;

        sleep(Duration::from_secs(measurement_secs)).await;
    }

    sen6x.stop_measurement().map_err(Error::StopMeasurement)?;

    Ok(())
}
