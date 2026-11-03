use std::{fs, path::PathBuf};

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;
use tempfile::TempDir;

use tali::scanner::Scanner;

/// Create a dataset with given file count and size
fn create_dataset(file_count: usize, file_size: usize) -> (TempDir, PathBuf, u64) {
    let dir = tempfile::tempdir().unwrap();

    let line = "one two three four five six seven eight\n";
    let contents = line.repeat((file_size / line.len()).max(1));

    let bytes_per_file = contents.len() as u64;

    for i in 0..file_count {
        let path = dir.path().join(format!("file-{i:05}.txt"));
        fs::write(path, &contents).unwrap();
    }

    let path = dir.path().to_path_buf();
    let total_bytes = bytes_per_file * file_count as u64;

    (dir, path, total_bytes)
}

// ---------
// BENCHMARK
// ---------

/// Benchmark the tali [scanner][Scanner]
fn benchmark_scanner(c: &mut Criterion) {
    // Define workloads to test
    let workloads = [
        ("many-small", 500, 1024),
        ("medium", 100, 64 * 1024),
        ("few-large", 10, 4 * 1024 * 1024),
    ];

    for (name, file_count, file_size) in workloads {
        // Create the dataset outside the timed benchmark
        let (_dir, path, total_bytes) = create_dataset(file_count, file_size);

        // Prepare paths for the scanner
        let paths = [path];

        // Create benchmark group
        let mut group = c.benchmark_group(format!("scanner/{name}"));
        group.throughput(Throughput::Bytes(total_bytes));

        // Set sample size and time
        group.sample_size(30);
        group.measurement_time(std::time::Duration::from_secs(10));

        // Run serial and parallel modes
        for (mode, parallel) in [("serial", false), ("parallel", true)] {
            let mut scanner = Scanner::new();
            if parallel {
                scanner.run_parallel(parallel);
            }
            group.bench_function(mode, |b| {
                b.iter(|| black_box(scanner.scan(black_box(&paths)).unwrap()))
            });
        }

        // Finish this group
        group.finish();
    }
}

criterion_group!(benches, benchmark_scanner);
criterion_main!(benches);
