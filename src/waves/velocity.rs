use crate::waves::parameters::WaveParameters;

/// Velocity calculator for wave generation using linear wave theory
pub struct VelocityCalculator {
    /// Wave parameters
    params: WaveParameters,
    /// Gravitational acceleration [m/s²]
    gravity: f64,
}

impl VelocityCalculator {
    /// Create new velocity calculator with wave parameters
    pub fn new(params: WaveParameters) -> Self {
        Self {
            params,
            gravity: 9.81,
        }
    }
    
    /// Update wave parameters
    pub fn update_parameters(&mut self, params: WaveParameters) {
        self.params = params;
    }
    
    /// Compute horizontal velocity at given position and time
    /// For one-layer (depth-averaged) case using linear wave theory
    pub fn horizontal_velocity(&self, x: f64, time: f64) -> f64 {
        let phase = self.params.k * x - self.params.omega * time;
        
        // For depth-averaged case, use linear wave theory velocity
        // u = (H/2) * (ω/k) * cos(kx - ωt) * cosh(k(z + d))/sinh(kd)
        // For depth-averaged: integrate over depth and divide by depth
        
        let amplitude = self.params.amplitude();
        let kd = self.params.k * self.params.d;
        
        // Depth-averaged velocity coefficient
        let velocity_coeff = if kd < 0.1 {
            // Shallow water limit: tanh(kd) ≈ kd, sinh(kd) ≈ kd
            1.0
        } else {
            // General case: use hyperbolic functions
            kd.tanh()
        };
        
        amplitude * self.params.c * velocity_coeff * phase.cos()
    }
    
    /// Compute vertical velocity at given position and time
    /// For 1D horizontal wave propagation, w = 0 (no vertical motion)
    pub fn vertical_velocity(&self, _x: f64, _time: f64) -> f64 {
        0.0
    }
    
    /// Compute velocity amplitude (maximum horizontal velocity)
    pub fn velocity_amplitude(&self) -> f64 {
        let kd = self.params.k * self.params.d;
        let velocity_coeff = if kd < 0.1 {
            1.0
        } else {
            kd.tanh()
        };
        
        self.params.amplitude() * self.params.c * velocity_coeff
    }
    
    /// Compute particle displacement at given position and time
    pub fn particle_displacement(&self, x: f64, time: f64) -> f64 {
        let phase = self.params.k * x - self.params.omega * time;
        let amplitude = self.params.amplitude();
        let kd = self.params.k * self.params.d;
        
        // Horizontal particle displacement
        let displacement_coeff = if kd < 0.1 {
            1.0
        } else {
            kd.tanh()
        };
        
        amplitude * displacement_coeff * phase.sin()
    }
    
    /// Compute wave orbital velocity components for educational purposes
    pub fn orbital_velocity_components(&self, x: f64, time: f64) -> (f64, f64) {
        let u = self.horizontal_velocity(x, time);
        let w = self.vertical_velocity(x, time);
        (u, w)
    }
    
    /// Get wave parameters
    pub fn parameters(&self) -> &WaveParameters {
        &self.params
    }
    
    /// Compute time series of velocity at fixed position
    pub fn velocity_time_series(&self, x: f64, time_points: &[f64]) -> Vec<f64> {
        time_points.iter()
            .map(|&t| self.horizontal_velocity(x, t))
            .collect()
    }
    
    /// Compute spatial series of velocity at fixed time
    pub fn velocity_spatial_series(&self, x_points: &[f64], time: f64) -> Vec<f64> {
        x_points.iter()
            .map(|&x| self.horizontal_velocity(x, time))
            .collect()
    }
    
    /// Validate velocity calculation by checking energy conservation
    pub fn validate_energy_conservation(&self, x: f64, time: f64) -> Result<f64, String> {
        let u = self.horizontal_velocity(x, time);
        let eta = self.surface_elevation(x, time);
        
        // Kinetic energy density: (1/2) * ρ * u² * d
        let kinetic_energy = 0.5 * u * u * self.params.d;
        
        // Potential energy density: (1/2) * ρ * g * η²
        let potential_energy = 0.5 * self.gravity * eta * eta;
        
        // Total energy should be constant
        let total_energy = kinetic_energy + potential_energy;
        
        // Expected energy for linear waves: (1/8) * ρ * g * H²
        let expected_energy = 0.125 * self.gravity * self.params.h * self.params.h;
        
        let energy_error = (total_energy - expected_energy).abs() / expected_energy;
        
        if energy_error > 0.1 {
            return Err(format!("Energy conservation violated: error = {:.2e}", energy_error));
        }
        
        Ok(energy_error)
    }
    
    /// Compute surface elevation at given position and time
    pub fn surface_elevation(&self, x: f64, time: f64) -> f64 {
        let phase = self.params.k * x - self.params.omega * time;
        self.params.amplitude() * phase.cos()
    }
    
    /// Compute wave steepness parameter (ak = kH/2)
    pub fn wave_steepness(&self) -> f64 {
        self.params.k * self.params.amplitude()
    }
    
    /// Check if wave is in linear regime (ak < 0.1)
    pub fn is_linear(&self) -> bool {
        self.wave_steepness() < 0.1
    }
    
