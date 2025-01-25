use crate::vec3::Vec3;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum Operation {
    Sum(Box<Operation>, Box<Operation>),
    Product(Box<Operation>, Box<Operation>),
    Mod(Box<Operation>, Box<Operation>),
    Constant(f64),
    VarX,
    VarY,
    VarT,
    Circle(f64, f64),
    Sine(Box<Operation>),
    Inverse(Box<Operation>),
    PerChannelMask(Box<Operation>, Box<Operation>, Box<Operation>, f64),
    BinaryMask(Box<Operation>, Box<Operation>, Box<Operation>, f64),
    SmoothMix(Box<Operation>, Box<Operation>, Box<Operation>),
    Well(Box<Operation>),
    Tent(Box<Operation>),
    RGB(Box<Operation>, Box<Operation>, Box<Operation>),
}

impl Operation {
    pub fn eval(&self, x: f64, y: f64, t: f64) -> Vec3 {
        match self {
            Operation::Sum(a, b) => a.eval(x, y, t) + b.eval(x, y, t),
            Operation::Product(a, b) => a.eval(x, y, t) * b.eval(x, y, t),
            Operation::Mod(a, b) => {
                let a_val = a.eval(x, y, t);
                let b_val = b.eval(x, y, t);
                Vec3::new(
                    a_val.x().rem_euclid(b_val.x()),
                    a_val.y().rem_euclid(b_val.y()),
                    a_val.z().rem_euclid(b_val.z()),
                )
            }
            Operation::Constant(value) => Vec3::new(*value, *value, *value),
            Operation::VarX => Vec3::new(x, x, x),
            Operation::VarY => Vec3::new(y, y, y),
            Operation::VarT => Vec3::new(t, t, t),
            Operation::Circle(center_x, center_y) => {
                let val = (x - center_x).hypot(y - center_y);
                Vec3::new(val, val, val)
            }
            Operation::Sine(a) => a.eval(x, y, t).map(|v| v.sin()),
            Operation::Inverse(a) => Vec3::new(0.0, 0.0, 0.0) - a.eval(x, y, t),
            Operation::PerChannelMask(m, a, b, threshold) => {
                let m_val = m.eval(x, y, t);
                let a_val = a.eval(x, y, t);
                let b_val = b.eval(x, y, t);
                Vec3::new(
                    if m_val.x() > *threshold {
                        a_val.x()
                    } else {
                        b_val.x()
                    },
                    if m_val.y() > *threshold {
                        a_val.y()
                    } else {
                        b_val.y()
                    },
                    if m_val.z() > *threshold {
                        a_val.z()
                    } else {
                        b_val.z()
                    },
                )
            }

            Operation::BinaryMask(m, a, b, threshold) => {
                let m_val = m.eval(x, y, t);
                let a_val = a.eval(x, y, t);
                let b_val = b.eval(x, y, t);
                if m_val.length() > *threshold {
                    a_val
                } else {
                    b_val
                }
            }
            Operation::SmoothMix(weight, a, b) => {
                let weight_val = weight.eval(x, y, t).length();
                let a_val = a.eval(x, y, t);
                let b_val = b.eval(x, y, t);
                (weight_val * a_val) + ((1.0 - weight_val) * b_val)
            }
            Operation::Well(input) => {
                let input_val = input.eval(x, y, t);
                Vec3::new(
                    Self::well_fn(input_val.x()),
                    Self::well_fn(input_val.y()),
                    Self::well_fn(input_val.z()),
                )
            }
            Operation::Tent(input) => {
                let input_val = input.eval(x, y, t);
                Vec3::new(
                    Self::tent_fn(input_val.x()),
                    Self::tent_fn(input_val.y()),
                    Self::tent_fn(input_val.z()),
                )
            }
            Operation::RGB(r, g, b) => Vec3::new(
                r.eval(x, y, t).x(),
                g.eval(x, y, t).y(),
                b.eval(x, y, t).z(),
            ),
        }
    }

