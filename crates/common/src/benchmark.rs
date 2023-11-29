use human_repr::HumanDuration;
use humansize::{format_size, BINARY};
use raw_cpuid::CpuId;
use std::time::Duration;
use sysinfo::{System, SystemExt};
use time::{format_description::well_known::Rfc2822, OffsetDateTime};

use crate::bench_size::BENCH_COUNT;

pub struct BenchmarkResult {
    pub name: String,
    pub part: usize,
    pub results: [Duration; BENCH_COUNT],
}

impl BenchmarkResult {
    pub fn average(&self) -> Duration {
        self.results.iter().sum::<Duration>().div_f64(BENCH_COUNT as f64)
    }

    pub fn plus_minus(&self) -> Duration {
        let average = self.average();
        let mut sum = 0.0;
        for result in &self.results {
            let diff = result.as_secs_f64() - average.as_secs_f64();
            sum += diff * diff;
        }
        let variance = sum / BENCH_COUNT as f64;
        let std_dev = variance.sqrt();
        let std_dev = std_dev.max(0.000_000_001);
        Duration::from_secs_f64(std_dev)
    }
}

pub struct BenchmarkCollection {
    pub name: String,
    pub benchmarks: Vec<BenchmarkResult>,
}

impl BenchmarkCollection {
    pub fn new(name: String) -> Self {
        Self { name, benchmarks: Vec::new() }
    }

    pub fn add(&mut self, benchmark: BenchmarkResult) {
        self.benchmarks.push(benchmark);
    }

    pub fn to_markdown(&self) -> String {
        let mut buffer = String::new();

        let cpu = CpuId::new();
        let cpu_name = cpu
            .get_processor_brand_string()
            .map(|s| s.as_str().to_string())
            .unwrap_or("Unknown".to_string());
        let os = os_info::get();
        let os_name = os.os_type().to_string();
        let os_version = os.version().to_string();
        let mut sys = System::new_all();
        sys.refresh_all();
        let mem_total = format_size(sys.total_memory(), BINARY);

        buffer.push_str(&format!("# {}\n\n", self.name));
        buffer.push_str(&format!(
            "Generated on {}\n\n",
            OffsetDateTime::now_utc().format(&Rfc2822).unwrap_or("Unknown".to_string())
        ));
        buffer.push_str("## Specifications\n\n");
        buffer.push_str(&format!("- CPU: {}\n", cpu_name));
        buffer.push_str(&format!("- OS: {} {}\n", os_name, os_version));
        buffer.push_str(&format!("- Memory: {}\n\n", mem_total));
        buffer.push_str("## Results\n\n");
        buffer.push_str("| Problem | Part 1 Time | Part 2 Time | Runs |\n");
        buffer.push_str("| ------- | ----------- | ----------- | ---- |\n");
        for bench_pair in self.benchmarks.chunks_exact(2) {
            let benchmark = &bench_pair[0];
            let benchmark2 = &bench_pair[1];
            buffer.push_str(&format!(
                "| {} | {} ± {} | {} ± {} | {} |\n",
                benchmark.name,
                &benchmark.average().human_duration(),
                &benchmark.plus_minus().human_duration(),
                &benchmark2.average().human_duration(),
                &benchmark2.plus_minus().human_duration(),
                benchmark.results.len(),
            ));
        }
        buffer
    }
}
