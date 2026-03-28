#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, symbol_short};

// Storage key for total users
const USER_COUNT: Symbol = symbol_short!("U_COUNT");

// Mapping user_id -> reputation
#[contracttype]
pub enum UserBook {
    User(u64)
}

// Structure for storing reputation
#[contracttype]
#[derive(Clone)]
pub struct Reputation {
    pub user_id: u64,
    pub score: u64,
}

#[contract]
pub struct ReputationEngine;

#[contractimpl]
impl ReputationEngine {

    // 1. Register a new user
    pub fn register_user(env: Env) -> u64 {
        let mut count: u64 = env.storage().instance().get(&USER_COUNT).unwrap_or(0);
        count += 1;

        let user = Reputation {
            user_id: count,
            score: 0,
        };

        env.storage().instance().set(&UserBook::User(count), &user);
        env.storage().instance().set(&USER_COUNT, &count);

        count
    }

    // 2. Add reputation score
    pub fn add_score(env: Env, user_id: u64, points: u64) {
        let mut user = Self::view_score(env.clone(), user_id);

        if user.user_id == 0 {
            panic!("User not found");
        }

        user.score += points;

        env.storage().instance().set(&UserBook::User(user_id), &user);
    }

    // 3. Deduct reputation score
    pub fn deduct_score(env: Env, user_id: u64, points: u64) {
        let mut user = Self::view_score(env.clone(), user_id);

        if user.user_id == 0 {
            panic!("User not found");
        }

        if user.score < points {
            panic!("Insufficient score");
        }

        user.score -= points;

        env.storage().instance().set(&UserBook::User(user_id), &user);
    }

    // 4. View reputation score
    pub fn view_score(env: Env, user_id: u64) -> Reputation {
        env.storage().instance().get(&UserBook::User(user_id)).unwrap_or(
            Reputation {
                user_id: 0,
                score: 0,
            }
        )
    }
}