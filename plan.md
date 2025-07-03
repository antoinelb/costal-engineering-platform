# Development Plan: 1D Wave Channel Simulator

## Project Overview
Create a native Rust coastal engineering platform that replicates core SWASH functionality for 1D wave propagation. Focus on educational value while maintaining numerical accuracy. The goal is for the education content to be hidden on the platform, but for a user to always be able to investigate the details behind something. For example, a result for significant wave height where clicking on a little question mark opens a box explaining what that is and how it's calculated. 

## Technical Foundation
- **Core Equations**: Non-hydrostatic shallow water equations with pressure splitting
- **Numerical Method**: Staggered finite difference grid (mass-momentum conservative)
- **Architecture**: Modular Rust simulation engine with native egui-based professional UI
- **GUI Framework**: egui chosen for high-performance 3D visualization, platform robustness, and native cross-platform deployment

## Mathematical Framework

### 1D Shallow Water Equations
- Continuity equation: $\frac{\partial h}{\partial t} + \frac{\partial (hu)}{\partial x} = 0$
- Momentum equation: $\frac{\partial (hu)}{\partial t} + \frac{\partial (hu^2)}{\partial x} + gh\frac{\partial h}{\partial x} = -\frac{\partial p}{\partial x}$

Where:
- $h$ is water depth
- $u$ is horizontal velocity
- $g$ is gravitational acceleration
- $p$ is non-hydrostatic pressure

### Staggered Grid Discretization
- Water levels ($h$) defined at cell centers: $i = 0, 1, ..., N-1$
- Velocities ($u$) defined at cell faces: $i+1/2 = 0.5, 1.5, ..., N-0.5$
- Time stepping with CFL condition: $\Delta t \leq \frac{\Delta x}{\sqrt{gh_{max}}}$

## Week-Long Development Timeline

### Days 1-2: Foundation & Core Numerics
- Set up project structure with dependencies (ndarray, egui, wgpu, three-d)
- Implement `Grid1D` struct with staggered grid layout
- Create core `WaveSolver` with shallow water equations
- Basic conservation tests (mass and momentum)

**Key Deliverables:**
- Working 1D grid system
- Basic shallow water solver (hydrostatic approximation)
- Unit tests for conservation properties

### Days 3-4: Wave Physics & Boundary Conditions  
- Add non-hydrostatic pressure correction for frequency dispersion
- Implement `RegularWaves` boundary condition for wave generation
- Add absorbing boundary conditions at domain exit
- Validate against analytical solutions (progressive waves)

**Key Deliverables:**
- Dispersive wave propagation
- Wave generation and absorption
- Validation against linear wave theory

### Days 5-6: GUI & Visualization
- Build real-time egui interface with parameter controls
- Implement surface elevation plotting with egui_plot
- Integrate basic 3D wave visualization with three-d
- Add play/pause/reset simulation controls
- Interactive parameter adjustment (wave height, period, depth)

**Key Deliverables:**
- Real-time visualization
- Interactive parameter controls
- User-friendly interface

### Day 7: Validation & Documentation
- Create educational examples (standing waves, shoaling)
- Performance optimization and stability testing
- Documentation with coastal engineering explanations
- Validation against known analytical solutions

**Key Deliverables:**
- Educational examples
- Performance benchmarks
- Complete documentation

## System Architecture

### Core Modules

#### 1. Grid Module (`src/grid.rs`)
```rust
struct Grid1D {
    nx: usize,           // Number of cells
    dx: f64,             // Grid spacing
    x_centers: Vec<f64>, // Cell center coordinates
    x_faces: Vec<f64>,   // Cell face coordinates
}
```

#### 2. Wave Solver Module (`src/solver.rs`)
```rust
struct WaveSolver {
    grid: Grid1D,
    h: Vec<f64>,      // Water depth at cell centers
    u: Vec<f64>,      // Velocity at cell faces
    p_nh: Vec<f64>,   // Non-hydrostatic pressure
}
```

