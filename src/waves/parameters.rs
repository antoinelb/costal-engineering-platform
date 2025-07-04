use std::f64::consts::PI;

/// Wave parameters structure for SWASH-style wave generation
#[derive(Debug, Clone)]
pub struct WaveParameters {
    /// Wave number (2π/L) [rad/m]
    pub k: f64,
    /// Angular frequency (2π/T) [rad/s]
    pub omega: f64,
    /// Phase velocity (ω/k) [m/s]
    pub c: f64,
    /// Wave height (H) [m]
    pub h: f64,
    /// Water depth (d) [m]
    pub d: f64,
    /// Wave period (T) [s]
    pub period: f64,
    /// Wavelength (L) [m]
    pub wavelength: f64,
}

impl WaveParameters {
    /// Create new wave parameters from basic inputs
    pub fn new(wave_height: f64, wave_period: f64, water_depth: f64) -> Result<Self, String> {
        // Validate inputs
        if wave_height <= 0.0 {
            return Err("Wave height must be positive".to_string());
        }
        if wave_period <= 0.0 {
            return Err("Wave period must be positive".to_string());
        }
        if water_depth <= 0.0 {
            return Err("Water depth must be positive".to_string());
        }
        
        // Check wave breaking criterion (H/d < 0.78 for depth-limited breaking)
        let breaking_ratio = wave_height / water_depth;
        if breaking_ratio > 0.78 {
            return Err(format!("Wave may break: H/d = {:.3} > 0.78", breaking_ratio));
        }
        
        let omega = 2.0 * PI / wave_period;
        
        // Initial parameters - k and c will be computed by dispersion solver
        Ok(WaveParameters {
            k: 0.0,        // To be computed
            omega,
            c: 0.0,        // To be computed
            h: wave_height,
            d: water_depth,
            period: wave_period,
            wavelength: 0.0, // To be computed
        })
    }
    
    /// Update wave parameters after dispersion relation solution
    pub fn update_from_dispersion(&mut self, wave_number: f64) {
        self.k = wave_number;
        self.c = self.omega / wave_number;
        self.wavelength = 2.0 * PI / wave_number;
    }
    
    /// Get wave amplitude (H/2)
    pub fn amplitude(&self) -> f64 {
        self.h / 2.0
    }
    
    /// Get frequency (1/T)
    pub fn frequency(&self) -> f64 {
        1.0 / self.period
    }
    
    /// Get dimensionless depth parameter (kd)
    pub fn dimensionless_depth(&self) -> f64 {
        self.k * self.d
    }
    
    /// Get depth-to-wavelength ratio (d/L)
    pub fn depth_wavelength_ratio(&self) -> f64 {
        self.d / self.wavelength
    }
    
    /// Classify water depth regime based on d/L ratio
    pub fn water_depth_regime(&self) -> WaterDepthRegime {
        let ratio = self.depth_wavelength_ratio();
        if ratio < 1.0 / 20.0 {
            WaterDepthRegime::Shallow
        } else if ratio > 0.5 {
            WaterDepthRegime::Deep
        } else {
            WaterDepthRegime::Intermediate
        }
    }
    
    /// Validate wave parameters for physical consistency
    pub fn validate(&self) -> Result<(), String> {
        if self.k <= 0.0 {
            return Err("Wave number must be positive".to_string());
        }
        if self.omega <= 0.0 {
            return Err("Angular frequency must be positive".to_string());
        }
        if self.c <= 0.0 {
            return Err("Phase velocity must be positive".to_string());
        }
        
        // Check if parameters are consistent
        let expected_c = self.omega / self.k;
        if (self.c - expected_c).abs() > 1e-6 {
            return Err(format!("Inconsistent parameters: c = {:.6}, ω/k = {:.6}", self.c, expected_c));
        }
        
        Ok(())
    }
}

/// Water depth regime classification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WaterDepthRegime {
    /// Shallow water: d/L < 1/20
    Shallow,
    /// Intermediate water: 1/20 ≤ d/L ≤ 1/2
    Intermediate,
    /// Deep water: d/L > 1/2
    Deep,
}

impl std::fmt::Display for WaterDepthRegime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WaterDepthRegime::Shallow => write!(f, "Shallow Water"),
            WaterDepthRegime::Intermediate => write!(f, "Intermediate Water"),
            WaterDepthRegime::Deep => write!(f, "Deep Water"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wave_parameters_creation() {
        let params = WaveParameters::new(1.0, 4.0, 2.0).unwrap();
        assert_eq!(params.h, 1.0);
        assert_eq!(params.period, 4.0);
        assert_eq!(params.d, 2.0);
        assert_eq!(params.amplitude(), 0.5);
        assert_eq!(params.frequency(), 0.25);
    }
    
    #[test]
    fn test_wave_breaking_validation() {
        // Should fail for waves that are too large
        let result = WaveParameters::new(2.0, 4.0, 2.0); // H/d = 1.0 > 0.78
        assert!(result.is_err());
        
        // Should succeed for reasonable waves
        let result = WaveParameters::new(1.0, 4.0, 2.0); // H/d = 0.5 < 0.78
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_invalid_parameters() {
        assert!(WaveParameters::new(0.0, 4.0, 2.0).is_err()); // Zero height
        assert!(WaveParameters::new(1.0, 0.0, 2.0).is_err()); // Zero period
        assert!(WaveParameters::new(1.0, 4.0, 0.0).is_err()); // Zero depth
    }
}