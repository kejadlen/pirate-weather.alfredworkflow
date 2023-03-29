mod errors;
mod forecast;
mod geocode;
mod location;
mod pirate_weather;
mod precipitation;
mod sparkline;
mod theme;

use std::env;
use std::process::Command;

use color_eyre::eyre::{anyhow, bail, Result};

use theme::Theme;

fn main() -> Result<()> {
    color_eyre::install()?;

    let pirate_weather_endpoint = env::var("pirate_weather_endpoint")?;
    let pirate_weather_api_key = env::var("pirate_weather_api_key")?;
    let location = location()?;
    let theme = match env::var("theme")?.as_str() {
        "auto" => {
            let apple_interface_style = String::from_utf8(
                Command::new("defaults")
                    .arg("read")
                    .arg("-g")
                    .arg("AppleInterfaceStyle")
                    .output()?
                    .stdout,
            )?;
            if apple_interface_style.trim() == "Dark" {
                Ok(Theme::Light)
            } else {
                Ok(Theme::Dark)
            }
        }
        "light" => Ok(Theme::Dark),
        "dark" => Ok(Theme::Light),
        theme => Err(anyhow!("invalid theme: '{}'", theme)),
    }?;
    let units = env::var("forecast_units").unwrap_or_else(|_| "auto".into());
    let units = match units.as_str() {
        "auto" => forecast::Units::Auto,
        "ca" => forecast::Units::Ca,
        "uk2" => forecast::Units::Uk2,
        "us" => forecast::Units::Us,
        "si" => forecast::Units::Si,
        units => bail!("invalid forecast units: '{}'", units),
    };
    let lang = env::var("forecast_lang").unwrap_or_else(|_| "en".into());
    let lang = match lang.as_str() {
        "ar" => forecast::Lang::Arabic,
        "az" => forecast::Lang::Azerbaijani,
        "be" => forecast::Lang::Belarusian,
        "bg" => forecast::Lang::Bulgarian,
        "bn" => forecast::Lang::Bengali,
        "bs" => forecast::Lang::Bosnian,
        "ca" => forecast::Lang::Catalan,
        "cs" => forecast::Lang::Czech,
        "da" => forecast::Lang::Danish,
        "de" => forecast::Lang::German,
        "el" => forecast::Lang::Greek,
        "en" => forecast::Lang::English,
        "eo" => forecast::Lang::Esperanto,
        "es" => forecast::Lang::Spanish,
        "et" => forecast::Lang::Estonian,
        "fi" => forecast::Lang::Finnish,
        "fr" => forecast::Lang::French,
        "he" => forecast::Lang::Hebrew,
        "hi" => forecast::Lang::Hindi,
        "hr" => forecast::Lang::Croatian,
        "hu" => forecast::Lang::Hungarian,
        "id" => forecast::Lang::Indonesian,
        "is" => forecast::Lang::Icelandic,
        "it" => forecast::Lang::Italian,
        "ja" => forecast::Lang::Japanese,
        "ka" => forecast::Lang::Georgian,
        "kn" => forecast::Lang::Kannada,
        "ko" => forecast::Lang::Korean,
        "kw" => forecast::Lang::Cornish,
        "lv" => forecast::Lang::Latvian,
        "ml" => forecast::Lang::Malayam,
        "mr" => forecast::Lang::Marathi,
        "nb" => forecast::Lang::NorwegianBokmal,
        "nl" => forecast::Lang::Dutch,
        "no" => forecast::Lang::NorwegianBokmal,
        "pa" => forecast::Lang::Punjabi,
        "pl" => forecast::Lang::Polish,
        "pt" => forecast::Lang::Portuguese,
        "ro" => forecast::Lang::Romanian,
        "ru" => forecast::Lang::Russian,
        "sk" => forecast::Lang::Slovak,
        "sl" => forecast::Lang::Slovenian,
        "sr" => forecast::Lang::Serbian,
        "sv" => forecast::Lang::Swedish,
        "ta" => forecast::Lang::Tamil,
        "te" => forecast::Lang::Telugu,
        "tet" => forecast::Lang::Tetum,
        "tr" => forecast::Lang::Turkish,
        "uk" => forecast::Lang::Ukrainian,
        "ur" => forecast::Lang::Urdu,
        "x-pig-latin" => forecast::Lang::IgpayAtinlay,
        "zh" => forecast::Lang::SimplifiedChinese,
        "zh-tw" => forecast::Lang::TraditionalChinese,
        lang => bail!("invalid forecast language: '{}'", lang),
    };

    let pirate_weather = pirate_weather::PirateWeather {
        pirate_weather_endpoint,
        pirate_weather_api_key,
        location,
        theme,
        units,
        lang,
    };

    pirate_weather.run()?;

    Ok(())
}

fn location() -> Result<location::Location> {
    let args: Vec<_> = env::args().skip(1).collect();
    let query = args.join(" ");
    let location = parse_default_location()?;
    match (query, location) {
        (ref query, _) if !query.is_empty() => {
            let api_key = env::var("google_api_key")?;
            let geocoder = geocode::Geocoder::new(&api_key);
            geocoder.geocode(query)
        }
        (_, Some(ref location)) => Ok(location.clone()),
        _ => location::Location::from_ip(),
    }
}

fn parse_default_location() -> Result<Option<location::Location>> {
    let location = match env::var("default_lat_long") {
        Ok(ref lat_long) if !lat_long.is_empty() => {
            let coord = location::Coordinate::try_from(lat_long.as_str())
                .map_err(|_| anyhow!("invalid default latitude,longitude: {}", lat_long))?;
            let description = env::var("default_location").unwrap_or_else(|_| "".into());
            let location = location::Location { description, coord };
            Some(location)
        }
        _ => None,
    };
    Ok(location)
}
