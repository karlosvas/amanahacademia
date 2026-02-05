pub mod controllers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

// Test helpers (fixtures compartidos)
#[cfg(test)]
#[path = "test/helpers/fixtures.rs"]
pub mod test_fixtures;
