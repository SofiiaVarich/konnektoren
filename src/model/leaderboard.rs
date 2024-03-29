use super::TestResult;
use serde::{Deserialize, Serialize};

pub const LEADERBOARD_SIZE: usize = 50;
pub const LEADERBOARD_KEY: &str = "leaderboard";

#[derive(Default, Serialize, Deserialize, Clone, PartialEq)]
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

    pub fn get_ranked(&self) -> Vec<TestResult> {
        let distinct = self.tests.iter().collect::<std::collections::HashSet<_>>();
        let mut tests: Vec<TestResult> = distinct.into_iter().cloned().collect();

        tests.sort_by(|a, b| {
            b.performance_percentage
                .partial_cmp(&a.performance_percentage)
                .unwrap()
        });
        tests.truncate(LEADERBOARD_SIZE);
        tests
    }
}
