use std::time::{Duration, Instant};

use crate::{bench_size::MAX_BENCH_COUNT, BenchmarkResult, Solution};

macro_rules! measure {
    ($self:ident, $fn:ident) => {{
        let start = Instant::now();
        std::hint::black_box($self.$fn());
        let end = Instant::now();
        end.duration_since(start)
    }};
}

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

    fn warmup_part1(&self) -> u128 {
        let start = Instant::now();
        for _ in 0..50 {
            std::hint::black_box(self.solve_part1());
        }
        let end = Instant::now();
        let duration = end.duration_since(start);

        duration.as_nanos() / 50
    }

    fn warmup_part2(&self) -> u128 {
        let start = Instant::now();
        for _ in 0..50 {
            std::hint::black_box(self.solve_part2());
        }
        let end = Instant::now();
        let duration = end.duration_since(start);

        duration.as_nanos() / 50
    }

    fn bench_part1(&self) -> BenchmarkResult {
        let mut results = [Duration::default(); MAX_BENCH_COUNT];
        let estimate_in_one_second: usize = (1_000_000_000 / self.warmup_part1())
            .try_into()
            .unwrap_or(MAX_BENCH_COUNT)
            .min(MAX_BENCH_COUNT);
        for result in results.iter_mut().take(estimate_in_one_second) {
            *result = measure!(self, solve_part1);
        }

        BenchmarkResult {
            name: self.name().to_owned(),
            part: 1,
            results,
            actual_size: estimate_in_one_second,
        }
    }

    fn bench_part2(&self) -> BenchmarkResult {
        let mut results = [Duration::default(); MAX_BENCH_COUNT];
        let estimate_in_one_second: usize = (1_000_000_000 / self.warmup_part2())
            .try_into()
            .unwrap_or(MAX_BENCH_COUNT)
            .min(MAX_BENCH_COUNT);
        for result in results.iter_mut().take(estimate_in_one_second) {
            *result = measure!(self, solve_part2);
        }

        BenchmarkResult {
            name: self.name().to_owned(),
            part: 1,
            results,
            actual_size: estimate_in_one_second,
        }
    }
}
