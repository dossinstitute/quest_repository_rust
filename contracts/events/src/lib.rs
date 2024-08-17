#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Vec, Symbol, symbol_short, vec};

#[contract]
pub struct QEventsContract;

#[contractimpl]
impl QEventsContract {
    pub fn qevents(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to] 
    }   
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let env = Env::default();
        let contract_id = env.register_contract(None, QEventsContract);
        let client = QEventsContractClient::new(&env, &contract_id);

        let words = client.qevents(&symbol_short!("Dev"));
        assert_eq!(
            words,
            vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
        );
    }
}
