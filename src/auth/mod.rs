// src/auth/mod.rs

pub mod hash;
pub mod jwt;

pub use hash::{hash_password, verify_password};
pub use jwt::{generate_token, verify_token, Claims};