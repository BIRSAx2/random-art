use crate::operations::Operation;
use crate::renderer::{RenderError, Renderer};
use crate::vec3::Vec3;

pub struct GpuRenderer {}

impl Renderer for GpuRenderer {
    fn render(
        &mut self,
        _x_res: usize,
        _y_res: usize,
        _min_x: f64,
        _max_x: f64,
        _min_y: f64,
        _max_y: f64,
        _root: &Operation,
    ) -> Result<Vec<Vec3>, RenderError> {
        todo!("Implement GPU renderer")
    }
}
