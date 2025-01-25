use crate::operations::Operation;
use rand::prelude::*;

pub trait ArtGrammar {
    fn generate_tree(&mut self, depth: usize) -> Operation;
}

pub struct RandomArtGrammar {
    rng: StdRng,
}

impl RandomArtGrammar {
    pub fn new(seed: u64) -> Self {
        RandomArtGrammar {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    fn rand_leaf(&mut self) -> Operation {
        let choices = [
            Operation::VarX,
            Operation::VarY,
            Operation::VarT,
            Operation::Constant(self.rng.gen_range(-1.0..=1.0)),
            Operation::Circle(
                self.rng.gen_range(-1.0..=1.0),
                self.rng.gen_range(-1.0..=1.0),
            ),
        ];
        choices.choose(&mut self.rng).unwrap().clone()
    }

    fn rand_internal_op(&mut self, depth: usize) -> Operation {
        let choices = [
            |g: &mut Self, depth| {
                Operation::Sum(
                    g.generate_tree(depth - 1).into(),
                    g.generate_tree(depth - 1).into(),
                )
            },
            |g: &mut Self, depth| Operation::Sine(g.generate_tree(depth - 1).into()),
            |g: &mut Self, depth| {
                Operation::PerChannelMask(
                    g.generate_tree(depth - 1).into(),
                    g.generate_tree(depth - 1).into(),
                    g.generate_tree(depth - 1).into(),
                    g.rng.gen_range(-1.0..=1.0),
                )
            },
            |g: &mut Self, depth| {
                Operation::BinaryMask(
                    g.generate_tree(depth - 1).into(),
                    g.generate_tree(depth - 1).into(),
                    g.generate_tree(depth - 1).into(),
                    g.rng.gen_range(-1.0..=1.0),
                )
            },
            |g: &mut Self, depth| {
                Operation::SmoothMix(
                    g.generate_tree(depth - 1).into(),
                    g.generate_tree(depth - 1).into(),
                    g.generate_tree(depth - 1).into(),
                )
            },
            |g: &mut Self, depth| Operation::Well(g.generate_tree(depth - 1).into()),
            |g: &mut Self, depth| Operation::Tent(g.generate_tree(depth - 1).into()),
            |g: &mut Self, depth| {
                Operation::Product(
                    g.generate_tree(depth - 1).into(),
                    g.generate_tree(depth - 1).into(),
                )
            },
            |g: &mut Self, depth| Operation::Inverse(g.generate_tree(depth - 1).into()),
            |g: &mut Self, depth| {
                Operation::Mod(
                    g.generate_tree(depth - 1).into(),
                    g.generate_tree(depth - 1).into(),
                )
            },
        ];

        let op_fn = choices.choose(&mut self.rng).unwrap();
        op_fn(self, depth)
    }
}

impl ArtGrammar for RandomArtGrammar {
    fn generate_tree(&mut self, depth: usize) -> Operation {
        if depth == 0 {
            self.rand_leaf()
        } else {
            self.rand_internal_op(depth)
        }
    }
}

// Structure to represent a choice with its probability
struct WeightedChoice<T> {
    choice: T,
    weight: f64,
}

impl<T> WeightedChoice<T> {
    fn new(choice: T, weight: f64) -> Self {
        WeightedChoice { choice, weight }
    }
}

fn weighted_random_choice<'a, T>(rng: &mut StdRng, choices: &'a [WeightedChoice<T>]) -> &'a T {
    let total_weight: f64 = choices.iter().map(|c| c.weight).sum();
    let mut random_weight = rng.gen_range(0.0..total_weight);

    for choice in choices {
        if random_weight < choice.weight {
            return &choice.choice;
        }
        random_weight -= choice.weight;
    }

    &choices.last().unwrap().choice
}

// Source: https://users.ece.cmu.edu/~adrian/projects/validation/validation.pdf
pub struct PerrigSongGrammar {
    rng: StdRng,
}

impl PerrigSongGrammar {
    pub fn new(seed: u64) -> Self {
        PerrigSongGrammar {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    fn generate_a(&mut self) -> Operation {
        let choices = [
            WeightedChoice::new(Operation::Constant(self.rng.gen_range(-1.0..=1.0)), 1.0),
            WeightedChoice::new(Operation::VarX, 1.0),
            WeightedChoice::new(Operation::VarY, 1.0),
            WeightedChoice::new(Operation::VarT, 1.0),
        ];

        weighted_random_choice(&mut self.rng, &choices).clone()
    }

    fn generate_c(&mut self, depth: usize) -> Operation {
        if depth == 0 {
            return self.generate_a();
        }

        let choices = [
            WeightedChoice::new(self.generate_a(), 1.0),
            WeightedChoice::new(
                Operation::Sum(
                    self.generate_c(depth - 1).into(),
                    self.generate_c(depth - 1).into(),
                ),
                2.0,
            ),
            WeightedChoice::new(
                Operation::Product(
                    self.generate_c(depth - 1).into(),
                    self.generate_c(depth - 1).into(),
                ),
                2.0,
            ),
        ];

        weighted_random_choice(&mut self.rng, &choices).clone()
    }
}

impl ArtGrammar for PerrigSongGrammar {
    fn generate_tree(&mut self, depth: usize) -> Operation {
        Operation::RGB(
            self.generate_c(depth).into(),
            self.generate_c(depth).into(),
            self.generate_c(depth).into(),
        )
    }
}
