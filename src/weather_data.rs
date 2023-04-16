use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use derive_more::Display;
use chrono::TimeZone;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    #[serde(rename = "currentConditions")]
    pub current_conditions: CurrentConditions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentConditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dewpoint: Option<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humidex: Option<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressure: Option<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Measurement>,
    #[serde(rename = "windChill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_chill: Option<Measurement>,
    #[serde(rename = "relativeHumidity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_humidity: Option<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind: Option<Wind>,
    #[serde(rename = "dateTime")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub date_time_xml: Vec<DateTimeXML>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wind {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gust: Option<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearing: Option<Measurement>,
}

#[derive(Debug, Deserialize, Display)]
#[serde(untagged)]
pub enum Stringf64 {
    Number(f64),
    Text(String),
}

// Custom serializer to deal with strings in the XML where we expect floats.
// We deserialize as custom Stringf64 and then convert to f64 when
// serializing to JSON.
impl Serialize for Stringf64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use Stringf64::*;
        match self {
            Text(txt) => {
                if let Ok(flt) = txt.parse::<f64>() {
                    serializer.serialize_f64(flt)
                } else {
                    serializer.serialize_str(txt)
                }
            }
            Number(flt) => {
                serializer.serialize_f64(*flt)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurement {
    #[serde(rename(deserialize = "$value"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Stringf64>,
    #[serde(rename = "unitType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateTimeXML {
    // attributes
    #[serde(rename = "UTCOffset")]
    pub utc_offset: i32,
    pub zone: String,
    // tags
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    #[serde(rename = "textSummary")]
    pub text_summary: String,
}

impl WeatherData {
    pub fn new(url: &str) -> Result<WeatherData,anyhow::Error> {
        let response = reqwest::blocking::get(url)?;
        if response.status().is_success() {
            let xml = response.text()?;
            let mut weather_data = serde_xml_rs::from_str::<WeatherData>(&xml)?;
            weather_data.current_conditions.timestamp = None;
            for d in &weather_data.current_conditions.date_time_xml {
                weather_data.current_conditions.timestamp = 
                    Some(chrono::FixedOffset::east_opt(d.utc_offset*60*60).unwrap()
                        .with_ymd_and_hms(d.year, d.month, d.day, d.hour, d.minute, 0).unwrap()
                        .with_timezone(&chrono::Utc)
                    );
            }
            weather_data.current_conditions.date_time_xml = vec![];
            Ok(weather_data)
        } else {
            Err(anyhow!("Failed to fetch weather data for {} with status: {}",url,response.status()))
        }
    }
    pub fn get_temperature(&self) -> Result<&Stringf64,anyhow::Error> {
        if let Some(temperature) = self.current_conditions.temperature.as_ref() {
            if let Some(temperature_value) = temperature.value.as_ref() {
                return Ok(temperature_value);
            }
        }
        Err(anyhow!("Temperature not available"))
    }
    pub fn get_relative_humidity(&self) -> Result<&Stringf64,anyhow::Error> {
        if let Some(relative_humidity) = self.current_conditions.relative_humidity.as_ref() {
            if let Some(relative_humidity_value) = relative_humidity.value.as_ref() {
                return Ok(relative_humidity_value);
            }
        }
        Err(anyhow!("Relative Humidity not available"))
    }
}

