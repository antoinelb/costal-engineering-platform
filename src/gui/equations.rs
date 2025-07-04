use std::collections::HashMap;
use std::path::Path;
use egui::{ColorImage, TextureHandle, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equation {
    pub id: String,
    pub latex: String,
    pub description: String,
    pub usage: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EquationRegistry {
    equations: Vec<Equation>,
}

pub struct EquationRenderer {
    equations: HashMap<String, Equation>,
    textures: HashMap<String, TextureHandle>,
}

impl EquationRenderer {
    pub fn new() -> Self {
        Self {
            equations: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    /// Load equation definitions from the registry file
    pub fn load_equations(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let equations_path = Path::new("scripts/equations.json");
        
        if !equations_path.exists() {
            return Err("Equations registry file not found".into());
        }

        let content = std::fs::read_to_string(equations_path)?;
        let registry: EquationRegistry = serde_json::from_str(&content)?;
        
        self.equations.clear();
        for equation in registry.equations {
            self.equations.insert(equation.id.clone(), equation);
        }

        Ok(())
    }


    /// Load an SVG equation as a texture
    pub fn load_equation_texture(&mut self, ctx: &Context, equation_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.textures.contains_key(equation_id) {
            return Ok(()); // Already loaded
        }

        let svg_path = format!("assets/equations/{}.svg", equation_id);
        let svg_path = Path::new(&svg_path);

        if !svg_path.exists() {
            return Err(format!("SVG file not found: {}", svg_path.display()).into());
        }

        // Load SVG file as bytes
        let mut svg_bytes = std::fs::read(svg_path)?;
        
        // Get the current text color from the theme
        let text_color = ctx.style().visuals.text_color();
        let color_rgb = format!("rgb({:.1}%, {:.1}%, {:.1}%)", 
                               text_color.r() as f32 / 255.0 * 100.0, 
                               text_color.g() as f32 / 255.0 * 100.0, 
                               text_color.b() as f32 / 255.0 * 100.0);
        
        // Replace black color with current text color
        let svg_string = String::from_utf8(svg_bytes)?;
        let modified_svg = svg_string.replace("rgb(0%, 0%, 0%)", &color_rgb);
        svg_bytes = modified_svg.into_bytes();
        
        // Convert SVG to image using resvg with high DPI for crisp rendering
        use usvg::TreeParsing;
        let mut svg_options = usvg::Options::default();
        svg_options.dpi = 300.0; // High DPI for crisp text rendering
        let svg_tree = usvg::Tree::from_data(&svg_bytes, &svg_options)?;
        let svg_size = svg_tree.size;
        
        // Render at 2x scale for high quality, then scale down in UI
        let scale_factor = 2.0;
        let render_width = (svg_size.width() * scale_factor) as u32;
        let render_height = (svg_size.height() * scale_factor) as u32;
        
        // Create a pixmap to render the SVG
        let mut pixmap = tiny_skia::Pixmap::new(render_width, render_height)
            .ok_or("Failed to create pixmap")?;
        
        // Clear the pixmap with transparent background
        pixmap.fill(tiny_skia::Color::TRANSPARENT);
        
        // Render SVG to pixmap with scaling transform
        let transform = tiny_skia::Transform::from_scale(scale_factor, scale_factor);
        resvg::Tree::from_usvg(&svg_tree).render(transform, &mut pixmap.as_mut());
        
        // Convert pixmap to ColorImage
        let rgba_data = pixmap.data();
        let color_image = ColorImage::from_rgba_unmultiplied(
            [render_width as usize, render_height as usize],
            rgba_data,
        );

        // Create texture from image with high quality settings
        let texture = ctx.load_texture(
            format!("equation_{}", equation_id),
            color_image,
            egui::TextureOptions {
                magnification: egui::TextureFilter::Linear,
                minification: egui::TextureFilter::Linear,
                wrap_mode: egui::TextureWrapMode::ClampToEdge,
                mipmap_mode: None,
            },
        );

        self.textures.insert(equation_id.to_string(), texture);
        Ok(())
    }

    /// Get a texture by equation ID
    pub fn get_texture(&self, equation_id: &str) -> Option<&TextureHandle> {
        self.textures.get(equation_id)
    }

}

impl Default for EquationRenderer {
    fn default() -> Self {
        Self::new()
    }
}