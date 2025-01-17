use crate::vec3::Vec3;

#[derive(Clone, Debug)]
pub enum Operation {
    Sum(Box<Operation>, Box<Operation>),
    Product(Box<Operation>, Box<Operation>),
    Mod(Box<Operation>, Box<Operation>),
    Constant(f64),
    VarX,
    VarY,
    Circle(f64, f64),
    Inverse(Box<Operation>),
    PerChannelMask(Box<Operation>, Box<Operation>, Box<Operation>, f64),
    ColorMix(Box<Operation>, Box<Operation>, Box<Operation>),
    BinaryMask(Box<Operation>, Box<Operation>, Box<Operation>, f64),
    SmoothMix(Box<Operation>, Box<Operation>, Box<Operation>),
    Well(Box<Operation>),
    Tent(Box<Operation>),
}

impl Operation {
    pub fn eval(&self, x: f64, y: f64) -> Vec3 {
        match self {
            Operation::Sum(a, b) => a.eval(x, y) + b.eval(x, y),
            Operation::Product(a, b) => a.eval(x, y) * b.eval(x, y),
            Operation::Mod(a, b) => {
                let a_val = a.eval(x, y);
                let b_val = b.eval(x, y);
                Vec3::new(
                    a_val.x().rem_euclid(b_val.x()),
                    a_val.y().rem_euclid(b_val.y()),
                    a_val.z().rem_euclid(b_val.z()),
                )
            }
            Operation::Constant(value) => Vec3::new(*value, *value, *value),
            Operation::VarX => Vec3::new(x, x, x),
            Operation::VarY => Vec3::new(y, y, y),
            Operation::Circle(center_x, center_y) => {
                let val = (x - center_x).hypot(y - center_y);
                Vec3::new(val, val, val)
            }
            Operation::Inverse(a) => Vec3::new(1.0, 1.0, 1.0) - a.eval(x, y),
            Operation::PerChannelMask(m, a, b, threshold) => {
                let m_val = m.eval(x, y);
                let a_val = a.eval(x, y);
                let b_val = b.eval(x, y);
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
            Operation::ColorMix(r, g, b) => {
                Vec3::new(r.eval(x, y).x(), g.eval(x, y).y(), b.eval(x, y).z())
            }
            Operation::BinaryMask(m, a, b, threshold) => {
                let m_val = m.eval(x, y);
                let a_val = a.eval(x, y);
                let b_val = b.eval(x, y);
                if m_val.length() > *threshold {
                    a_val
                } else {
                    b_val
                }
            }
            Operation::SmoothMix(weight, a, b) => {
                let weight_val = weight.eval(x, y).length();
                let a_val = a.eval(x, y);
                let b_val = b.eval(x, y);
                (weight_val * a_val) + ((1.0 - weight_val) * b_val)
            }
            Operation::Well(input) => {
                let input_val = input.eval(x, y);
                Vec3::new(
                    Self::well_fn(input_val.x()),
                    Self::well_fn(input_val.y()),
                    Self::well_fn(input_val.z()),
                )
            }
            Operation::Tent(input) => {
                let input_val = input.eval(x, y);
                Vec3::new(
                    Self::tent_fn(input_val.x()),
                    Self::tent_fn(input_val.y()),
                    Self::tent_fn(input_val.z()),
                )
            }
        }
    }

    fn well_fn(x: f64) -> f64 {
        (1.0 - 2.0 / (1.0 + x * x)).powi(8)
    }

    fn tent_fn(x: f64) -> f64 {
        1.0 - 2.0 * x.abs()
    }

    pub fn get_arity(&self) -> usize {
        match self {
            Operation::Sum(_, _) => 2,
            Operation::Product(_, _) => 2,
            Operation::Mod(_, _) => 2,
            Operation::Constant(_) => 0,
            Operation::VarX => 0,
            Operation::VarY => 0,
            Operation::Circle(_, _) => 0,
            Operation::Inverse(_) => 1,
            Operation::PerChannelMask(_, _, _, _) => 3,
            Operation::ColorMix(_, _, _) => 3,
            Operation::BinaryMask(_, _, _, _) => 3,
            Operation::SmoothMix(_, _, _) => 3,
            Operation::Well(_) => 1,
            Operation::Tent(_) => 1,
        }
    }
}
