mod args;
mod weather_data;
mod error;

fn display_weather(args: &args::Args) -> Result<(),anyhow::Error> {
    let weather_data = weather_data::WeatherData::new(args.url.as_ref().unwrap())?;
    if args.temperature_only {
        println!("{}",weather_data.get_temperature()?);
    } else if args.relative_humidity_only {
        println!("{}",weather_data.get_relative_humidity()?);
    } else {
        let json = serde_json::to_string(&weather_data.current_conditions)?;
        println!("{}",json);
    }
    Ok(())
}

fn main() {
    let args = args::Args::new();
    if let Err(e) = display_weather(&args) {
        error::exit_error_json(&e);
    }
}