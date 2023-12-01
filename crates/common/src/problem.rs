use std::time::{Duration, Instant};

use crate::{bench_size::BENCH_COUNT, BenchmarkResult, Solution};

pub trait Problem {
    fn problem_input(&self) -> &'static str;
    fn day(&self) -> u8;
    fn name(&self) -> &str;
    fn solve_part1_with(&self, input: &str) -> Solution;
    fn solve_part2_with(&self, input: &str) -> Solution;

    fn solve_part1(&self) -> Solution {
        self.solve_part1_with(self.problem_input())
    }

    fn solve_part2(&self) -> Solution {
        self.solve_part2_with(self.problem_input())
    }

    fn bench_part1(&self) -> BenchmarkResult {
        let results = [Duration::default(); BENCH_COUNT].map(|_| {
            let start = Instant::now();
            self.solve_part1();
            let end = Instant::now();
            end.duration_since(start)
        });
        BenchmarkResult { name: self.name().to_owned(), part: 1, results }
    }

    fn bench_part2(&self) -> BenchmarkResult {
        let results = [Duration::default(); BENCH_COUNT].map(|_| {
            let start = Instant::now();
            self.solve_part2();
            let end = Instant::now();
            end.duration_since(start)
        });
        BenchmarkResult { name: self.name().to_owned(), part: 2, results }
    }
}
