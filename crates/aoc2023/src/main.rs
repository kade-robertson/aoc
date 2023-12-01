use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
use common::{BenchmarkCollection, Problem};
mod day01;
use day01::*;
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let problems: Vec<Box<dyn Problem>> = vec![Box::new(Day01)];
    if args.contains(&"bench".to_string()) {
        for problem in problems {
            let bench = problem.bench_part1();
            println!(
                "{} - Part 1: {:?} ({} runs)",
                problem.name(),
                bench.average(),
                bench.results.len()
            );
            let bench = problem.bench_part2();
            println!(
                "{} - Part 2: {:?} ({} runs)",
                problem.name(),
                bench.average(),
                bench.results.len()
            );
        }
    } else if args.contains(&"bench-md".to_string()) {
        let mut collection = BenchmarkCollection::new(format!("Advent of Code {}", 2023u32));
        for problem in problems {
            let bench = problem.bench_part1();
            collection.add(bench);
            let bench = problem.bench_part2();
            collection.add(bench);
        }
        println!("{}", collection.to_markdown());
    } else {
        for problem in problems {
            println!("{} - Part 1: {}", problem.name(), problem.solve_part1());
            println!("{} - Part 2: {}", problem.name(), problem.solve_part2());
        }
    }
}