#### 3. Boundary Conditions Module (`src/boundary.rs`)
```rust
trait BoundaryCondition {
    fn apply(&self, solver: &mut WaveSolver, time: f64);
}

struct RegularWaves {
    amplitude: f64,
    period: f64,
    depth: f64,
}
```

#### 4. Simulation Module (`src/simulation.rs`)
```rust
struct Simulation {
    solver: WaveSolver,
    boundary_conditions: Vec<Box<dyn BoundaryCondition>>,
    time: f64,
    dt: f64,
}
```

#### 5. GUI Module (`src/gui.rs`)
```rust
struct WaveChannelApp {
    simulation: Simulation,
    plot_data: PlotData,
    control_panel: ControlPanel,
}
```

## Key Dependencies

### Native Application (Cargo.toml)
```toml
[dependencies]
ndarray = "0.15"           # Grid storage and operations
nalgebra = "0.32"          # Linear algebra for pressure solver
serde = { version = "1.0", features = ["derive"] }  # Data serialization
anyhow = "1.0"             # Error handling
egui = "0.24"              # Immediate mode GUI framework
eframe = "0.24"            # egui application framework
wgpu = "0.18"              # Direct GPU access for 3D rendering
three-d = "0.16"           # 3D graphics engine
plotters = "0.3"           # 2D plotting for scientific charts
tokio = { version = "1.0", features = ["full"] }     # Async runtime
```

## Why egui for Professional Platform

**High-Performance 3D Visualization:**
- **Direct GPU access**: wgpu provides zero-overhead graphics programming
- **Native performance**: No webview serialization bottlenecks for large datasets
- **Real-time capability**: Immediate mode GUI perfect for live wave simulations
- **Memory efficiency**: Direct control over GPU buffers and rendering pipeline
- **Custom 3D integration**: three-d engine for complex wave field visualizations

**Platform Robustness:**
- **Production-proven**: Used in professional engineering and scientific applications
- **Memory safety**: Full Rust stack eliminates entire classes of runtime errors
- **Simple architecture**: Single native binary, no complex frontend/backend coordination
- **Cross-platform**: Native compilation for Windows, macOS, Linux
- **Web deployment**: Same code compiles to WASM for browser deployment

**Professional Engineering Requirements:**
- **Scientific visualization**: Proven in tools like Rerun.io for complex 3D data
- **Educational integration**: Rich tooltip system with progressive disclosure
- **Real-time interaction**: Immediate parameter updates with live visualization
- **Performance predictability**: Consistent behavior under computational load
- **Single interface**: Progressive disclosure without separate modes

## Educational Value

### Wave Theory Fundamentals
- **Linear Wave Theory**: Demonstrate celerity = $\sqrt{gh}$ in shallow water
- **Frequency Dispersion**: Show how different wavelengths travel at different speeds
- **Wave Shoaling**: Visualize wave changes as water depth decreases
- **Wave Breaking**: Demonstrate when wave height becomes too large relative to depth

### Numerical Methods Education
- **Finite Difference Approximations**: Show discretization accuracy effects
- **Stability Criteria**: Demonstrate CFL condition importance
- **Conservation Properties**: Visualize mass and momentum conservation
- **Grid Resolution Effects**: Show how grid spacing affects accuracy

### Interactive Features for Learning
- Real-time parameter adjustment to see immediate effects
- Visualization of both water surface and velocity fields
- **Educational tooltips**: Contextual help for all parameters and results (e.g., "?" icons)
- **Progressive disclosure**: Professional interface with optional educational content
- Ability to compare with analytical solutions
- Display of key parameters (wave celerity, wavelength, etc.) with explanations

## Validation Cases

### Test Scenarios
1. **Standing Waves**: Reflection from a wall boundary
2. **Progressive Waves**: Constant depth wave propagation
3. **Shoaling Waves**: Wave transformation over depth transitions
4. **Wave Breaking**: Critical wave height scenarios

