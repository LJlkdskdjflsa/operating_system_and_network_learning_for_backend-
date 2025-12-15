//! Lab 3 Reference Answer

use rand::seq::SliceRandom;
use std::time::Instant;

/// Sum array elements sequentially (good locality)
fn sum_sequential(arr: &[i64]) -> i64 {
    arr.iter().sum()
}

/// Sum array elements in random order (poor locality)
fn sum_random(arr: &[i64], indices: &[usize]) -> i64 {
    indices.iter().map(|&i| arr[i]).sum()
}

/// Sum 2D array in row-major order (good locality for Rust arrays)
fn sum_row_major(arr: &[[i64; 1000]; 1000]) -> i64 {
    let mut sum = 0i64;
    for i in 0..1000 {
        for j in 0..1000 {
            sum += arr[i][j];
        }
    }
    sum
}

/// Sum 2D array in column-major order (poor locality for Rust arrays)
fn sum_column_major(arr: &[[i64; 1000]; 1000]) -> i64 {
    let mut sum = 0i64;
    for j in 0..1000 {
        for i in 0..1000 {
            sum += arr[i][j];
        }
    }
    sum
}

fn benchmark<F, T>(name: &str, mut f: F) -> (T, std::time::Duration)
where
    F: FnMut() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("{:25} {:?}", name, duration);
    (result, duration)
}

fn main() {
    #[cfg(debug_assertions)]
    {
        eprintln!("WARNING: Running in debug mode!");
        eprintln!("Results will be much slower and less meaningful.");
        eprintln!("Please run with: cargo run --release\n");
    }

    const SIZE: usize = 10_000_000;

    println!("=== Cache Locality Experiment ===\n");
    println!(
        "Array size: {} elements ({} MB)\n",
        SIZE,
        SIZE * 8 / 1_000_000
    );

    // Create and initialize array
    println!("Creating array...");
    let arr: Vec<i64> = (0..SIZE as i64).collect();

    // Create shuffled indices for random access
    println!("Shuffling indices...");
    let mut indices: Vec<usize> = (0..SIZE).collect();
    let mut rng = rand::thread_rng();
    indices.shuffle(&mut rng);

    println!("Running benchmarks...\n");

    // Part 1: Sequential vs Random
    println!("--- 1D Array Access ---");
    let (seq_result, seq_time) = benchmark("Sequential sum:", || sum_sequential(&arr));
    let (rand_result, rand_time) = benchmark("Random sum:", || sum_random(&arr, &indices));

    assert_eq!(seq_result, rand_result, "Results should match!");
    println!("Results match: {}", seq_result);

    let speedup_1d = rand_time.as_nanos() as f64 / seq_time.as_nanos() as f64;
    println!("Speedup: {:.1}x faster\n", speedup_1d);

    // Part 2: Row-major vs Column-major
    println!("--- 2D Array Access (1000x1000) ---");
    println!("Creating 2D array...");

    let arr_2d: Box<[[i64; 1000]; 1000]> = {
        let mut arr = Box::new([[0i64; 1000]; 1000]);
        for i in 0..1000 {
            for j in 0..1000 {
                arr[i][j] = (i * 1000 + j) as i64;
            }
        }
        arr
    };

    let (row_result, row_time) = benchmark("Row-major sum:", || sum_row_major(&arr_2d));
    let (col_result, col_time) = benchmark("Column-major sum:", || sum_column_major(&arr_2d));

    assert_eq!(row_result, col_result, "Results should match!");
    println!("Results match: {}", row_result);

    let speedup_2d = col_time.as_nanos() as f64 / row_time.as_nanos() as f64;
    println!("Speedup: {:.1}x faster\n", speedup_2d);

    println!("=== Analysis ===");
    println!("1D Array:");
    println!("  Sequential was {:.1}x faster than random", speedup_1d);
    println!("  - Sequential access utilizes cache lines efficiently");
    println!("  - Random access causes cache misses on almost every access\n");

    println!("2D Array:");
    println!("  Row-major was {:.1}x faster than column-major", speedup_2d);
    println!("  - Rust stores arrays in row-major order");
    println!("  - Row-major: elements are contiguous in memory");
    println!("  - Column-major: jumps 8000 bytes between accesses\n");

    println!("Try running with perf to see cache statistics:");
    println!("  perf stat -e cache-misses,cache-references ./target/release/locality_demo");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_sum() {
        let arr: Vec<i64> = (1..=100).collect();
        assert_eq!(sum_sequential(&arr), 5050);
    }

    #[test]
    fn test_random_sum() {
        let arr: Vec<i64> = (1..=100).collect();
        let indices: Vec<usize> = (0..100).collect();
        assert_eq!(sum_random(&arr, &indices), 5050);
    }

    #[test]
    fn test_sequential_equals_random() {
        let arr: Vec<i64> = (0..1000).collect();
        let mut indices: Vec<usize> = (0..1000).collect();
        let mut rng = rand::thread_rng();
        indices.shuffle(&mut rng);

        assert_eq!(sum_sequential(&arr), sum_random(&arr, &indices));
    }

    #[test]
    fn test_row_major_equals_column_major() {
        let arr: [[i64; 10]; 10] = {
            let mut arr = [[0i64; 10]; 10];
            for i in 0..10 {
                for j in 0..10 {
                    arr[i][j] = (i * 10 + j) as i64;
                }
            }
            arr
        };

        // Small array for testing
        let row_sum: i64 = arr.iter().flatten().sum();
        let col_sum: i64 = {
            let mut sum = 0;
            for j in 0..10 {
                for i in 0..10 {
                    sum += arr[i][j];
                }
            }
            sum
        };

        assert_eq!(row_sum, col_sum);
    }
}
