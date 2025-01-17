use crate::operations::Operation;
use crate::renderer::{RenderError, Renderer};
use crate::vec3::Vec3;

pub struct CpuRenderer;

impl Renderer for CpuRenderer {
    fn render(
        &mut self,
        x_res: usize,
        y_res: usize,
        min_x: f64,
        max_x: f64,
        min_y: f64,
        max_y: f64,
        root: &Operation,
    ) -> Result<Vec<Vec3>, RenderError> {
        let mut values = Vec::with_capacity(x_res * y_res);

        for y in 0..y_res {
            for x in 0..x_res {
                let fx = min_x + (x as f64 / x_res as f64) * (max_x - min_x);
                let fy = min_y + (y as f64 / y_res as f64) * (max_y - min_y);
                let color = root.eval(fx, fy);
                values.push(color);
            }
        }

        Ok(values)
    }
}
