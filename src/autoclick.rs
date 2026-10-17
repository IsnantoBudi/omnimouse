pub const MIN_INTERVAL_MS: u32 = 10;
pub const MAX_INTERVAL_MS: u32 = 500;
pub const DEFAULT_INTERVAL_MS: u32 = 100;

#[inline]
pub fn clamp_interval(ms: u32) -> u32 {
    ms.clamp(MIN_INTERVAL_MS, MAX_INTERVAL_MS)
}

#[inline]
pub fn calc_cps(ms: u32) -> f64 {
    let valid_ms = clamp_interval(ms);
    1000.0 / (valid_ms as f64)
}

pub fn format_interval_label(ms: u32) -> String {
    let valid_ms = clamp_interval(ms);
    let cps = calc_cps(valid_ms);
    format!("Interval: {} ms (~{:.1} Klik/Detik)", valid_ms, cps)
}

pub fn calc_click_timings(interval_ms: u32) -> (u64, u64) {
    let valid_ms = clamp_interval(interval_ms) as u64;
    let down_ms = if valid_ms <= 20 {
        5
    } else {
        10
    };
    let up_ms = valid_ms.saturating_sub(down_ms);
    (down_ms, up_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_interval_bounds() {
        assert_eq!(clamp_interval(0), MIN_INTERVAL_MS);
        assert_eq!(clamp_interval(5), MIN_INTERVAL_MS);
        assert_eq!(clamp_interval(10), 10);
        assert_eq!(clamp_interval(100), 100);
        assert_eq!(clamp_interval(500), 500);
        assert_eq!(clamp_interval(999), MAX_INTERVAL_MS);
    }

    #[test]
    fn test_calc_cps() {
        let cps_10 = calc_cps(10);
        assert!((cps_10 - 100.0).abs() < 0.001);

        let cps_50 = calc_cps(50);
        assert!((cps_50 - 20.0).abs() < 0.001);

        let cps_100 = calc_cps(100);
        assert!((cps_100 - 10.0).abs() < 0.001);

        let cps_200 = calc_cps(200);
        assert!((cps_200 - 5.0).abs() < 0.001);

        let cps_500 = calc_cps(500);
        assert!((cps_500 - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_click_timings_sum() {
        for ms in [10, 20, 50, 100, 250, 500] {
            let (down, up) = calc_click_timings(ms);
            assert_eq!(down + up, ms as u64);
            assert!(down > 0);
            assert!(up >= 5);
        }
    }

    #[test]
    fn test_format_interval_label() {
        let s = format_interval_label(50);
        assert!(s.contains("50 ms"));
        assert!(s.contains("20.0"));
    }
}
