//! A [hello world] contract on [Soroban] platform
//!
//! [hello world]: https://soroban.stellar.org/docs/getting-started/hello-world
//! [soroban]: https://soroban.stellar.org/

#![no_std]

use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec};

pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }
}
