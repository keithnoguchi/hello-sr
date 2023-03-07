//! A [hello world] contract on [Soroban] platform
//!
//! [hello world]: https://soroban.stellar.org/docs/getting-started/hello-world
//! [soroban]: https://soroban.stellar.org/docs

#![no_std]

use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec};

/// A hello world contract.
#[derive(Debug)]
pub struct Hello;

const COUNTER: Symbol = symbol!("COUNTER");

#[contractimpl]
impl Hello {
    /// Returns a vector with `Hello` appended to `to` argument.
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }

    /// Increments a counter by '1' and returns the result.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().get(&COUNTER).unwrap_or(Ok(0)).unwrap();
        count += 1;
        env.storage().set(&COUNTER, &count);
        count
    }
}

#[cfg(test)]
mod test;
