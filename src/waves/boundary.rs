use crate::waves::{WaveParameters, VelocityCalculator};

/// Boundary condition applicator for wave generation
pub struct BoundaryApplicator {
    /// Velocity calculator for wave generation
    velocity_calc: VelocityCalculator,
    /// Current simulation time
    current_time: f64,
    /// Wave generation position (typically x = 0)
    generation_position: f64,
    /// Flag to enable/disable wave generation
    enabled: bool,
}

impl BoundaryApplicator {
    /// Create new boundary applicator with wave parameters
    pub fn new(params: WaveParameters) -> Self {
        Self {
            velocity_calc: VelocityCalculator::new(params),
            current_time: 0.0,
            generation_position: 0.0,
            enabled: true,
        }
    }
    
    /// Update wave parameters
    pub fn update_parameters(&mut self, params: WaveParameters) {
        self.velocity_calc.update_parameters(params);
    }
    
    /// Set wave generation position
    pub fn set_generation_position(&mut self, x: f64) {
        self.generation_position = x;
    }
    
    /// Enable or disable wave generation
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Check if wave generation is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Update simulation time
    pub fn update_time(&mut self, time: f64) {
        self.current_time = time;
    }
    
    /// Get current simulation time
    pub fn current_time(&self) -> f64 {
        self.current_time
    }
    
    /// Compute boundary velocity at current time
    pub fn boundary_velocity(&self) -> f64 {
        if !self.enabled {
            return 0.0;
        }
        
        self.velocity_calc.horizontal_velocity(self.generation_position, self.current_time)
    }
    
    /// Compute boundary surface elevation at current time
    pub fn boundary_surface_elevation(&self) -> f64 {
        if !self.enabled {
            return 0.0;
        }
        
        self.velocity_calc.surface_elevation(self.generation_position, self.current_time)
    }
    
    /// Apply boundary conditions to a 1D grid
    /// This is a placeholder for integration with actual solver
    pub fn apply_boundary_conditions(&self, velocities: &mut [f64], surface_elevations: &mut [f64]) {
        if !self.enabled || velocities.is_empty() || surface_elevations.is_empty() {
            return;
        }
        
        // Apply wave generation at the first grid point (left boundary)
        velocities[0] = self.boundary_velocity();
        surface_elevations[0] = self.boundary_surface_elevation();
    }
    
    /// Get wave parameters
    pub fn parameters(&self) -> &WaveParameters {
        self.velocity_calc.parameters()
    }
    
    /// Compute ramp-up factor for smooth wave generation startup
    pub fn ramp_up_factor(&self, ramp_duration: f64) -> f64 {
        if !self.enabled {
            return 0.0;
        }
        
        if ramp_duration <= 0.0 {
            return 1.0;
        }
        
        if self.current_time < ramp_duration {
            // Smooth ramp using cosine taper
            let t_normalized = self.current_time / ramp_duration;
            0.5 * (1.0 - (std::f64::consts::PI * t_normalized).cos())
        } else {
            1.0
        }
    }
    
    /// Apply ramped boundary conditions for smooth startup
    pub fn apply_ramped_boundary_conditions(&self, velocities: &mut [f64], surface_elevations: &mut [f64], ramp_duration: f64) {
        if !self.enabled || velocities.is_empty() || surface_elevations.is_empty() {
            return;
        }
        
        let ramp_factor = self.ramp_up_factor(ramp_duration);
        
        // Apply wave generation at the first grid point with ramping
        velocities[0] = self.boundary_velocity() * ramp_factor;
        surface_elevations[0] = self.boundary_surface_elevation() * ramp_factor;
    }
    
    /// Compute boundary flux (velocity × depth) for mass conservation
    pub fn boundary_flux(&self) -> f64 {
        if !self.enabled {
            return 0.0;
        }
        
        let velocity = self.boundary_velocity();
        let depth = self.parameters().d;
        velocity * depth
    }
    
    /// Check if wave generation should be active based on time
    pub fn should_generate_waves(&self, simulation_duration: f64) -> bool {
        self.enabled && self.current_time < simulation_duration
    }
    
    /// Get wave generation status information
    pub fn status(&self) -> BoundaryStatus {
        BoundaryStatus {
            enabled: self.enabled,
            current_time: self.current_time,
            generation_position: self.generation_position,
            current_velocity: self.boundary_velocity(),
            current_elevation: self.boundary_surface_elevation(),
            wave_parameters: self.parameters().clone(),
        }
    }
    
