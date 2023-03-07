use super::{Hello, HelloClient};
use soroban_sdk::{symbol, vec, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Hello);
    let client = HelloClient::new(&env, &contract_id);

    let got = client.hello(&symbol!("Soroban"));
    assert_eq!(
        got,
        vec![&env, symbol!("Hello"), symbol!("Soroban")],
    );
}
