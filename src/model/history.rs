use super::TestResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct History {
    test_results: Vec<TestResult>,
}

impl History {
    pub fn new() -> Self {
        Self {
            test_results: Vec::new(),
        }
    }

    pub fn add_test_result(&mut self, test_result: TestResult) {
        self.test_results.push(test_result);
    }

    pub fn get_test_results(&self) -> &Vec<TestResult> {
        &self.test_results
    }

    pub fn longest_streak(&self) -> i64 {
        if self.test_results.is_empty() {
            return 0;
        }

        let mut dates: Vec<DateTime<Utc>> =
            self.test_results.iter().map(|result| result.date).collect();
        dates.sort_unstable();
        dates.dedup();

        let mut longest_streak = 1;
        let mut current_streak = 1;

        for i in 1..dates.len() {
            if dates[i].signed_duration_since(dates[i - 1]).num_days() == 1 {
                current_streak += 1;
                longest_streak = longest_streak.max(current_streak);
            } else if dates[i].signed_duration_since(dates[i - 1]).num_days() > 1 {
                current_streak = 1;
            }
        }

        longest_streak
    }
}

#[cfg(test)]
mod tests {
    use super::super::TestType;
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn history_new() {
        let history = History::new();
        assert_eq!(history.test_results.len(), 0);
    }

    #[test]
    fn history_add_test_result() {
        let mut history = History::new();
        let test_result = TestResult::new(
            TestType::Adjectives,
            10,
            8,
            2,
            "Player".to_string(),
            Utc::now(),
        );
        history.add_test_result(test_result.clone());
        assert_eq!(history.test_results.len(), 1);
        assert_eq!(history.test_results[0], test_result);
    }

    #[test]
    fn history_get_test_results() {
        let mut history = History::new();

        let test_result = TestResult::new(
            TestType::Adjectives,
            10,
            8,
            2,
            "Player".to_string(),
            Utc::now(),
        );
        history.add_test_result(test_result.clone());
        assert_eq!(history.get_test_results().len(), 1);
        assert_eq!(history.get_test_results()[0], test_result);
    }

    #[test]
    fn history_longest_streak() {
        let mut history = History::new();
        let test_result1 = TestResult::new(
            TestType::Adjectives,
            10,
            8,
            2,
            "Player".to_string(),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 1, 1).and_hms(0, 0, 0), Utc),
        );
        let test_result2 = TestResult::new(
            TestType::Adjectives,
            10,
            8,
            2,
            "Player".to_string(),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 1, 2).and_hms(0, 0, 0), Utc),
        );
        let test_result3 = TestResult::new(
            TestType::Adjectives,
            10,
            8,
            2,
            "Player".to_string(),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 1, 3).and_hms(0, 0, 0), Utc),
        );
        let test_result4 = TestResult::new(
            TestType::Adjectives,
            10,
            8,
            2,
            "Player".to_string(),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 1, 4).and_hms(0, 0, 0), Utc),
        );
        let test_result5 = TestResult::new(
            TestType::Adjectives,
            10,
            8,
            2,
            "Player".to_string(),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 1, 5).and_hms(0, 0, 0), Utc),
        );
        let test_result6 = TestResult::new(
            TestType::Adjectives,
            10,
            8,
            2,
            "Player".to_string(),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 1, 6).and_hms(0, 0, 0), Utc),
        );
        let test_result7 = TestResult::new(
            TestType::Adjectives,
            10,
            8,
            2,
            "Player".to_string(),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 1, 8).and_hms(0, 0, 0), Utc),
        );
        let test_result8 = TestResult::new(
            TestType::Adjectives,
            10,
            8,
            2,
            "Player".to_string(),
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 1, 9).and_hms(0, 0, 0), Utc),
        );
        history.add_test_result(test_result1);
        history.add_test_result(test_result2);
        history.add_test_result(test_result3);
        history.add_test_result(test_result4);
        history.add_test_result(test_result5);
        history.add_test_result(test_result6);
        history.add_test_result(test_result7);
        history.add_test_result(test_result8);

        assert_eq!(history.longest_streak(), 6);
    }
}