    /// Reset boundary applicator to initial state
    pub fn reset(&mut self) {
        self.current_time = 0.0;
        self.enabled = true;
    }
    
    /// Advance time by one time step
    pub fn advance_time(&mut self, dt: f64) {
        self.current_time += dt;
    }
    
    /// Get recommended time step for stable wave generation
    pub fn recommended_time_step(&self) -> f64 {
        self.velocity_calc.recommended_time_step()
    }
}

/// Status information for wave generation boundary
#[derive(Debug, Clone)]
pub struct BoundaryStatus {
    /// Whether wave generation is enabled
    pub enabled: bool,
    /// Current simulation time
    pub current_time: f64,
    /// Wave generation position
    pub generation_position: f64,
    /// Current boundary velocity
    pub current_velocity: f64,
    /// Current boundary surface elevation
    pub current_elevation: f64,
    /// Wave parameters
    pub wave_parameters: WaveParameters,
}

impl BoundaryStatus {
    /// Get wave phase at current time
    pub fn current_phase(&self) -> f64 {
        let k = self.wave_parameters.k;
        let omega = self.wave_parameters.omega;
        k * self.generation_position - omega * self.current_time
    }
    
    /// Get wave period completion fraction
    pub fn period_completion(&self) -> f64 {
        let periods_elapsed = self.current_time / self.wave_parameters.period;
        periods_elapsed - periods_elapsed.floor()
    }
    
    /// Check if currently at wave crest
    pub fn at_wave_crest(&self, tolerance: f64) -> bool {
        let phase = self.current_phase();
        let crest_phase = phase % (2.0 * std::f64::consts::PI);
        crest_phase.abs() < tolerance || (crest_phase - 2.0 * std::f64::consts::PI).abs() < tolerance
    }
    
