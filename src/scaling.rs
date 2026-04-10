//! Scaling utilities for gauge ratios.

/// Apply logarithmic scaling to a ratio value.
///
/// Converts a linear ratio (0.0 to 1.0) to a logarithmic scale, making smaller values
/// more visible in the UI. This is useful for displaying power consumption which often
/// has wide dynamic range.
///
/// The function uses base-10 logarithm and scales the result so that:
/// - 0.0 remains 0.0
/// - 1.0 remains 1.0
/// - Values between have non-linear distribution with more visual space for low values
///
/// # Arguments
///
/// * `ratio` - A value between 0.0 and 1.0
///
/// # Returns
///
/// A scaled ratio value between 0.0 and 1.0
pub fn log_scale_ratio(ratio: f64) -> f64 {
    if ratio <= 0.0 {
        return 0.0;
    }
    if ratio >= 1.0 {
        return 1.0;
    }

    // Use base-10 logarithm with scaling factor
    // log10_scale transforms [0.001, 1.0] to approximately [0.0, 1.0]
    let log_scale_factor = 3.0; // This maps the range [10^-3, 1] to [0, 1]
    let log_value = ratio.log10() / log_scale_factor + 1.0; // Shift to [0, 1] range

    log_value.clamp(0.0, 1.0)
}

/// Apply logarithmic scaling to a 32-bit float ratio.
///
/// Same as `log_scale_ratio` but accepts and returns f32.
pub fn log_scale_ratio_f32(ratio: f32) -> f32 {
    log_scale_ratio(ratio as f64) as f32
}

/// Apply either linear or logarithmic scaling based on a flag.
///
/// # Arguments
///
/// * `ratio` - A value between 0.0 and 1.0
/// * `use_log_scale` - If true, apply logarithmic scaling; otherwise return ratio unchanged
///
/// # Returns
///
/// The possibly scaled ratio value
pub fn apply_scaling(ratio: f64, use_log_scale: bool) -> f64 {
    if use_log_scale {
        log_scale_ratio(ratio)
    } else {
        ratio.clamp(0.0, 1.0)
    }
}

/// Apply either linear or logarithmic scaling based on a flag (f32 version).
pub fn apply_scaling_f32(ratio: f32, use_log_scale: bool) -> f32 {
    if use_log_scale {
        log_scale_ratio_f32(ratio)
    } else {
        ratio.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_scale_boundaries() {
        assert_eq!(log_scale_ratio(0.0), 0.0);
        assert_eq!(log_scale_ratio(1.0), 1.0);
    }

    #[test]
    fn test_log_scale_mid_range() {
        let mid = log_scale_ratio(0.5);
        assert!(mid > 0.0 && mid < 1.0);
        // At 0.5, log scale should give a higher value than linear
        assert!(mid > 0.5);
    }

    #[test]
    fn test_log_scale_low_values() {
        let low = log_scale_ratio(0.01);
        assert!(low > 0.0);
        // Low values should be amplified by log scale
        assert!(low > 0.01);
    }

    #[test]
    fn test_apply_scaling_linear() {
        assert_eq!(apply_scaling(0.5, false), 0.5);
    }

    #[test]
    fn test_apply_scaling_log() {
        let linear = 0.5;
        let log_scaled = apply_scaling(linear, true);
        assert!(log_scaled > linear);
    }
}
