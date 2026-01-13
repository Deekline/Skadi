use serde::Deserialize;
use serde::Serialize;
use serde::de;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    pub current_units: CurrentUnits,
    pub current: Current,
    pub hourly: Hourly,
    pub daily_units: DailyUnits,
    pub daily: Daily,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentUnits {
    pub time: String,
    pub interval: String,
    pub temperature_2m: String,
    pub apparent_temperature: String,
    pub relative_humidity_2m: String,
    pub precipitation: String,
    pub weather_code: String,
    pub wind_speed_10m: String,
    #[serde(deserialize_with = "de_opt_i32_from_any")]
    pub wind_direction_0m: Option<i32>,
    pub visibility: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Current {
    pub time: String,
    pub interval: i64,
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub relative_humidity_2m: i64,
    pub precipitation: f64,
    pub weather_code: i64,
    pub wind_speed_10m: f64,
    pub wind_direction_0m: Option<i32>,
    pub visibility: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub relative_humidity_2m: Vec<i64>,
    pub weather_code: Vec<i64>,
    pub precipitation: Vec<f64>,
    pub precipitation_probability: Vec<i64>,
    pub wind_speed_10m: Vec<f64>,
    pub wind_direction_10m: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyUnits {
    pub time: String,
    pub weather_code: String,
    pub temperature_2m_max: String,
    pub temperature_2m_min: String,
    pub precipitation_sum: String,
    pub precipitation_probability_max: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daily {
    pub time: Vec<String>,
    pub weather_code: Vec<i64>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub precipitation_sum: Vec<f64>,
    pub precipitation_probability_max: Vec<i64>,
}

#[derive(Deserialize, Debug)]
pub struct CurrentWeather {
    pub time: String,
    pub interval: i64,
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub relative_humidity_2m: i64,
    pub precipitation: f64,
    pub weather_code: i64,
    pub wind_speed_10m: f64,
    pub wind_direction_0m: Option<i32>,
    pub visibility: f64,
}

#[derive(Deserialize, Debug)]
pub struct HourlyWeather {
    pub time: Vec<String>,
    pub temperature: Vec<f64>,
    pub humidity: Vec<i64>,
    pub weather_code: Vec<i64>,
    pub precipitation: Vec<f64>,
    pub precipitation_probability: Vec<i64>,
    pub wind_speed: Vec<f64>,
}

#[derive(Deserialize, Debug)]
pub struct DailyWeather {
    pub date: Vec<String>,
    pub min_temp: Vec<f64>,
    pub max_temp: Vec<f64>,
    pub weather_code: Vec<i64>,
    pub precipitation_sum: Vec<f64>,
    pub precipitation_probability_max: Vec<i64>,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub current: CurrentWeather,
    pub hourly: HourlyWeather,
    pub daily: DailyWeather,
}

impl From<Current> for CurrentWeather {
    fn from(current: Current) -> Self {
        Self {
            time: current.time,
            interval: current.interval,
            temperature_2m: current.temperature_2m,
            apparent_temperature: current.apparent_temperature,
            relative_humidity_2m: current.relative_humidity_2m,
            precipitation: current.precipitation,
            weather_code: current.weather_code,
            wind_speed_10m: current.wind_speed_10m,
            wind_direction_0m: current.wind_direction_0m,
            visibility: current.visibility,
        }
    }
}

impl From<Hourly> for HourlyWeather {
    fn from(hourly: Hourly) -> Self {
        Self {
            time: hourly.time,
            temperature: hourly.temperature_2m,
            humidity: hourly.relative_humidity_2m,
            weather_code: hourly.weather_code,
            precipitation: hourly.precipitation,
            precipitation_probability: hourly.precipitation_probability,
            wind_speed: hourly.wind_speed_10m,
        }
    }
}

impl From<Daily> for DailyWeather {
    fn from(daily: Daily) -> Self {
        Self {
            date: daily.time,
            min_temp: daily.temperature_2m_min,
            max_temp: daily.temperature_2m_max,
            weather_code: daily.weather_code,
            precipitation_sum: daily.precipitation_sum,
            precipitation_probability_max: daily.precipitation_probability_max,
        }
    }
}

impl From<WeatherResponse> for Weather {
    fn from(response: WeatherResponse) -> Self {
        Self {
            current: response.current.into(),
            hourly: response.hourly.into(),
            daily: response.daily.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum IntOrString {
    Int(i64),
    Str(String),
}

pub fn de_opt_i32_from_any<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<IntOrString>::deserialize(deserializer)?;

    match opt {
        None => Ok(None),

        Some(IntOrString::Int(n)) => i32::try_from(n).map(Some).map_err(de::Error::custom),

        Some(IntOrString::Str(s)) => {
            let t = s.trim();

            if t.is_empty() || t.eq_ignore_ascii_case("undefined") || t.eq_ignore_ascii_case("null")
            {
                return Ok(None);
            }

            t.parse::<i32>().map(Some).map_err(de::Error::custom)
        }
    }
}
