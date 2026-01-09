pub fn weather_icon_and_label(code: Option<i64>) -> (&'static str, &'static str) {
    match code {
        Some(0) => ("☀️", "Clear"),
        Some(1 | 2) => ("🌤️", "Mostly Clear"),
        Some(3) => ("☁️", "Cloudy"),
        Some(45 | 48) => ("🌫️", "Fog"),
        Some(51 | 53 | 55) => ("🌦️", "Drizzle"),
        Some(61 | 63 | 65) => ("🌧️", "Rain"),
        Some(66 | 67) => ("🌧️❄️", "Freezing Rain"),
        Some(71 | 73 | 75) => ("❄️", "Snow"),
        Some(77) => ("🌨️", "Snow Grains"),
        Some(80..=82) => ("🌦️", "Rain Showers"),
        Some(85 | 86) => ("🌨️", "Snow Showers"),
        Some(95) => ("⛈️", "Thunderstorm"),
        Some(96 | 99) => ("⛈️⚡", "Thunderstorm w/ Hail"),
        None => ("?", "Unknown"),
        _ => ("?", "Unknown"),
    }
}

#[cfg(test)]
mod tests {
    use super::weather_icon_and_label;

    fn assert_map(code: Option<i64>, icon: &'static str, label: &'static str) {
        assert_eq!(weather_icon_and_label(code), (icon, label));
    }

    #[test]
    fn maps_clear() {
        assert_map(Some(0), "☀️", "Clear");
    }

    #[test]
    fn maps_mostly_clear() {
        assert_map(Some(1), "🌤️", "Mostly Clear");
        assert_map(Some(2), "🌤️", "Mostly Clear");
    }

    #[test]
    fn maps_cloudy() {
        assert_map(Some(3), "☁️", "Cloudy");
    }

    #[test]
    fn maps_fog() {
        assert_map(Some(45), "🌫️", "Fog");
        assert_map(Some(48), "🌫️", "Fog");
    }

    #[test]
    fn maps_drizzle() {
        assert_map(Some(51), "🌦️", "Drizzle");
        assert_map(Some(53), "🌦️", "Drizzle");
        assert_map(Some(55), "🌦️", "Drizzle");
    }

    #[test]
    fn maps_rain() {
        assert_map(Some(61), "🌧️", "Rain");
        assert_map(Some(63), "🌧️", "Rain");
        assert_map(Some(65), "🌧️", "Rain");
    }

    #[test]
    fn maps_freezing_rain() {
        assert_map(Some(66), "🌧️❄️", "Freezing Rain");
        assert_map(Some(67), "🌧️❄️", "Freezing Rain");
    }

    #[test]
    fn maps_snow() {
        assert_map(Some(71), "❄️", "Snow");
        assert_map(Some(73), "❄️", "Snow");
        assert_map(Some(75), "❄️", "Snow");
    }

    #[test]
    fn maps_snow_grains() {
        assert_map(Some(77), "🌨️", "Snow Grains");
    }

    #[test]
    fn maps_rain_showers() {
        assert_map(Some(80), "🌦️", "Rain Showers");
        assert_map(Some(81), "🌦️", "Rain Showers");
        assert_map(Some(82), "🌦️", "Rain Showers");
    }

    #[test]
    fn maps_snow_showers() {
        assert_map(Some(85), "🌨️", "Snow Showers");
        assert_map(Some(86), "🌨️", "Snow Showers");
    }

    #[test]
    fn maps_thunderstorm() {
        assert_map(Some(95), "⛈️", "Thunderstorm");
    }

    #[test]
    fn maps_thunderstorm_with_hail() {
        assert_map(Some(96), "⛈️⚡", "Thunderstorm w/ Hail");
        assert_map(Some(99), "⛈️⚡", "Thunderstorm w/ Hail");
    }

    #[test]
    fn maps_none_to_unknown() {
        assert_map(None, "?", "Unknown");
    }

    #[test]
    fn maps_unhandled_codes_to_unknown() {
        assert_map(Some(-1), "?", "Unknown");
        assert_map(Some(4), "?", "Unknown");
        assert_map(Some(999), "?", "Unknown");
    }
}
