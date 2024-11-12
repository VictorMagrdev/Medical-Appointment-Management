#[macro_export]
macro_rules! json_get {
    ($json:expr, $key:expr, String) => {
        $json
            .get($key)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    };
    ($json:expr, $key:expr, i32) => {
        $json.get($key).and_then(|v| v.as_i64()).unwrap_or(0) as i32
    };
    ($json:expr, $key:expr, i64) => {
        $json.get($key).and_then(|v| v.as_i64()).unwrap_or(0)
    };
    ($json:expr, $key:expr, f64) => {
        $json.get($key).and_then(|v| v.as_f64()).unwrap_or(0.0)
    };
    ($json:expr, $key:expr, Date) => {
        $json
            .get($key)
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
            .unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
    };
}
