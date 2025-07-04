use crate::waves::parameters::WaveParameters;

/// Dispersion relation solver for SWASH-style wave generation
pub struct DispersionSolver {
    /// Maximum iterations for Newton-Raphson solver
    max_iterations: usize,
    /// Convergence tolerance
    tolerance: f64,
    /// Gravitational acceleration [m/s²]
    gravity: f64,
}

impl Default for DispersionSolver {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            tolerance: 1e-10,
            gravity: 9.81,
        }
    }
}

impl DispersionSolver {
    /// Create new dispersion solver with default parameters
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create new dispersion solver with custom parameters
    pub fn with_params(max_iterations: usize, tolerance: f64, gravity: f64) -> Self {
        Self {
            max_iterations,
            tolerance,
            gravity,
        }
    }
    
    /// Solve wave parameters using one-layer SWASH dispersion relation
    pub fn solve_wave_parameters(&self, wave_height: f64, wave_period: f64, water_depth: f64) -> Result<WaveParameters, String> {
        // Create initial wave parameters
        let mut params = WaveParameters::new(wave_height, wave_period, water_depth)?;
        
        // Solve for wave number using Newton-Raphson iteration
        let wave_number = self.solve_wave_number(params.omega, water_depth)?;
        
        // Update parameters with computed wave number
        params.update_from_dispersion(wave_number);
        
        // Validate final parameters
        params.validate()?;
        
        Ok(params)
    }
    
    /// Solve for wave number given angular frequency and depth
    /// Uses one-layer SWASH dispersion relation: ω² = gk * (kd)/(1 + (kd)²/4)
    fn solve_wave_number(&self, omega: f64, depth: f64) -> Result<f64, String> {
        // Initial guess: deep water wave number
        let mut k = omega * omega / self.gravity;
        
        for _iteration in 0..self.max_iterations {
            let f = self.dispersion_function(k, omega, depth);
            let df_dk = self.dispersion_derivative(k, omega, depth);
            
            if df_dk.abs() < self.tolerance {
                return Err("Derivative too small in Newton-Raphson iteration".to_string());
            }
            
            let k_new = k - f / df_dk;
            
            // Check convergence
            if (k_new - k).abs() < self.tolerance {
                return Ok(k_new);
            }
            
            // Ensure positive wave number
            k = k_new.max(self.tolerance);
        }
        
        Err(format!("Newton-Raphson failed to converge after {} iterations", self.max_iterations))
    }
    
    /// One-layer SWASH dispersion function: f(k) = ω² - gk * (kd)/(1 + (kd)²/4)
    fn dispersion_function(&self, k: f64, omega: f64, depth: f64) -> f64 {
        let kd = k * depth;
        let dispersion_rhs = self.gravity * k * kd / (1.0 + kd * kd / 4.0);
        omega * omega - dispersion_rhs
    }
    
    /// Derivative of dispersion function with respect to k
    fn dispersion_derivative(&self, k: f64, _omega: f64, depth: f64) -> f64 {
        let kd = k * depth;
        let kd2 = kd * kd;
        let denominator = 1.0 + kd2 / 4.0;
        let denominator2 = denominator * denominator;
        
        // d/dk [gk * (kd)/(1 + (kd)²/4)]
        // = g * [kd/(1 + (kd)²/4) + k * d * (1 + (kd)²/4 - kd * kd/2) / (1 + (kd)²/4)²]
        // = g * [kd/(1 + (kd)²/4) + k * d * (1 - (kd)²/4) / (1 + (kd)²/4)²]
        
        let term1 = kd / denominator;
        let term2 = k * depth * (1.0 - kd2 / 4.0) / denominator2;
        
        -self.gravity * (term1 + term2)
    }
    
    /// Compute phase velocity from dispersion relation
    pub fn phase_velocity(&self, k: f64, depth: f64) -> f64 {
        let kd = k * depth;
        let c_squared = self.gravity * kd / (k * (1.0 + kd * kd / 4.0));
        c_squared.sqrt()
    }
    
    /// Compute group velocity (∂ω/∂k)
    pub fn group_velocity(&self, k: f64, depth: f64) -> f64 {
        let kd = k * depth;
        let kd2 = kd * kd;
        let denominator = 1.0 + kd2 / 4.0;
        
        // For ω² = gk * (kd)/(1 + (kd)²/4), compute ∂ω/∂k
        let omega_squared = self.gravity * k * kd / denominator;
        let omega = omega_squared.sqrt();
        
        // ∂ω/∂k = (1/2ω) * ∂(ω²)/∂k
        let domega2_dk = self.gravity * depth * (1.0 - kd2 / 4.0) / denominator.powi(2);
        
        domega2_dk / (2.0 * omega)
    }
    
    /// Validate dispersion relation accuracy against linear theory
    pub fn validate_dispersion(&self, k: f64, omega: f64, depth: f64) -> Result<f64, String> {
        // Compute dispersion relation residual
        let residual = self.dispersion_function(k, omega, depth);
        
        // Check if residual is small enough
        if residual.abs() > 1e-6 {
            return Err(format!("Dispersion relation not satisfied: residual = {:.2e}", residual));
        }
        
        Ok(residual)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_dispersion_solver_creation() {
        let solver = DispersionSolver::new();
        assert_eq!(solver.max_iterations, 100);
        assert_eq!(solver.tolerance, 1e-10);
        assert_eq!(solver.gravity, 9.81);
    }
    
    #[test]
    fn test_shallow_water_limit() {
        let solver = DispersionSolver::new();
        let params = solver.solve_wave_parameters(0.3, 4.0, 1.0).unwrap();
        
        // In shallow water, c ≈ √(gd)
        let expected_c = (solver.gravity * params.d).sqrt();
        let relative_error = (params.c - expected_c).abs() / expected_c;
        
        // Should be very close to shallow water limit
        assert!(relative_error < 0.1, "Shallow water limit not satisfied: c = {:.3}, expected = {:.3}", params.c, expected_c);
    }
    
    #[test]
    fn test_deep_water_limit() {
        let solver = DispersionSolver::new();
        let params = solver.solve_wave_parameters(1.0, 8.0, 20.0).unwrap();
        
        // In deep water, c = gT/(2π)
        let expected_c = solver.gravity * params.period / (2.0 * PI);
        let relative_error = (params.c - expected_c).abs() / expected_c;
        
        // Should be reasonably close to deep water limit
        assert!(relative_error < 0.1, "Deep water limit not satisfied: c = {:.3}, expected = {:.3}", params.c, expected_c);
    }
    
    #[test]
    fn test_dispersion_validation() {
        let solver = DispersionSolver::new();
        let params = solver.solve_wave_parameters(1.0, 4.0, 2.0).unwrap();
        
        // Validate that dispersion relation is satisfied
        let result = solver.validate_dispersion(params.k, params.omega, params.d);
        assert!(result.is_ok());
        
        let residual = result.unwrap();
        assert!(residual.abs() < 1e-6, "Dispersion relation residual too large: {:.2e}", residual);
    }
    
    #[test]
    fn test_phase_velocity_consistency() {
        let solver = DispersionSolver::new();
        let params = solver.solve_wave_parameters(1.0, 4.0, 2.0).unwrap();
        
        // Compute phase velocity directly from dispersion solver
        let c_direct = solver.phase_velocity(params.k, params.d);
        
        // Should match c from wave parameters
        let relative_error = (params.c - c_direct).abs() / params.c;
        assert!(relative_error < 1e-6, "Phase velocity inconsistency: c = {:.6}, c_direct = {:.6}", params.c, c_direct);
    }
}