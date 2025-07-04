# SWASH Wave Generation Implementation Plan

## Executive Summary

This document outlines the implementation plan for SWASH-style wave generation in the coastal engineering platform. The implementation will focus on regular wave generation using layer-based dispersion relations with equidistant vertical layers, integrating with the existing 1D wave channel simulator architecture. For this implementation, waves propagate from left to right in a 1D channel configuration.

## Mathematical Foundation

### Layer-Based Dispersion Relations

SWASH uses layer-specific dispersion relations that provide optimal accuracy for the numerical discretization:

**One Layer (Depth-Averaged):**
$$\omega^2 = gk \cdot \frac{kd}{1 + \frac{k^2d^2}{4}}$$

**Two Layers:**
$$\omega^2 = gk \cdot \frac{kd + \frac{k^3d^3}{16}}{1 + \frac{3k^2d^2}{8} + \frac{k^4d^4}{256}}$$

**Three Layers:**
$$\omega^2 = gk \cdot \frac{kd + \frac{k^3d^3}{54} + \frac{k^5d^5}{1296}}{1 + \frac{5k^2d^2}{12} + \frac{5k^4d^4}{432} + \frac{k^6d^6}{46656}}$$

### Stokes Discrete Solutions

For equidistant layer distributions with ≤4 layers, SWASH uses analytical Stokes solutions derived from vertically discretized model equations. These provide:
- Layer-averaged velocities that exactly satisfy discrete governing equations
- Enhanced accuracy through perturbation theory
- Proper phase relationships between layers

### Boundary Condition Implementation

Wave generation occurs by prescribing horizontal velocities at the left boundary:
- **Horizontal component (u)**: Prescribed from wave theory (1D propagation)
- **Vertical component (w)**: Homogeneous Neumann condition (∂w/∂n = 0)
- **Boundary type**: Weakly reflective at right boundary to minimize spurious reflections
- **Wave direction**: Fixed left-to-right propagation (positive x-direction)

## Architecture Overview

### Core Components

#### 1. Wave Parameters Structure
```rust
pub struct WaveParameters {
    k: f64,          // wave number (from dispersion relation)
    omega: f64,      // angular frequency (2π/T)
    c: f64,          // phase velocity (ω/k)
    h: f64,          // wave height
    d: f64,          // water depth
    period: f64,     // wave period
    // Note: direction fixed as left-to-right (positive x) for 1D channel
}
```

#### 2. Layer Configuration
```rust
pub struct LayerConfig {
    num_layers: usize,
    thickness: Vec<f64>,  // Fractional thickness (equidistant assumed)
    total_depth: f64,
}
```

#### 3. Wave Generator
```rust
pub struct SwashWaveGenerator {
    wave_params: WaveParameters,
    layer_config: LayerConfig,
    dispersion_solver: DispersionSolver,
    velocity_calculator: VelocityCalculator,
    boundary_applicator: BoundaryApplicator,
}
```

### Integration Points

#### With Existing Grid System
- **Staggered grid compatibility**: Velocities applied at appropriate grid faces
- **Layer mapping**: Convert layer indices to grid vertical coordinates
- **Boundary identification**: Interface with existing boundary condition system

#### With Solver Architecture
- **Time integration**: Boundary conditions updated at each time step
- **CFL consideration**: Wave generation respects stability constraints
- **Conservation**: Maintains mass and momentum conservation properties

## Implementation Phases

### Phase 1: Core Wave Theory (Days 1-2)

**Objectives:**
- Implement layer-specific dispersion relation solvers
- Create Newton-Raphson iteration for wave number computation
- Develop velocity profile calculations for equidistant layers

**Deliverables:**
- `DispersionSolver` with layer-specific relations
- `WaveParameters` structure with validation
- Unit tests for dispersion accuracy

