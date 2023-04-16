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
        let mut args = Self::parse();
        let api_url = "https://dd.weather.gc.ca/citypage_weather/xml";
        if args.url.is_none() {
            args.url = Some(format!("{}/{}/{}_{}.xml", api_url, args.province, args.station_id, args.language));

        }
        args
    }
}
