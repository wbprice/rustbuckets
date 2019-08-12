use rand::{
    distributions::{Distribution, Standard}, Rng
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Heading {
    East,
    South
}

impl Heading {
    pub fn flip(&self) -> Heading {
        match self {
            Heading::East => Heading::South,
            Heading::South => Heading::East
        }
    }
}

impl Default for Heading {
    fn default() -> Self {
        Heading::East
    }
}

impl Distribution<Heading> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Heading {
        match rng.gen_range(0, 1) {
            0 => Heading::East,
            1 => Heading::South,
            _ => Heading::South
        }
    }
}