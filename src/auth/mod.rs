// src/auth/mod.rs

pub mod hash;

pub use hash::{hash_password, verify_password};