use anyhow::anyhow;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dewpoint: Option<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    humidex: Option<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pressure: Option<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<Measurement>,
    #[serde(rename = "windChill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    wind_chill: Option<Measurement>,
    #[serde(rename = "relativeHumidity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_humidity: Option<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wind: Option<Wind>,
    #[serde(rename = "dateTime")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    date_time: Vec<DateTime>, 
}

#[derive(Debug, Serialize, Deserialize)]
struct Wind {
    #[serde(skip_serializing_if = "Option::is_none")]
    speed: Option<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gust: Option<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bearing: Option<Measurement>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Measurement {
    #[serde(rename(deserialize = "$value"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<f64>,
    #[serde(rename = "unitType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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