    /// Get recommended time step for stable numerical integration
    pub fn recommended_time_step(&self) -> f64 {
        // CFL condition: Δt ≤ Δx / c
        // Use conservative factor of 0.5
        let min_wavelength = self.params.wavelength;
        let typical_dx = min_wavelength / 20.0; // 20 points per wavelength
        0.5 * typical_dx / self.params.c
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::waves::dispersion::DispersionSolver;

    fn create_test_velocity_calculator() -> VelocityCalculator {
        let solver = DispersionSolver::new();
        let params = solver.solve_wave_parameters(0.5, 4.0, 2.0).unwrap();
        VelocityCalculator::new(params)
    }
    
    #[test]
    fn test_velocity_calculator_creation() {
        let calc = create_test_velocity_calculator();
        assert_eq!(calc.params.h, 0.5);
        assert_eq!(calc.params.period, 4.0);
        assert_eq!(calc.params.d, 2.0);
    }
    
    #[test]
    fn test_horizontal_velocity() {
        let calc = create_test_velocity_calculator();
        
        // At x=0, t=0: phase = 0, cos(0) = 1, maximum velocity
        let u_max = calc.horizontal_velocity(0.0, 0.0);
        assert!(u_max > 0.0);
        
        // At x=0, t=T/4: phase = -π/2, cos(-π/2) = 0, zero velocity
        let u_zero = calc.horizontal_velocity(0.0, calc.params.period / 4.0);
        assert!(u_zero.abs() < 1e-10);
        
        // At x=0, t=T/2: phase = -π, cos(-π) = -1, minimum velocity
        let u_min = calc.horizontal_velocity(0.0, calc.params.period / 2.0);
        assert!(u_min < 0.0);
        assert!((u_min + u_max).abs() < 1e-10); // Should be symmetric
    }
    
    #[test]
    fn test_vertical_velocity() {
        let calc = create_test_velocity_calculator();
        
        // For 1D horizontal propagation, vertical velocity should be zero
        let w = calc.vertical_velocity(0.0, 0.0);
        assert_eq!(w, 0.0);
    }
    
    #[test]
    fn test_velocity_amplitude() {
        let calc = create_test_velocity_calculator();
        let u_amp = calc.velocity_amplitude();
        
        // Should be positive
        assert!(u_amp > 0.0);
        
        // Should be consistent with maximum velocity
        let u_max = calc.horizontal_velocity(0.0, 0.0);
        assert!((u_amp - u_max).abs() < 1e-10);
    }
    
    #[test]
    fn test_surface_elevation() {
        let calc = create_test_velocity_calculator();
        
        // At x=0, t=0: phase = 0, cos(0) = 1, maximum elevation
        let eta_max = calc.surface_elevation(0.0, 0.0);
        assert_eq!(eta_max, calc.params.amplitude());
        
        // At x=0, t=T/4: phase = -π/2, cos(-π/2) = 0, zero elevation
        let eta_zero = calc.surface_elevation(0.0, calc.params.period / 4.0);
        assert!(eta_zero.abs() < 1e-10);
    }
    
    #[test]
    fn test_wave_steepness() {
        let calc = create_test_velocity_calculator();
        let steepness = calc.wave_steepness();
        
        // Should be positive and reasonable for linear waves
        assert!(steepness > 0.0);
        assert!(steepness < 0.5); // Should be well within linear regime
    }
    
    #[test]
    fn test_linearity_check() {
        let calc = create_test_velocity_calculator();
        
        // With moderate wave height, should be linear
        assert!(calc.is_linear());
    }
    
    #[test]
    fn test_time_series() {
        let calc = create_test_velocity_calculator();
        let time_points: Vec<f64> = (0..10).map(|i| i as f64 * 0.1).collect();
        let velocities = calc.velocity_time_series(0.0, &time_points);
        
        assert_eq!(velocities.len(), time_points.len());
        
        // Check that velocities are periodic
        let period_points = (calc.params.period / 0.1) as usize;
        if velocities.len() > period_points {
            let diff = (velocities[0] - velocities[period_points]).abs();
            assert!(diff < 1e-10, "Velocity not periodic: diff = {:.2e}", diff);
        }
    }
    
    #[test]
    fn test_spatial_series() {
        let calc = create_test_velocity_calculator();
        let x_points: Vec<f64> = (0..10).map(|i| i as f64 * 0.1).collect();
        let velocities = calc.velocity_spatial_series(&x_points, 0.0);
        
        assert_eq!(velocities.len(), x_points.len());
        
        // Check that velocities are spatially periodic
        let wavelength_points = (calc.params.wavelength / 0.1) as usize;
        if velocities.len() > wavelength_points {
            let diff = (velocities[0] - velocities[wavelength_points]).abs();
            assert!(diff < 1e-10, "Velocity not spatially periodic: diff = {:.2e}", diff);
        }
    }
    
    #[test]
    fn test_recommended_time_step() {
        let calc = create_test_velocity_calculator();
        let dt = calc.recommended_time_step();
        
        // Should be positive and reasonable
        assert!(dt > 0.0);
        assert!(dt < calc.params.period / 10.0); // Should be much smaller than period
    }
}