### Analytical Comparisons
- Linear wave theory for small amplitude waves
- Shallow water wave celerity validation
- Energy conservation checks
- Frequency dispersion accuracy

## Performance Considerations

### Optimization Targets
- Real-time visualization ($30+$ FPS)
- Efficient memory usage for large grids
- Direct GPU rendering for 3D wave fields
- Parallel computation for pressure solver
- Adaptive time stepping for stability

### Computational Efficiency
- Vectorized operations using ndarray
- Minimal memory allocations in time loop
- Direct GPU memory management with wgpu
- Efficient 3D rendering pipeline
- Zero-copy data transfer between simulation and visualization

## Success Metrics

### Technical Objectives
- Accurate wave propagation matching analytical solutions
- Stable numerical scheme with proper conservation
- Interactive GUI with responsive controls
- Educational value through clear visualization

### Educational Objectives
- Clear demonstration of wave physics principles
- Interactive learning through parameter experimentation
- Validation examples with known solutions
- Foundation for advanced coastal engineering concepts

## Future Extensions

### Potential Enhancements
- 2D wave propagation with advanced 3D visualization
- Wave-structure interaction with real-time rendering
- Sediment transport with particle visualization
- Irregular wave spectra with spectral analysis tools
- Tidal forcing with long-term simulation capabilities
- Wave breaking models with detailed 3D flow visualization

## Testing Strategy

### Frontend GUI Testing with egui

#### Primary Testing Framework: egui_kittest
- **Purpose**: Comprehensive GUI testing framework built specifically for egui applications
- **Features**: Accessibility-first testing using AccessKit, direct widget interaction simulation
- **Usage**: Test user interface components, parameter controls, and simulation state management

#### Visual/Screenshot Testing: egui-screenshot-testing
- **Purpose**: Regression testing for 3D wave visualization and UI layouts
- **Features**: Automated screenshot comparison, visual validation of simulation outputs
- **Usage**: Test wave visualization accuracy, UI consistency, and educational tooltip rendering

### Scientific Validation Testing

#### Analytical Comparison Tests
- **Purpose**: Validate numerical accuracy against known analytical solutions
- **Coverage**: Linear wave theory, shallow water equations, conservation properties

#### Conservation Property Tests
- **Purpose**: Ensure mass and momentum conservation in numerical scheme
- **Validation**: Long-term stability, numerical accuracy, physical correctness

### Performance and Integration Testing

#### Real-time Performance Tests
- **Purpose**: Ensure simulation meets real-time requirements (30+ FPS)
- **Metrics**: Simulation step timing, memory usage, GPU utilization

#### Educational Feature Testing
- **Purpose**: Validate educational tooltips, progressive disclosure, and help systems
- **Coverage**: Tooltip accuracy, mathematical explanations, user experience flows

### Testing Dependencies

```toml
[dev-dependencies]
egui_kittest = "0.30"           # GUI testing framework
egui-screenshot-testing = "0.1" # Visual regression testing
approx = "0.5"                  # Floating point comparison
criterion = "0.5"               # Performance benchmarking
```

### Test Organization

```
tests/
├── unit/                       # Unit tests for individual modules
│   ├── grid_tests.rs
│   ├── solver_tests.rs
│   └── boundary_tests.rs
├── integration/                # Integration tests
│   ├── simulation_tests.rs
│   └── gui_tests.rs
├── visual/                     # Visual regression tests
│   ├── wave_visualization.rs
│   └── ui_layout.rs
├── performance/                # Performance benchmarks
│   └── simulation_benchmarks.rs
└── validation/                 # Scientific validation
    ├── analytical_comparison.rs
    └── conservation_tests.rs
```

This comprehensive testing strategy ensures both technical correctness and educational value while maintaining the high performance requirements of real-time coastal engineering simulation.

This plan balances technical rigour with educational utility, creating a foundation for more advanced coastal engineering simulations while being achievable in one week.