**Implementation Details:**
```rust
impl DispersionSolver {
    pub fn solve_wave_number(&self, omega: f64, depth: f64, num_layers: usize) -> f64 {
        // Newton-Raphson iteration for k given ω, d, and layer count
        // Uses layer-specific dispersion relations
    }
    
    fn dispersion_function(&self, k: f64, omega: f64, depth: f64, num_layers: usize) -> f64 {
        // Returns f(k) = ω² - dispersion_relation(k, d, layers)
    }
}
```

### Phase 2: Velocity Computation (Days 3-4)

**Objectives:**
- Implement Stokes discrete solutions for layer-averaged velocities
- Create fallback to linear theory for validation
- Develop layer-center coordinate mapping

**Deliverables:**
- `VelocityCalculator` with Stokes discrete and linear options
- Layer coordinate transformation utilities
- Validation against analytical solutions

**Implementation Details:**
```rust
impl VelocityCalculator {
    pub fn compute_layer_velocity(&self, layer: usize, x: f64, time: f64) -> f64 {
        // Returns u velocity for specified layer at given position and time
        // Uses Stokes discrete solutions for equidistant layers
        // 1D channel: only horizontal (u) component needed
    }
    
    fn stokes_discrete_amplitude(&self, layer: usize) -> f64 {
        // Layer-specific velocity amplitude from Stokes theory
    }
}
```

### Phase 3: Boundary Application (Days 5-6)

**Objectives:**
- Integrate with existing boundary condition system
- Implement weakly reflective boundary conditions
- Add time-dependent boundary updates

**Deliverables:**
- `BoundaryApplicator` integrated with solver
- Weakly reflective boundary implementation
- Real-time boundary condition updates

**Implementation Details:**
```rust
impl BoundaryApplicator {
    pub fn apply_wave_boundary(&self, solver: &mut WaveSolver, time: f64) {
        // Apply computed u velocities to left boundary grid points
        // Maintain proper phase relationships across layers
        // 1D channel: only left boundary wave generation needed
    }
    
    pub fn apply_weakly_reflective(&self, solver: &mut WaveSolver) {
        // Implement outgoing wave boundary conditions at right boundary
        // Allows waves to exit domain without reflection
    }
}
```

### Phase 4: Educational Integration (Day 7)

**Objectives:**
- Add educational tooltips for wave parameters
- Integrate with existing egui interface
- Create parameter adjustment controls

**Deliverables:**
- Educational tooltips for dispersion relations
- Interactive wave parameter controls
- Real-time visualization of wave generation

## Testing and Validation Strategy

### Analytical Validation

#### 1. Dispersion Accuracy Tests
- **Linear wave theory**: Validate against analytical celerity
- **Layer convergence**: Verify accuracy improvement with more layers
- **Frequency range**: Test validity limits and cut-off frequencies

#### 2. Wave Propagation Tests
- **Phase speed validation**: Compare computed vs analytical phase speeds
- **Wavelength accuracy**: Verify spatial wave characteristics
- **Amplitude conservation**: Check energy conservation in propagation

#### 3. Boundary Condition Tests
- **Reflection coefficient**: Measure spurious reflections (<5%)
- **Phase alignment**: Verify proper phase relationships
- **Stability**: Long-term simulation stability (>100 wave periods)

### Conservation Property Tests

#### 1. Mass Conservation
```rust
#[test]
fn test_mass_conservation() {
    // Verify ∫ ∂h/∂t dx + ∫ ∂(hu)/∂x dx = 0
    // over computational domain
}
```

#### 2. Energy Conservation
```rust
#[test]
fn test_energy_conservation() {
    // Verify total energy conservation in closed domain
    // E = ∫ (½ρgh² + ½ρhu²) dx
}
```

### Performance Validation

#### 1. Real-time Capability
- **Target**: 30+ FPS with wave generation active
- **Benchmark**: Boundary condition computation time
- **Optimization**: Profile and optimize critical paths

