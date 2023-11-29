use std::time::{Duration, Instant};

use crate::{bench_size::BENCH_COUNT, BenchmarkResult, Solution};

pub trait Problem {
    fn day(&self) -> u8;
    fn name(&self) -> &str;
    fn solve(&self) -> Solution;
    fn solve_part2(&self) -> Solution;

    fn bench(&self) -> BenchmarkResult {
        let results = [Duration::default(); BENCH_COUNT].map(|_| {
            let start = Instant::now();
            self.solve();
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
