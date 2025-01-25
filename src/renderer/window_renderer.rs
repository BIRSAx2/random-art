use crate::operations::Operation;
use macroquad::camera::set_default_camera;
use macroquad::color::WHITE;
use macroquad::material::{gl_use_material, load_material, MaterialParams};
use macroquad::miniquad::window::set_window_size;
use macroquad::miniquad::{Comparison, PipelineParams, ShaderSource, UniformDesc, UniformType};
use macroquad::prelude::{clear_background, draw_rectangle, get_time, next_frame};
use std::process::exit;

pub struct WindowRenderer {}

impl Default for WindowRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowRenderer {
    pub fn new() -> Self {
        WindowRenderer {}
    }
    pub async fn render(&mut self, x_res: usize, y_res: usize, root: &Operation) {
        loop {
            clear_background(WHITE);
            set_window_size(x_res as u32, y_res as u32);
            let vertex_shader = DEFAULT_VERTEX_SHADER.to_string();
            let pipeline_params = PipelineParams {
                depth_write: true,
                depth_test: Comparison::LessOrEqual,
                ..Default::default()
            };
            let glsl = root.to_glsl();
            let fragment_shader = DEFAULT_FRAGMENT_SHADER.replace("#REPLACE_ME#", &glsl);
            let uniforms = vec![UniformDesc::new("time", UniformType::Float1)];

            let material = match load_material(
                ShaderSource::Glsl {
                    vertex: &vertex_shader,
                    fragment: &fragment_shader,
                },
                MaterialParams {
                    pipeline_params,
                    uniforms,
                    ..Default::default()
                },
            ) {
                Ok(mat) => mat,
                Err(err) => {
                    eprintln!("Error loading material: {:?}", err);
                    exit(1);
                }
            };

            material.set_uniform("time", get_time() as f32);

            gl_use_material(&material);

            draw_rectangle(-1.0, -1.0, 2.0, 2.0, WHITE);

            set_default_camera();

            material.set_uniform("time", get_time() as f32);

            next_frame().await
        }
    }
}

const DEFAULT_FRAGMENT_SHADER: &str = r#"
#version 330
in vec2 fragTexCoord;
out vec4 finalColor;
uniform float time;

vec4 map_color(vec3 rgb) {
    return vec4((rgb +1)/2.0 , 1.0);
}

float well_fn(float x) {
    return pow(1.0 - 2.0 / (1.0 + x * x), 8);
}

float tent_fn(float x) {
    return 1.0 - 2.0 * abs(x);
}

void main() {
    float x = fragTexCoord.x * 2.0 - 1.0;
    float y = fragTexCoord.y * 2.0 - 1.0;
    float t = sin(time);
    finalColor = map_color(#REPLACE_ME#);
}
"#;

const DEFAULT_VERTEX_SHADER: &str = r#"
#version 330 core

in vec3 position;
in vec2 texcoord;

out vec2 fragTexCoord;

void main() {
    gl_Position = vec4(position, 1);
    fragTexCoord = texcoord;
}
"#;
