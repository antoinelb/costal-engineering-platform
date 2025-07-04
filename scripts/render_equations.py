#!/usr/bin/env python3
"""
Render LaTeX equations to SVG using tectonic and dvisvgm.
Only renders equations that don't already have SVG files.
"""

import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Dict, List

# Template for standalone LaTeX documents
LATEX_TEMPLATE = r"""
\documentclass[12pt]{{standalone}}
\usepackage{{amsmath}}
\usepackage{{amssymb}}
\usepackage{{mathtools}}
\begin{{document}}
${equation}$
\end{{document}}
"""

def load_equations(equations_file: Path) -> List[Dict]:
    """Load equation definitions from JSON file."""
    try:
        with open(equations_file, 'r') as f:
            data = json.load(f)
        return data.get('equations', [])
    except FileNotFoundError:
        print(f"Error: Equations file not found: {equations_file}")
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"Error parsing equations file: {e}")
        sys.exit(1)

def check_dependencies():
    """Check if required tools are available."""
    missing = []
    
    # Check tectonic
    try:
        subprocess.run(['tectonic', '--version'], 
                     capture_output=True, check=True)
    except (subprocess.CalledProcessError, FileNotFoundError):
        missing.append('tectonic')
    
    # Check pdftocairo (uses -v instead of --version)
    try:
        subprocess.run(['pdftocairo', '-v'], 
                     capture_output=True, check=True)
    except (subprocess.CalledProcessError, FileNotFoundError):
        missing.append('pdftocairo')
    
    if missing:
        print(f"Error: Missing required tools: {', '.join(missing)}")
        print("Please install:")
        for tool in missing:
            if tool == 'tectonic':
                print("  - tectonic: https://tectonic-typesetting.github.io/")
            elif tool == 'pdftocairo':
                print("  - pdftocairo: Usually comes with poppler-utils package")
        sys.exit(1)

def render_equation(equation: Dict, output_dir: Path, temp_dir: Path) -> bool:
    """Render a single equation to SVG. Returns True if successful."""
    equation_id = equation['id']
    latex_code = equation['latex']
    output_file = output_dir / f"{equation_id}.svg"
    
    # Skip if SVG already exists
    if output_file.exists():
        print(f"Skipping {equation_id} (SVG already exists)")
        return True
    
    print(f"Rendering {equation_id}...")
    
    # Create LaTeX file
    latex_content = LATEX_TEMPLATE.format(equation=latex_code)
    tex_file = temp_dir / f"{equation_id}.tex"
    pdf_file = temp_dir / f"{equation_id}.pdf"
    
    try:
        # Write LaTeX file
        with open(tex_file, 'w') as f:
            f.write(latex_content)
        
        # Run tectonic to generate PDF
        result = subprocess.run([
            'tectonic', 
            '--outdir', str(temp_dir),
            str(tex_file)
        ], capture_output=True, text=True)
        
        if result.returncode != 0:
            print(f"Error running tectonic for {equation_id}:")
            print(result.stderr)
            return False
        
        # Check if PDF file was created
        if not pdf_file.exists():
            print(f"Error: PDF file not created for {equation_id}")
            return False
        
        # Run pdftocairo to convert PDF to SVG
        svg_file = temp_dir / f"{equation_id}.svg"
        result = subprocess.run([
            'pdftocairo',
            '-svg',        # Output SVG format
            str(pdf_file),
            str(svg_file)
        ], capture_output=True, text=True)
        
        if result.returncode != 0:
            print(f"Error running pdftocairo for {equation_id}:")
            print(result.stderr)
            return False
        
        # Move SVG to output directory
        if svg_file.exists():
            svg_file.rename(output_file)
            print(f"Successfully rendered {equation_id}")
            return True
        else:
            print(f"Error: SVG file not created for {equation_id}")
            return False
            
    except Exception as e:
        print(f"Error rendering {equation_id}: {e}")
        return False
    finally:
        # Clean up temporary files
        for ext in ['.tex', '.pdf', '.log', '.aux']:
            temp_file = temp_dir / f"{equation_id}{ext}"
            if temp_file.exists():
                temp_file.unlink()

def main():
    """Main function to render all equations."""
    # Get script directory
    script_dir = Path(__file__).parent
    project_dir = script_dir.parent
    
    # Set up paths
    equations_file = script_dir / 'equations.json'
    output_dir = project_dir / 'assets' / 'equations'
    temp_dir = script_dir / 'temp'
    
    # Create directories
    output_dir.mkdir(parents=True, exist_ok=True)
    temp_dir.mkdir(exist_ok=True)
    
    # Check dependencies
    check_dependencies()
    
    # Load equations
    equations = load_equations(equations_file)
    
    if not equations:
        print("No equations found in equations.json")
        return
    
    print(f"Found {len(equations)} equations to process")
    
    # Render equations
    successful = 0
    failed = 0
    
    for equation in equations:
        if render_equation(equation, output_dir, temp_dir):
            successful += 1
        else:
            failed += 1
    
    # Clean up temp directory
    if temp_dir.exists():
        import shutil
        shutil.rmtree(temp_dir)
    
    # Report results
    print(f"\nRendering complete:")
    print(f"  Successful: {successful}")
    print(f"  Failed: {failed}")
    print(f"  SVG files saved to: {output_dir}")
    
    if failed > 0:
        sys.exit(1)

if __name__ == '__main__':
    main()