    pub fn to_glsl(&self) -> String {
        match self {
            Operation::Sum(a, b) => format!("(({}) + ({}))", a.to_glsl(), b.to_glsl()),
            Operation::Product(a, b) => format!("({} * {})", a.to_glsl(), b.to_glsl()),
            Operation::Mod(a, b) => format!("mod({}, {})", a.to_glsl(), b.to_glsl()),
            Operation::Constant(value) => format!("vec3({}, {}, {})", value, value, value),
            Operation::VarX => "vec3(x,x,x)".to_string(),
            Operation::VarY => "vec3(y,y,y)".to_string(),
            Operation::VarT => "vec3(t,t,t)".to_string(),
            Operation::Circle(cx, cy) => {
                format!("vec3(distance(vec2(x, y), vec2({}, {})))", cx, cy)
            }
            Operation::Inverse(a) => format!("(vec3(0.0, 0.0, 0.0) - {})", a.to_glsl()),
            Operation::PerChannelMask(m, a, b, threshold) => {
                let m = m.to_glsl();
                let a = a.to_glsl();
                let b = b.to_glsl();
                format!(
                    "vec3(({}.x > {} ? {}.x : {}.x), ({}.y > {} ? {}.y : {}.y), ({}.z > {} ? {}.z : {}.z))",
                    m, threshold, a, b, m, threshold, a, b, m, threshold, a, b
                )
            }
            Operation::Sine(a) => format!("sin({})", a.to_glsl()),

            Operation::BinaryMask(m, a, b, threshold) => {
                let m = m.to_glsl();
                let a = a.to_glsl();
                let b = b.to_glsl();
                format!("(length({}) > {} ? {} : {})", m, threshold, a, b)
            }

            Operation::SmoothMix(weight, a, b) => format!(
                "(({} * {}) + ((1.0 - {}) * {}))",
                weight.to_glsl(),
                a.to_glsl(),
                weight.to_glsl(),
                b.to_glsl()
            ),
            Operation::Well(a) => format!(
                "vec3(well_fn({}.x), well_fn({}.y), well_fn({}.z))",
                a.to_glsl(),
                a.to_glsl(),
                a.to_glsl()
            ),
            Operation::Tent(a) => format!(
                "vec3(tent_fn({}.x), tent_fn({}.y), tent_fn({}.z))",
                a.to_glsl(),
                a.to_glsl(),
                a.to_glsl()
            ),
            Operation::RGB(r, g, b) => format!(
                "vec3({}.x, {}.y, {}.z)",
                r.to_glsl(),
                g.to_glsl(),
                b.to_glsl()
            ),
        }
    }

    fn well_fn(x: f64) -> f64 {
        (1.0 - 2.0 / (1.0 + x * x)).powi(8)
    }

    fn tent_fn(x: f64) -> f64 {
        1.0 - 2.0 * x.abs()
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Sum(a, b) => write!(f, "({} + {})", a, b),
            Operation::Product(a, b) => write!(f, "({} * {})", a, b),
            Operation::Mod(a, b) => write!(f, "({} % {})", a, b),
            Operation::Constant(value) => write!(f, "{}", value),
            Operation::VarX => write!(f, "x"),
            Operation::VarY => write!(f, "y"),
            Operation::VarT => write!(f, "t"),
            Operation::Circle(center_x, center_y) => {
                write!(f, "circle({}, {})", center_x, center_y)
            }
            Operation::Sine(a) => write!(f, "sin({})", a),
            Operation::Inverse(a) => write!(f, "-{}", a),
            Operation::PerChannelMask(m, a, b, threshold) => {
                write!(f, "per_channel_mask({}, {}, {}, {})", m, a, b, threshold)
            }
            Operation::BinaryMask(m, a, b, threshold) => {
                write!(f, "binary_mask({}, {}, {}, {})", m, a, b, threshold)
            }
            Operation::SmoothMix(weight, a, b) => write!(f, "smooth_mix({}, {}, {})", weight, a, b),
            Operation::Well(a) => write!(f, "well({})", a),
            Operation::Tent(a) => write!(f, "tent({})", a),
            Operation::RGB(r, g, b) => write!(f, "rgb({}, {}, {})", r, g, b),
        }
    }
}
