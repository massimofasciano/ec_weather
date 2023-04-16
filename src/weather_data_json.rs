use anyhow::anyhow;
// use chrono::TimeZone;
use serde_json::json;

pub struct WeatherData {
    json_value : serde_json::value::Value,
}

impl WeatherData {
    fn convert_json_value(json_value: &mut serde_json::value::Value) {
        match json_value {
            serde_json::value::Value::Object(map) => {
                if let Some(json_value) = map.remove("$value") {
                    map.insert("value".to_string(), json_value);
                }
                for json_value in map.values_mut() {
                    Self::convert_json_value(json_value);
                }
            },
            serde_json::value::Value::Array(vec) => {
                for json_value in vec.iter_mut() {
                    Self::convert_json_value(json_value);
                }
            },
            serde_json::value::Value::String(txt) => {
                if let Ok(flt) = txt.parse::<f64>() {
                    *json_value = json!(flt);
                }
            },
            _ => {}
        }
    }
    pub fn new(url: &str) -> Result<WeatherData,anyhow::Error> {
        let response = reqwest::blocking::get(url)?;
        if response.status().is_success() {
            let xml = response.text()?;
            let mut weather_data = WeatherData { 
                json_value : serde_xml_rs::from_str::<serde_json::value::Value>(&xml)? 
            };
            Self::convert_json_value(&mut weather_data.json_value);
            Ok(weather_data)
        } else {
            Err(anyhow!("Failed to fetch weather data for {} with status: {}",url,response.status()))
        }
    }
    pub fn get_temperature(&self) -> Result<&serde_json::value::Value,anyhow::Error> {
        if let Some(temperature) = self.get_current_conditions()?.get("temperature") {
            if let Some(temperature_value) = temperature.get("value") {
                return Ok(temperature_value);
            }
        }
        Err(anyhow!("Temperature not available"))
    }
    pub fn get_relative_humidity(&self) -> Result<&serde_json::value::Value,anyhow::Error> {
        if let Some(relative_humidity) = self.get_current_conditions()?.get("relativeHumidity") {
            if let Some(relative_humidity_value) = relative_humidity.get("value") {
                return Ok(relative_humidity_value);
            }
        }
        Err(anyhow!("Relative Humidity not available"))
    }
    fn get_current_conditions(&self) -> Result<&serde_json::value::Value,anyhow::Error> {
        self.json_value.get("currentConditions")
            .ok_or(anyhow!("no field named currentConditions"))
    }
    pub fn to_json(&self) -> Result<String,anyhow::Error> {
        Ok(serde_json::to_string(self.get_current_conditions()?)?)
    }
}
