//! A way to convert an input into a series of choices in a unpredictable,
//! determinstic way.

use num::{BigUint, ToPrimitive};
use sha2::{Digest, Sha512};
use std::ops::{Div, Rem};

lazy_static! {
    static ref MAX_512_BIT: BigUint = {
        use num::{bigint::ToBigUint, pow::pow};
        pow(2u32.to_biguint().unwrap(), 512)
    };
}

pub struct Genome {
    /// Used for making choices
    remaining: BigUint,

    /// Biggest that `remaining` could be, given past choices.
    ///
    /// When this reaches 0, the Genotype is exhausted.
    current_max: BigUint,
}

impl Genome {
    pub fn via_sha512<T>(input: T) -> Self
    where
        T: Into<String>,
    {
        let input = input.into();
        let hash = Sha512::digest(input.as_bytes());
        Self {
            remaining: BigUint::from_bytes_be(&hash),
            current_max: MAX_512_BIT.clone(),
        }
    }

    /// Generate a number from 0 to `size`, by taking entropy from the genome,
    /// reducing `self.remaining` and `self.current_max` appropriately.
    ///
    /// Returns an error if there isn't enough entropy remaining to fulfill the
    /// request.
    fn take<T>(&mut self, size: T) -> Result<BigUint, ()>
    where
        T: Into<BigUint>,
    {
        let size = size.into();
        if size > self.current_max {
            return Err(());
        }
        let res = (&self.remaining).rem(&size);
        self.remaining = (&self.remaining).div(&size);
        self.current_max /= &size;
        Ok(res)
    }

    pub fn gen<T: GenomeGen>(&mut self) -> GenomeResult<T> {
        T::gen(self)
    }

    pub fn gen_range<T: GenomeGenRange>(&mut self, low: T, high: T) -> GenomeResult<T> {
        T::gen_range(self, low, high)
    }

    pub fn choose<T: Clone>(&mut self, choices: &Vec<T>) -> GenomeResult<T> {
        Ok(choices[self.gen_range(0, choices.len())?].clone())
    }

    pub fn choose_weighted<T: Clone>(&mut self, choices: &Vec<(T, usize)>) -> GenomeResult<T> {
        let sum_weights = choices.iter().map(|c| c.1).sum();
        let mut choice = self.gen_range(0, sum_weights)?;
        for (item, weight) in choices.iter() {
            if choice < *weight {
                return Ok(item.clone());
            }
            choice -= *weight;
        }
        unreachable!("No items chosen");
    }
}

pub type GenomeResult<T> = Result<T, ()>;

/// Implement this trait to generate a value of type from a Genome
pub trait GenomeGen: Sized {
    fn gen(genome: &mut Genome) -> GenomeResult<Self>;
}

/// Implement this trait to generate a value of type in a range from a Genome
pub trait GenomeGenRange: Sized {
    fn gen_range(genome: &mut Genome, high: Self, low: Self) -> GenomeResult<Self>;
}

macro_rules! genome_gens_int {
    ($t:ty, $to_prim:ident) => {
        impl GenomeGen for $t {
            fn gen(genome: &mut Genome) -> GenomeResult<$t> {
                genome.take(Self::max_value())?.$to_prim().ok_or(())
            }
        }

        impl GenomeGenRange for $t {
            fn gen_range(genome: &mut Genome, low: $t, high: $t) -> GenomeResult<$t> {
                Ok(genome.take(high - low)?.$to_prim().ok_or(())? + low)
            }
        }
    };
}

genome_gens_int!(usize, to_usize);
genome_gens_int!(u8, to_u8);
genome_gens_int!(u16, to_u16);
genome_gens_int!(u64, to_u64);
