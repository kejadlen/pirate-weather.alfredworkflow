# Pirate Weather Workflow for Alfred

![screenshot][screenshot]

[screenshot]: http://i.imgur.com/lbA9fPW.png

Support [Pirate Weather](https://weather.pirateweather.net/):

- https://www.buymeacoffee.com/pirateweather
- https://github.com/sponsors/alexander0042

## Installation

Download and install the [workflow][download].

[download]: https://github.com/kejadlen/pirate-weather.alfredworkflow/releases/download/v4.0.0/pirate-weather.alfredworkflow

These environment variables can be [configured in Alfred][env-vars]:

- `PIRATE_WEATHER_API_KEY`: Get an API key [here][pirate-weather-api-key].
- `PIRATE_WEATHER_ENDPOINT`: Set to `api.darksky.net` to use Dark Sky (until
  the API is deprecated).
- `GOOGLE_API_KEY`: Get an API key [here][google-api-key]. (Used for geocoding
  queries. *This can be omitted if you only want the forecast for the current
  location*.)
- `FORECAST_UNITS`: Defaults to `auto`, which sets the units based on the
  location. Use `si` for Celsius and `us` for Fahrenheit.
- `FORECAST_LANG`: Defaults to `en`. See [Dark Sky
  documentation][dark-sky-lang] for full list of language options.
- `DEFAULT_LAT_LONG`: Set this to override IP geolocation. Ex:
  `47.7396,-122.3426` for Seattle.
- `DEFAULT_LOCATION`: Used for displaying the location name when using
  `DEFAULT_LAT_LONG`.
- `LIGHT_ICONS`: `true` gives white icons, `false` gives black icons.

[env-vars]: https://www.alfredapp.com/help/workflows/advanced/variables/
[pirate-weather-api-key]: https://pirateweather.net/getting-started
[google-api-key]: https://developers.google.com/maps/documentation/geocoding/#api_key
[dark-sky-lang]: https://darksky.net/dev/docs#forecast-request

# Attributions

- [Climacons](http://adamwhitcroft.com/climacons/)
- [Pirate Weather API](https://pirateweather.net/)
- [Google Geocoding API](https://developers.google.com/maps/documentation/geocoding/)
- [ipinfo.io](http://ipinfo.io/)
