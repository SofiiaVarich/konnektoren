use super::TestResult;
use serde::{Deserialize, Serialize};

pub const LEADERBOARD_SIZE: usize = 50;
pub const LEADERBOARD_KEY: &str = "leaderboard";

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Leaderboard {
    tests: Vec<TestResult>,
}

impl Leaderboard {
    pub fn new() -> Self {
        Self { tests: Vec::new() }
    }

    pub fn add_test(&mut self, test: TestResult) {
        self.tests.push(test);
    }

    pub fn get_tests(&self) -> &Vec<TestResult> {
        &self.tests
    }
}