    /// Check if currently at wave trough
    pub fn at_wave_trough(&self, tolerance: f64) -> bool {
        let phase = self.current_phase();
        let trough_phase = (phase + std::f64::consts::PI) % (2.0 * std::f64::consts::PI);
        trough_phase.abs() < tolerance || (trough_phase - 2.0 * std::f64::consts::PI).abs() < tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::waves::dispersion::DispersionSolver;

    fn create_test_boundary_applicator() -> BoundaryApplicator {
        let solver = DispersionSolver::new();
        let params = solver.solve_wave_parameters(0.5, 4.0, 2.0).unwrap();
        BoundaryApplicator::new(params)
    }
    
    #[test]
    fn test_boundary_applicator_creation() {
        let applicator = create_test_boundary_applicator();
        assert_eq!(applicator.current_time, 0.0);
        assert_eq!(applicator.generation_position, 0.0);
        assert!(applicator.enabled);
    }
    
    #[test]
    fn test_time_advancement() {
        let mut applicator = create_test_boundary_applicator();
        
        applicator.advance_time(0.1);
        assert_eq!(applicator.current_time, 0.1);
        
        applicator.update_time(1.0);
        assert_eq!(applicator.current_time, 1.0);
    }
    
    #[test]
    fn test_boundary_velocity() {
        let mut applicator = create_test_boundary_applicator();
        
        // At t=0, should have some velocity
        let v0 = applicator.boundary_velocity();
        assert!(v0 != 0.0);
        
        // At t=T/4, should be close to zero
        applicator.update_time(applicator.parameters().period / 4.0);
        let v_quarter = applicator.boundary_velocity();
        assert!(v_quarter.abs() < 1e-10);
        
        // At t=T/2, should be opposite to initial
        applicator.update_time(applicator.parameters().period / 2.0);
        let v_half = applicator.boundary_velocity();
        assert!((v0 + v_half).abs() < 1e-10);
    }
    
    #[test]
    fn test_boundary_surface_elevation() {
        let mut applicator = create_test_boundary_applicator();
        
        // At t=0, should have maximum elevation
        let eta0 = applicator.boundary_surface_elevation();
        assert_eq!(eta0, applicator.parameters().amplitude());
        
        // At t=T/4, should be zero
        applicator.update_time(applicator.parameters().period / 4.0);
        let eta_quarter = applicator.boundary_surface_elevation();
        assert!(eta_quarter.abs() < 1e-10);
    }
    
    #[test]
    fn test_enable_disable() {
        let mut applicator = create_test_boundary_applicator();
        
        // Initially enabled
        assert!(applicator.is_enabled());
        let v_enabled = applicator.boundary_velocity();
        assert!(v_enabled != 0.0);
        
        // Disable
        applicator.set_enabled(false);
        assert!(!applicator.is_enabled());
        let v_disabled = applicator.boundary_velocity();
        assert_eq!(v_disabled, 0.0);
        
        // Re-enable
        applicator.set_enabled(true);
        assert!(applicator.is_enabled());
        let v_reenabled = applicator.boundary_velocity();
        assert_eq!(v_reenabled, v_enabled);
    }
    
    #[test]
    fn test_ramp_up_factor() {
        let mut applicator = create_test_boundary_applicator();
        let ramp_duration = 2.0;
        
        // At t=0, should be 0
        applicator.update_time(0.0);
        assert_eq!(applicator.ramp_up_factor(ramp_duration), 0.0);
        
        // At t=ramp_duration, should be 1
        applicator.update_time(ramp_duration);
        assert!((applicator.ramp_up_factor(ramp_duration) - 1.0).abs() < 1e-10);
        
        // At t=ramp_duration/2, should be 0.5
        applicator.update_time(ramp_duration / 2.0);
        assert!((applicator.ramp_up_factor(ramp_duration) - 0.5).abs() < 1e-10);
        
        // Beyond ramp duration, should be 1
        applicator.update_time(ramp_duration * 2.0);
        assert_eq!(applicator.ramp_up_factor(ramp_duration), 1.0);
    }
    
    #[test]
    fn test_boundary_conditions_application() {
        let applicator = create_test_boundary_applicator();
        let mut velocities = vec![0.0; 10];
        let mut elevations = vec![0.0; 10];
        
        applicator.apply_boundary_conditions(&mut velocities, &mut elevations);
        
        // First element should be set to boundary values
        assert_eq!(velocities[0], applicator.boundary_velocity());
        assert_eq!(elevations[0], applicator.boundary_surface_elevation());
        
        // Other elements should remain unchanged
        for i in 1..10 {
            assert_eq!(velocities[i], 0.0);
            assert_eq!(elevations[i], 0.0);
        }
    }
    
    #[test]
    fn test_ramped_boundary_conditions() {
        let mut applicator = create_test_boundary_applicator();
        let mut velocities = vec![0.0; 10];
        let mut elevations = vec![0.0; 10];
        let ramp_duration = 2.0;
        
        // At t=0, should apply zero boundary conditions
        applicator.update_time(0.0);
        applicator.apply_ramped_boundary_conditions(&mut velocities, &mut elevations, ramp_duration);
        assert_eq!(velocities[0], 0.0);
        assert_eq!(elevations[0], 0.0);
        
        // At t=ramp_duration, should apply full boundary conditions
        applicator.update_time(ramp_duration);
        applicator.apply_ramped_boundary_conditions(&mut velocities, &mut elevations, ramp_duration);
        assert_eq!(velocities[0], applicator.boundary_velocity());
        assert_eq!(elevations[0], applicator.boundary_surface_elevation());
    }
    
    #[test]
    fn test_boundary_flux() {
        let applicator = create_test_boundary_applicator();
        let expected_flux = applicator.boundary_velocity() * applicator.parameters().d;
        assert_eq!(applicator.boundary_flux(), expected_flux);
    }
    
    #[test]
    fn test_status() {
        let applicator = create_test_boundary_applicator();
        let status = applicator.status();
        
        assert_eq!(status.enabled, applicator.is_enabled());
        assert_eq!(status.current_time, applicator.current_time());
        assert_eq!(status.generation_position, applicator.generation_position);
        assert_eq!(status.current_velocity, applicator.boundary_velocity());
        assert_eq!(status.current_elevation, applicator.boundary_surface_elevation());
    }
    
    #[test]
    fn test_reset() {
        let mut applicator = create_test_boundary_applicator();
        
        // Modify state
        applicator.advance_time(5.0);
        applicator.set_enabled(false);
        
        // Reset
        applicator.reset();
        
        // Should be back to initial state
        assert_eq!(applicator.current_time, 0.0);
        assert!(applicator.is_enabled());
    }
    
    #[test]
    fn test_recommended_time_step() {
        let applicator = create_test_boundary_applicator();
        let dt = applicator.recommended_time_step();
        
        assert!(dt > 0.0);
        assert!(dt < applicator.parameters().period / 10.0);
    }
}