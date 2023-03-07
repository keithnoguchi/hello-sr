# hello-sr

[![CI](https://github.com/keithnoguchi/hello-sr/actions/workflows/ci.yml/badge.svg)](
https://github.com/keithnoguchi/hello-sr/actions)

[hello world]: https://soroban.stellar.org/docs/getting-started/hello-world
[soroban]: https://soroban.stellar.org/
[stellar]: https://stellar.org/

A [Hello World] contract with [Soroban] platform on [Stellar] network.

## Examples

```
#![no_std]

use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec};

pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }
}
```

## Unit test

You can run the following unit test with the standard `cargo test`:

```
use super::{Hello, HelloClient};
use soroban_sdk::{symbol, vec, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Hello);
    let client = HelloClient::new(&env, &contract_id);

    let got = client.hello(&symbol!("Soroban"));
    assert_eq!(got, vec![&env, symbol!("Hello"), symbol!("Soroban")],);
}
```

Happy Hacking!
