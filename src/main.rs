use anyhow::anyhow;
use duplicate::duplicate_item;
use serde::{Deserialize, Serialize};
use clap::{Parser, ValueEnum};
use derive_more::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Display)]
pub enum Language {
    /// XML text info in french
    #[display(fmt="f")]
    French,
    /// XML text info in english
    #[display(fmt="e")]
    English,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Display)]
pub enum Province {
    AB,
    BC,
    HEF,
    MB,
    NB,
    NL,
    NS,
    NT,
    NU,
    ON,
    PE,
    QC,
    SK,
    YT,
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// The full URL for the XML data (only needed if they change the site layout)
    #[arg(short, long)]
    pub url: Option<String>,

    /// Only display the temperature
    #[arg(short, long, default_value_t = false, conflicts_with="relative_humidity_only")]
    pub temperature_only: bool,

    /// Only display the humidity
    #[arg(short, long, default_value_t = false, conflicts_with="temperature_only")]
    pub relative_humidity_only: bool,

    /// In which province is the weather station ?
    #[arg(short, long, default_value = "qc", conflicts_with="url")]
    pub province: Province,

    /// The ID of the weather station
    /// see https://dd.weather.gc.ca/citypage_weather/xml/siteList.xml
    #[arg(short, long, default_value = "s0000635", conflicts_with="url")]
    pub station_id: String,

    /// The language of text info in the XML data (currently only affects text summary of date)
    #[arg(value_enum, short, long, default_value = "english", conflicts_with="url")]
    pub language: Language,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct WeatherData {
    #[serde(rename = "currentConditions")]
    current_conditions: CurrentConditions,
}

#[derive(Debug, Serialize, Deserialize)]
struct CurrentConditions {
    temperature: Option<Temperature>,
    dewpoint: Option<Dewpoint>,
    humidex: Option<Humidex>,
    pressure: Option<Pressure>,
    visibility: Option<Visibility>,
    #[serde(rename = "windChill")]
    wind_chill: Option<WindChill>,
    #[serde(rename = "relativeHumidity")]
    relative_humidity: Option<RelativeHumidity>,
    #[serde(rename = "dateTime")]
    date_time: Vec<DateTime>, 
}

#[duplicate_item(Measurement; [Temperature]; [Dewpoint]; [Humidex]; [Pressure]; [Visibility]; [RelativeHumidity]; [WindChill])]
#[derive(Debug, Serialize, Deserialize)]
struct Measurement {
    #[serde(rename(deserialize = "$value"))]
    value: Option<f64>,
    #[serde(rename = "unitType")]
    unit_type: Option<String>,
    units: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DateTime {
    // attributes
    #[serde(rename = "UTCOffset")]
    utc_offset: i8,
    zone: String,
    // tags
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    #[serde(rename = "textSummary")]
    text_summary: String,
}

fn get_weather(args: &Args) -> Result<WeatherData,anyhow::Error> {
    let api_url = "https://dd.weather.gc.ca/citypage_weather/xml";
    let url;
    if let Some(full_url) = args.url.clone() {
        url = full_url;
    } else {
        url = format!("{}/{}/{}_{}.xml", api_url, args.province, args.station_id, args.language);
    }

    let response = reqwest::blocking::get(&url)?;
    if response.status().is_success() {
        let xml = response.text()?;
        let weather_data = serde_xml_rs::from_str::<WeatherData>(&xml)?;
        Ok(weather_data)
    } else {
        Err(anyhow!("Failed to fetch weather data for {} with status: {}",url,response.status()))
    }
}

fn display_weather(args: &Args) -> Result<(),anyhow::Error> {
    let weather_data = get_weather(&args)?;
    if args.temperature_only {
        if let Some(temperature) = weather_data.current_conditions.temperature {
            if let Some(temperature_value) = temperature.value {
                println!("{}",temperature_value);
                return Ok(());
            }
        }
        Err(anyhow!("Temperature not available"))
    } else if args.relative_humidity_only {
        if let Some(relative_humidity) = weather_data.current_conditions.relative_humidity {
            if let Some(relative_humidity_value) = relative_humidity.value {
                println!("{}",relative_humidity_value);
                return Ok(());
            }
        }
        Err(anyhow!("Relative humidity not available"))
    } else {
        let json = serde_json::to_string(&weather_data.current_conditions)?;
        println!("{}",json);
        Ok(())
    }
}

fn main() {
    let args = Args::new();
    if let Err(e) = display_weather(&args) {
        println!("{{\"error\":\"{}\"}}",e);
        std::process::exit(1);
    }
}