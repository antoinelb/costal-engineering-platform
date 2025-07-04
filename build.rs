use std::process::Command;
use std::path::Path;

fn main() {
    // Tell cargo to rerun this build script if the equations.json file changes
    println!("cargo:rerun-if-changed=scripts/equations.json");
    
    // Also rerun if the rendering script changes
    println!("cargo:rerun-if-changed=scripts/render_equations.py");
    
    // Check if Python is available
    let python_available = Command::new("python3")
        .arg("--version")
        .output()
        .is_ok();
    
    if !python_available {
        println!("cargo:warning=Python3 not found. Equation rendering will be skipped.");
        return;
    }
    
    // Check if tectonic is available
    let tectonic_available = Command::new("tectonic")
        .arg("--version")
        .output()
        .is_ok();
    
    if !tectonic_available {
        println!("cargo:warning=Tectonic not found. Equation rendering will be skipped.");
        println!("cargo:warning=Install tectonic from https://tectonic-typesetting.github.io/");
        return;
    }
    
    // Check if pdftocairo is available
    let pdftocairo_available = Command::new("pdftocairo")
        .arg("-v")
        .output()
        .is_ok();
    
    if !pdftocairo_available {
        println!("cargo:warning=pdftocairo not found. Equation rendering will be skipped.");
        println!("cargo:warning=pdftocairo usually comes with poppler-utils package.");
        return;
    }
    
    // Check if equations.json exists
    let equations_file = Path::new("scripts/equations.json");
    if !equations_file.exists() {
        println!("cargo:warning=equations.json not found. Equation rendering will be skipped.");
        return;
    }
    
    // Run the equation rendering script
    let output = Command::new("python3")
        .arg("scripts/render_equations.py")
        .output();
    
    match output {
        Ok(output) => {
            if !output.status.success() {
                println!("cargo:warning=Equation rendering failed:");
                println!("cargo:warning={}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("cargo:warning=Failed to run equation rendering script: {}", e);
        }
    }
}