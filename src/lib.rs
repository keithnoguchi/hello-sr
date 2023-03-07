//! A [hello world] contract on [Soroban] platform
//!
//! [hello world]: https://soroban.stellar.org/docs/getting-started/hello-world
//! [soroban]: https://soroban.stellar.org/docs

#![no_std]

use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec};

/// A hello world contract.
#[derive(Debug)]
pub struct Hello;

#[contractimpl]
impl Hello {
    /// Returns a vector with `Hello` appended to `to` argument.
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }
}

#[cfg(test)]
mod test;
