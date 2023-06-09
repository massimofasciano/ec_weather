use anyhow::anyhow;

mod args;
#[cfg(not(feature="data_json"))]
mod weather_data;
#[cfg(not(feature="data_json"))]
use weather_data::WeatherData;
#[cfg(feature="data_json")]
mod weather_data_json;
#[cfg(feature="data_json")]
use weather_data_json::WeatherData;
mod error;

fn display_weather(args: &args::Args) -> Result<(),anyhow::Error> {
    let weather_data = WeatherData::new(args.url.as_ref().ok_or(anyhow!("no url in args"))?)?;
    if args.temperature_only {
        println!("{}",weather_data.get_temperature()?);
    } else if args.relative_humidity_only {
        println!("{}",weather_data.get_relative_humidity()?);
    } else {
        println!("{}",weather_data.to_json()?);
    }
    Ok(())
}



fn main() {
    let args = args::Args::new();
    if let Err(e) = display_weather(&args) {
        error::exit_error_json(&e);
    }
}