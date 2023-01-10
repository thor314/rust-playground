//! Goal: implement an 8-round ChaCha Rng, using
//! https://lib.rs/crates/rand_chacha,
//! https://cr.yp.to/chacha.html
//! as a reference.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::fmt;

use anyhow::Result;
use rand::{CryptoRng, RngCore, SeedableRng};
use rand_core::block::{BlockRng, BlockRngCore};

mod array64;
use array64::Array64;

#[cfg(test)] mod tests;
mod utils;

mod chacha {
  use ppv_lite86::vec128_storage;

  // Ref:
  // https://github.com/rust-random/rand/blob/master/rand_chacha/src/guts.rs#L17
  // https://cr.yp.to/chacha/chacha-20080128.pdf
  /// Block Size
  pub(crate) const BLOCK: usize = 16;
  pub(crate) const BLOCK64: u64 = BLOCK as u64;
  /// Number of Buffer Blocks
  const LOG2_BUFBLOCKS: u64 = 2;
  const BUFBLOCKS: u64 = 1 << LOG2_BUFBLOCKS;
  /// Buffer Size
  pub(crate) const BUFSZ64: u64 = BLOCK64 * BUFBLOCKS;
  pub(crate) const BUFSZ: usize = BUFSZ64 as usize;

  // Implementation of the crypto-simd API for x86
  // on SIMD instructions: https://cryptologie.net/article/405/simd-instructions-in-crypto/
  // https://lib.rs/crates/ppv-lite86
  #[derive(Clone, PartialEq, Eq)]
  pub struct ChaCha {
    pub(crate) b: vec128_storage,
    pub(crate) c: vec128_storage,
    pub(crate) d: vec128_storage,
  }
  impl ChaCha {
    // ref: https://github.com/rust-random/rand/blob/master/rand_chacha/src/guts.rs#L79
    pub fn refill4(&mut self, drounds: u32, out: &mut [u32; BUFSZ]) {
      todo!();
    }
  }
}

use chacha::ChaCha;
/// ChaCha with 8 rounds
/// ref: https://github.com/rust-random/rand/blob/master/rand_chacha/src/guts.rs#L28
#[derive(Clone, PartialEq, Eq)]
struct ChaCha8Core {
  state: ChaCha,
}

impl fmt::Debug for ChaCha8Core {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "ChaCha8Core {{}}") }
}

impl BlockRngCore for ChaCha8Core {
  /// 2.1: https://cr.yp.to/chacha/chacha-20080128.pdf
  /// Salsa20 invertibly updates 4 32-bit state words
  type Item = u32;
  type Results = Array64<u32>;

  fn generate(&mut self, results: &mut Self::Results) { self.state.refill4(8, &mut results.0); }
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
