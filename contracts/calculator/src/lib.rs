#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, vec, Env, Symbol, Vec};

const HISTORY: Symbol = symbol_short!("HISTORY");
const LAST_OP: Symbol = symbol_short!("LAST_OP");
const COUNTER: Symbol = symbol_short!("COUNTER");

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Operation {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub op: Op,
    pub id: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum Op {
    Sum,
    Sub,
    Mul,
    Div,
}

#[contract]
pub struct Calculator;

#[contractimpl]
impl Calculator {
    fn get_next_id(env: &Env) -> u32 {
        let mut id: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        id += 1;

        env.storage().instance().set(&COUNTER, &id);
        id
    }

    fn store_operation(env: &Env, operation: Operation) {
        let mut history: Vec<Operation> =
            env.storage().instance().get(&HISTORY).unwrap_or(vec![env]);

        history.push_front(operation);

        env.storage().instance().set(&HISTORY, &history);
        env.storage().instance().set(&LAST_OP, &operation);
        env.storage().instance().extend_ttl(100, 100);
    }

    pub fn sum(env: Env, x: u32, y: u32) -> u32 {
        let z = if x > u32::MAX - y { u32::MAX } else { x + y };
        let id = Calculator::get_next_id(&env);

        Calculator::store_operation(
            &env,
            Operation {
                x,
                y,
                z,
                op: Op::Sum,
                id,
            },
        );

        log!(&env, "sum: {}", z);
        z
    }

    pub fn sub(env: Env, x: u32, y: u32) -> u32 {
        let z = if x > y { x - y } else { y - x };
        let id = Calculator::get_next_id(&env);

        Calculator::store_operation(
            &env,
            Operation {
                x,
                y,
                z,
                op: Op::Sub,
                id,
            },
        );

        log!(&env, "sub: {}", z);
        z
    }

    pub fn mul(env: Env, x: u32, y: u32) -> u32 {
        let z = x * y;
        let id = Calculator::get_next_id(&env);

        Calculator::store_operation(
            &env,
            Operation {
                x,
                y,
                z,
                op: Op::Mul,
                id,
            },
        );

        log!(&env, "mul: {}", z);
        z
    }

    pub fn div(env: Env, x: u32, y: u32) -> u32 {
        let z = if y == 0 { u32::MAX } else { x / y };
        let id = Calculator::get_next_id(&env);

        Calculator::store_operation(
            &env,
            Operation {
                x,
                y,
                z,
                op: Op::Div,
                id,
            },
        );

        log!(&env, "div: {}", z);
        z
    }

    pub fn last_op(env: Env) -> Operation {
        let result: Operation = env
            .storage()
            .instance()
            .get(&LAST_OP)
            .expect("no last operation");

        log!(&env, "last_op: {}", result);
        result
    }

    pub fn get_op(env: Env, id: u32) -> Option<Operation> {
        let history: Vec<Operation> = env
            .storage()
            .instance()
            .get(&HISTORY)
            .unwrap_or(Vec::new(&env));

        let result = history.into_iter().find(|op| op.id == id);

        log!(&env, "get_op: {}", result);
        result
    }

    pub fn all_op(env: Env) -> Vec<Operation> {
        let result: Vec<Operation> = env
            .storage()
            .instance()
            .get(&HISTORY)
            .unwrap_or(Vec::new(&env));

        log!(&env, "all_op: {}", result);
        result
    }
}

mod test;