#### 2. Numerical Stability
- **CFL condition**: Verify stability under various wave conditions
- **Long-term stability**: Extended simulations without drift
- **Parameter sensitivity**: Robust behaviour across parameter ranges

## Educational Features

### Progressive Disclosure System

#### 1. Basic Parameters
- **Wave height**: Visual representation with tooltip explanation
- **Wave period**: Connection to frequency and wavelength
- **Water depth**: Relation to wave regime (shallow/deep water)
- **Wave direction**: Fixed left-to-right for 1D channel simplicity

#### 2. Advanced Theory
- **Dispersion relations**: Mathematical derivation accessible via help
- **Layer effects**: Explanation of vertical discretization impact
- **Boundary conditions**: Theory behind wave generation methods

#### 3. Interactive Learning
- **Parameter sliders**: Real-time visualization of changes
- **Comparison modes**: Analytical vs numerical solutions
- **Validation plots**: Error analysis and convergence studies

### Tooltip Content Examples

#### Wave Height Tooltip
```
Wave Height (H)
The vertical distance between wave crest and trough.
In linear theory: H = 2a where a is the amplitude.
For non-linear waves: H affects wave speed and shape.
Click for detailed explanation of wave breaking criteria.
```

#### Dispersion Relation Tooltip
```
Dispersion Relation
Mathematical relationship between wave frequency (ω) and wave number (k).
For shallow water: c = √(gd) (frequency independent)
For deep water: c = √(g/k) (frequency dependent)
SWASH uses layer-specific relations for improved accuracy.
```

## Implementation Priorities

### High Priority (Week 1)
1. **Dispersion solver**: Core wave number computation
2. **Velocity calculator**: Layer-averaged velocities
3. **Boundary integration**: Interface with existing solver
4. **Basic validation**: Analytical comparison tests

### Medium Priority (Week 2)
1. **Educational tooltips**: Mathematical explanations
2. **Parameter controls**: Interactive adjustment
3. **Visualization integration**: Real-time wave display
4. **Performance optimization**: Real-time capability

### Low Priority (Future)
1. **Second-order corrections**: Bound wave suppression
2. **Irregular waves**: Spectral wave generation
3. **2D wave channel**: Extension to multi-directional waves
4. **Advanced validation**: Experimental comparisons

## Success Metrics

### Technical Accuracy
- **Dispersion error**: <1% for valid frequency range
- **Reflection coefficient**: <5% with absorbing boundaries
- **Conservation**: Mass and energy conserved to machine precision
- **Stability**: Stable for >100 wave periods

### Educational Value
- **Accessibility**: Clear explanations for all parameters
- **Interactivity**: Real-time parameter adjustment
- **Progressive learning**: Basic to advanced concept progression
- **Validation**: Comparison with analytical solutions

### Performance
- **Frame rate**: 30+ FPS with active wave generation
- **Memory efficiency**: Minimal allocation in simulation loop
- **Responsiveness**: <100ms parameter update latency
- **Scalability**: Support for different grid sizes

## Risk Assessment and Mitigation

### Technical Risks
1. **Numerical instability**: Mitigated by rigorous CFL condition enforcement
2. **Dispersion accuracy**: Validated against analytical solutions
3. **Boundary reflections**: Implemented weakly reflective conditions
4. **Performance bottlenecks**: Profiling and optimization planned

### Educational Risks
1. **Complexity overload**: Progressive disclosure design
2. **Mathematical accuracy**: Peer review of educational content
3. **User experience**: Iterative design with user feedback
4. **Maintenance burden**: Automated testing of educational features

## Conclusion

This implementation plan provides a structured approach to integrating SWASH-style wave generation into the coastal engineering platform. The focus on equidistant layers with layer-specific dispersion relations ensures both numerical accuracy and educational value. The phased approach allows for iterative development and validation while maintaining the platform's educational mission.

The integration with the existing egui-based interface and the emphasis on progressive disclosure ensures that users can engage with the content at their appropriate level while having access to deeper mathematical understanding when desired.