//! Goal: implement an 8-round ChaCha Rng, using
//! https://lib.rs/crates/rand_chacha
//! as a reference.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::fmt;

use anyhow::Result;
use rand::{CryptoRng, RngCore, SeedableRng};
use rand_core::block::{BlockRng, BlockRngCore};

mod error;
#[cfg(test)] mod tests;
mod utils;

mod chacha {
  use ppv_lite86::vec128_storage;

  // Implementation of the crypto-simd API for x86
  #[derive(Clone, PartialEq, Eq)]
  pub struct ChaCha {
    pub(crate) b: vec128_storage,
    pub(crate) c: vec128_storage,
    pub(crate) d: vec128_storage,
  }
}

use chacha::ChaCha;

/// ChaCha with 8 rounds
#[derive(Clone, PartialEq, Eq)]
struct ChaCha8Core {
  state: ChaCha,
}

impl fmt::Debug for ChaCha8Core {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}

impl BlockRngCore for ChaCha8Core {
  type Item;
  type Results;

  fn generate(&mut self, results: &mut Self::Results) { todo!() }
}
impl SeedableRng for ChaCha8Core {
  type Seed;

  fn from_seed(seed: Self::Seed) -> Self { todo!() }
}

impl CryptoRng for ChaCha8Core {}

#[derive(Clone, Debug)]
pub struct ChaCha8Rng {
  rng: BlockRng<ChaCha8Core>,
}

impl SeedableRng for ChaCha8Rng {
  type Seed;

  fn from_seed(seed: Self::Seed) -> Self { todo!() }
}

impl RngCore for ChaCha8Rng {
  fn next_u32(&mut self) -> u32 { todo!() }

  fn next_u64(&mut self) -> u64 { todo!() }

  fn fill_bytes(&mut self, dest: &mut [u8]) { todo!() }

  fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> { todo!() }
}
impl CryptoRng for ChaCha8Rng {}

impl From<ChaCha8Core> for ChaCha8Rng {
  fn from(value: ChaCha8Core) -> Self {
    todo!();
  }
}

impl ChaCha8Rng {
  pub fn get_word_pos(&self) -> u128 {
    todo!();
  }

  pub fn set_word_pos(&mut self, word_offset: u128) {
    todo!();
  }

  pub fn set_stream(&mut self, stream: u64) {
    todo!();
  }

  pub fn get_stream(&self) -> u64 {
    todo!();
  }

  pub fn get_seed(&self) -> [u8; 32] {
    todo!();
  }
}
