use crate::operations::Operation;
use crate::vec3::Vec3;

pub mod cpu;
mod gpu;

pub trait Renderer {
    fn render(
        &mut self,
        x_res: usize,
        y_res: usize,
        min_x: f64,
        max_x: f64,
        min_y: f64,
        max_y: f64,
        root: &Operation,
    ) -> Result<Vec<Vec3>, RenderError>;
}

#[derive(Debug)]
pub enum RenderError {
    GpuError(String),
    CpuError(String),
}
