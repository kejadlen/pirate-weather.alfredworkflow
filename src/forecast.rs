use std::fmt;
use std::fmt::Display;
use std::result;

use chrono::prelude::*;
use chrono::serde::ts_seconds;
use serde::Deserialize;

use crate::precipitation::{Intensity, Probability};

#[derive(Clone, Debug, Deserialize)]
pub struct Temperature(f64);

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}°", self.0.round())
    }
}

#[derive(Debug)]
pub enum Units {
    Auto,
    Ca,
    Uk2,
    Us,
    Si,
}

impl Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Units::Auto => "us",
            Units::Ca => "ca",
            Units::Uk2 => "uk",
            Units::Us => "us",
            Units::Si => "si",
        })
    }
}

#[derive(Debug)]
pub enum Lang {
    Arabic,
    Azerbaijani,
    Belarusian,
    Bulgarian,
    Bengali,
    Bosnian,
    Catalan,
    Czech,
    Danish,
    German,
    Greek,
    English,
    Esperanto,
    Spanish,
    Estonian,
    Finnish,
    French,
    Hebrew,
    Hindi,
    Croatian,
    Hungarian,
    Indonesian,
    Icelandic,
    Italian,
    Japanese,
    Georgian,
    Kannada,
    Korean,
    Cornish,
    Latvian,
    Malayam,
    Marathi,
    NorwegianBokmal,
    Dutch,
    Punjabi,
    Polish,
    Portuguese,
    Romanian,
    Russian,
    Slovak,
    Slovenian,
    Serbian,
    Swedish,
    Tamil,
    Telugu,
    Tetum,
    Turkish,
    Ukrainian,
    Urdu,
    IgpayAtinlay,
    SimplifiedChinese,
    TraditionalChinese,
}

#[derive(Clone, Debug)]
pub enum Icon {
    Clear,
    ClearDay,
    ClearNight,
    Rain,
    Snow,
    Sleet,
    Wind,
    Fog,
    Cloudy,
    PartlyCloudyDay,
    PartlyCloudyNight,
    Unknown(String),
}

impl<'de> Deserialize<'de> for Icon {
    fn deserialize<D>(deserializer: D) -> result::Result<Icon, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "clear" => Icon::Clear,
            "clear-day" => Icon::ClearDay,
            "clear-night" => Icon::ClearNight,
            "rain" => Icon::Rain,
            "snow" => Icon::Snow,
            "sleet" => Icon::Sleet,
            "wind" => Icon::Wind,
            "fog" => Icon::Fog,
            "cloudy" => Icon::Cloudy,
            "partly-cloudy-day" => Icon::PartlyCloudyDay,
            "partly-cloudy-night" => Icon::PartlyCloudyNight,
            s => Icon::Unknown(s.into()),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Forecast {
    pub currently: Option<Point>,
    pub minutely: Option<Block>,
    pub daily: Option<Block>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    #[serde(rename = "temperature")]
    pub temp: Option<f64>,
    pub apparent_temperature: Option<Temperature>,
    pub apparent_temperature_min: Option<Temperature>,
    pub apparent_temperature_max: Option<Temperature>,
    pub icon: Option<Icon>,
    pub precip_intensity: Option<Intensity>,
    pub precip_probability: Option<Probability>,
    pub summary: Option<String>,
    #[serde(with = "ts_seconds")]
    pub time: DateTime<Utc>,
}

impl Point {
    pub fn human_precipitation(&self) -> Option<String> {
        let intensity = self.precip_intensity.clone()?;
        let probability = self.precip_probability.clone()?;
        if probability.0 > 0. {
            Some(format!(
                "{} chance of {} rain.",
                probability,
                intensity.humanized()
            ))
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub data: Vec<Point>,
    pub summary: Option<String>,
    pub icon: Option<Icon>,
}

impl Block {
    // These aren't quite right since I'm just dropping precipitation values that aren't there...

    pub fn precip_intensities(&self) -> Vec<Intensity> {
        self.data
            .iter()
            .flat_map(|x| x.precip_intensity.clone())
            .collect()
    }

    pub fn precip_probabilities(&self) -> Vec<Probability> {
        self.data
            .iter()
            .flat_map(|x| x.precip_probability.clone())
            .collect()
    }
}
