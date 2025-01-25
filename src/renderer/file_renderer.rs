use crate::operations::Operation;
use crate::utils::{write_image, ImageWriteError};

pub struct FileRenderer {
    output_path: String,
}
impl FileRenderer {
    pub fn new(output_path: String) -> Self {
        FileRenderer { output_path }
    }

    pub fn render(
        &mut self,
        x_res: usize,
        y_res: usize,
        root: &Operation,
    ) -> Result<(), ImageWriteError> {
        let mut values = Vec::with_capacity(x_res * y_res);
        let t = 0.0;

        for y in 0..y_res {
            for x in 0..x_res {
                let fx = 0.0 + (x as f64 / x_res as f64) * (1.0 - 0.0);
                let fy = 0.0 + (y as f64 / y_res as f64) * (1.0 - 0.0);
                let color = root.eval(fx, fy, t);
                values.push(color);
            }
        }
        write_image(&self.output_path, x_res, y_res, &values)
    }
}
