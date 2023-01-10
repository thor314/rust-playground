//! Goal: implement an 8-round ChaCha Rng, using
//! https://lib.rs/crates/rand_chacha,
//! https://cr.yp.to/chacha.html
//! as a reference.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::fmt;

use anyhow::Result;
use array64::Array64;
use chacha::ChaCha;
use rand::{CryptoRng, RngCore, SeedableRng};
use rand_core::block::{BlockRng, BlockRngCore};

mod array64;
#[cfg(test)] mod tests;
mod utils;

pub(crate) mod consts {

  // Ref:
  // https://github.com/rust-random/rand/blob/master/rand_chacha/src/guts.rs#L17
  // https://cr.yp.to/chacha/chacha-20080128.pdf
  /// Block Size
  pub(crate) const BLOCK: usize = 16;
  pub(crate) const BLOCK64: u64 = BLOCK as u64;
  /// Number of Buffer Blocks
  pub(crate) const LOG2_BUFBLOCKS: u64 = 2;
  pub(crate) const BUFBLOCKS: u64 = 1 << LOG2_BUFBLOCKS;
  /// Buffer Size
  pub(crate) const BUFSZ64: u64 = BLOCK64 * BUFBLOCKS;
  pub(crate) const BUFSZ: usize = BUFSZ64 as usize;
}

pub(crate) type KeyArray = [u8; 32];

mod chacha {
  use ppv_lite86::vec128_storage;

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
    pub fn new(key: &crate::KeyArray, nonce: &[u8]) -> Self {
      todo!();
    }

    // ref: https://github.com/rust-random/rand/blob/master/rand_chacha/src/guts.rs#L79
    pub fn refill4(&mut self, drounds: u32, out: &mut [u32; crate::consts::BUFSZ]) {
      todo!();
    }

    pub fn get_seed(&self) -> crate::KeyArray { todo!() }

    pub fn get_block_pos(&self) -> u64 { todo!() }
  }
}

/// ChaCha with 8 rounds
/// ref: https://github.com/rust-random/rand/blob/master/rand_chacha/src/guts.rs#L28
#[derive(Clone, PartialEq, Eq)]
struct ChaCha8Core {
  state: ChaCha,
}

impl fmt::Debug for ChaCha8Core {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "ChaCha8Core {{}}") }
}

// BlockRngCore implements the core functionality of BlockRng
impl BlockRngCore for ChaCha8Core {
  /// 2.1: https://cr.yp.to/chacha/chacha-20080128.pdf
  /// Salsa20 invertibly updates 4 32-bit state words
  type Item = u32;
  type Results = Array64<u32>;

  fn generate(&mut self, results: &mut Self::Results) { self.state.refill4(8, &mut results.0); }
}

// SeedableRng allows us to construct a ChaCha8Core from a seed
impl SeedableRng for ChaCha8Core {
  /// 256-bit seed
  type Seed = KeyArray;

  fn from_seed(seed: Self::Seed) -> Self { ChaCha8Core { state: ChaCha::new(&seed, &[0u8; 8]) } }
}

// Marker that ChaCha8 is cryptographically secure
impl CryptoRng for ChaCha8Core {}

#[derive(Clone, Debug)]
pub struct ChaCha8Rng {
  rng: BlockRng<ChaCha8Core>,
}

impl SeedableRng for ChaCha8Rng {
  type Seed = KeyArray;

  fn from_seed(seed: Self::Seed) -> Self {
    let core = ChaCha8Core::from_seed(seed);
    Self { rng: BlockRng::new(core) }
  }
}

// Core RNG functionality, boilerplate mostly
impl RngCore for ChaCha8Rng {
  fn next_u32(&mut self) -> u32 { self.rng.next_u32() }

  fn next_u64(&mut self) -> u64 { self.rng.next_u64() }

  fn fill_bytes(&mut self, dest: &mut [u8]) { self.rng.fill_bytes(dest) }

  fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
    self.rng.try_fill_bytes(dest)
  }
}

impl CryptoRng for ChaCha8Rng {}

impl From<ChaCha8Core> for ChaCha8Rng {
  fn from(core: ChaCha8Core) -> Self { ChaCha8Rng { rng: BlockRng::new(core) } }
}

const BLOCK_WORDS: u64 = 4;
impl ChaCha8Rng {
  /// Get the offset from the start of the stream in 32-bit words.
  ///
  /// Since the generated blocks are 16 bit words, and the counter is 16 bits, the offset is a 68
  /// bit number.
  pub fn get_word_pos(&self) -> u128 {
    let buf_start_block = {
      let buf_end_block = self.rng.core.state.get_block_pos();
      u64::wrapping_sub(buf_end_block, consts::BUFBLOCKS)
    };
    let (buf_offset_blocks, block_offset_words) = { 
      (blocks_part, words_part) };
    let pos_block = u64::wrapping_add(buf_start_block, buf_offset_blocks);
    let pos_block_words = u128::from(pos_block) * u128::from(BLOCK_WORDS);
    pos_block_words + u128::from(block_offset_words)
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

  pub fn get_seed(&self) -> [u8; 32] { self.rng.core.state.get_seed() }
}
