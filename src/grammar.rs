use crate::operations::Operation;
use rand::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrammarError {
    #[error("No internal operations available in grammar")]
    NoInternalOperations,

    #[error("No leaf operations available in grammar")]
    NoLeafOperations,

    #[error("Could not choose a constant")]
    ConstantSelectionError,

    #[error("Error building expression tree")]
    TreeBuildError,
}

pub struct Grammar {
    internal_ops: Vec<fn(&mut Grammar) -> Result<Operation, GrammarError>>,
    leaf_ops: Vec<fn(&mut Grammar) -> Result<Operation, GrammarError>>,
    rng: StdRng,
    constants: Vec<f64>,
}

impl Grammar {
    pub fn new(seed: u64) -> Result<Self, GrammarError> {
        let mut rng = StdRng::seed_from_u64(seed);

        let num_constants = 10;
        let constants: Vec<f64> = (0..num_constants).map(|_| rng.gen()).collect();

        let internal_ops: Vec<fn(&mut Grammar) -> Result<Operation, GrammarError>> = vec![
            |g: &mut Grammar| {
                Ok(Operation::Sum(
                    g.build_tree(0)?.into(),
                    g.build_tree(0)?.into(),
                ))
            },
            |g: &mut Grammar| {
                Ok(Operation::PerChannelMask(
                    g.build_tree(0)?.into(),
                    g.build_tree(0)?.into(),
                    g.build_tree(0)?.into(),
                    g.get_constant()?,
                ))
            },
            |g: &mut Grammar| {
                Ok(Operation::ColorMix(
                    g.build_tree(0)?.into(),
                    g.build_tree(0)?.into(),
                    g.build_tree(0)?.into(),
                ))
            },
            |g: &mut Grammar| {
                Ok(Operation::BinaryMask(
                    g.build_tree(0)?.into(),
                    g.build_tree(0)?.into(),
                    g.build_tree(0)?.into(),
                    g.get_constant()?,
                ))
            },
            |g: &mut Grammar| {
                Ok(Operation::SmoothMix(
                    g.build_tree(0)?.into(),
                    g.build_tree(0)?.into(),
                    g.build_tree(0)?.into(),
                ))
            },
            |g: &mut Grammar| Ok(Operation::Well(g.build_tree(0)?.into())),
            |g: &mut Grammar| Ok(Operation::Tent(g.build_tree(0)?.into())),
            |g: &mut Grammar| {
                Ok(Operation::Product(
                    g.build_tree(0)?.into(),
                    g.build_tree(0)?.into(),
                ))
            },
            |g: &mut Grammar| Ok(Operation::Inverse(g.build_tree(0)?.into())),
            |g: &mut Grammar| {
                Ok(Operation::Mod(
                    g.build_tree(0)?.into(),
                    g.build_tree(0)?.into(),
                ))
            },
        ];

        if internal_ops.is_empty() {
            return Err(GrammarError::NoInternalOperations);
        }

        let leaf_ops: Vec<fn(&mut Grammar) -> Result<Operation, GrammarError>> = vec![
            |g: &mut Grammar| Ok(Operation::Constant(g.get_constant()?)),
            |_g: &mut Grammar| Ok(Operation::VarX),
            |_g: &mut Grammar| Ok(Operation::VarY),
            |g: &mut Grammar| Ok(Operation::Circle(g.get_constant()?, g.get_constant()?)),
        ];

        if leaf_ops.is_empty() {
            return Err(GrammarError::NoLeafOperations);
        }

        Ok(Grammar {
            internal_ops,
            leaf_ops,
            rng,
            constants,
        })
    }

    fn rand_op(&mut self) -> Result<Operation, GrammarError> {
        self.internal_ops
            .choose_mut(&mut self.rng)
            .copied()
            .map(|op_fn| op_fn(self))
            .ok_or(GrammarError::NoInternalOperations)?
    }

    fn rand_leaf(&mut self) -> Result<Operation, GrammarError> {
        self.leaf_ops
            .choose_mut(&mut self.rng)
            .copied()
            .map(|op_fn| op_fn(self))
            .ok_or(GrammarError::NoLeafOperations)?
    }

    pub fn get_constant(&mut self) -> Result<f64, GrammarError> {
        self.constants
            .choose(&mut self.rng)
            .copied()
            .ok_or(GrammarError::ConstantSelectionError)
    }

    pub fn build_tree(&mut self, depth: usize) -> Result<Operation, GrammarError> {
        if depth == 0 {
            self.rand_leaf()
        } else {
            let mut op = self.rand_op()?;
            match &mut op {
                Operation::Sum(a, b) => {
                    **a = self.build_tree(depth - 1)?;
                    **b = self.build_tree(depth - 1)?;
                }
                Operation::Product(a, b) => {
                    **a = self.build_tree(depth - 1)?;
                    **b = self.build_tree(depth - 1)?;
                }
                Operation::Mod(a, b) => {
                    **a = self.build_tree(depth - 1)?;
                    **b = self.build_tree(depth - 1)?;
                }
                Operation::Inverse(a) => **a = self.build_tree(depth - 1)?,
                Operation::PerChannelMask(m, a, b, _) => {
                    **m = self.build_tree(depth - 1)?;
                    **a = self.build_tree(depth - 1)?;
                    **b = self.build_tree(depth - 1)?;
                }
                Operation::ColorMix(r, g, b) => {
                    **r = self.build_tree(depth - 1)?;
                    **g = self.build_tree(depth - 1)?;
                    **b = self.build_tree(depth - 1)?;
                }
                Operation::BinaryMask(m, a, b, _) => {
                    **m = self.build_tree(depth - 1)?;
                    **a = self.build_tree(depth - 1)?;
                    **b = self.build_tree(depth - 1)?;
                }
                Operation::SmoothMix(w, a, b) => {
                    **w = self.build_tree(depth - 1)?;
                    **a = self.build_tree(depth - 1)?;
                    **b = self.build_tree(depth - 1)?;
                }
                Operation::Well(a) => **a = self.build_tree(depth - 1)?,
                Operation::Tent(a) => **a = self.build_tree(depth - 1)?,
                _ => {}
            }

            Ok(op)
        }
    }
}
