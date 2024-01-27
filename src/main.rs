use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::time::Instant;

const LIMIT: u64 = 100000000; // 10^8
const THREAD_COUNT: u64 = 8;
const OUTPUT_FILE: &str = "primes.txt";

fn main() {
    // 0 & 1 are not prime so we start at 2
    let counter = Arc::new(AtomicU64::new(2));
    let primes = Arc::new(Mutex::new(vec![]));

    let start_time = Instant::now();

    let mut join_handles = Vec::with_capacity(THREAD_COUNT as usize);

    // Spawn THREAD_COUNT prime number checking threads
    for _ in 0..THREAD_COUNT {
        let thread_local_counter = counter.clone();
        let thread_local_primes = primes.clone();

        let handle = spawn(move || loop {
            let maybe_prime = thread_local_counter.fetch_add(1, Ordering::Relaxed);

            if maybe_prime > LIMIT {
                return;
            }

            if is_prime(maybe_prime) {
                let mut primes = thread_local_primes.lock().unwrap();
                primes.push(maybe_prime);
            }
        });

        join_handles.push(handle);
    }

    // Wait for all the prime number checking threads to return
    for handle in join_handles {
        handle.join().unwrap();
    }

    let execution_time = start_time.elapsed().as_millis();

    let mut final_primes = primes.lock().unwrap();
    final_primes.sort();

    let count = final_primes.len();
    let sum: u64 = final_primes.iter().sum();

    let largest_ten: Vec<String> = final_primes
        .iter()
        .rev()
        .take(10)
        .rev()
        .map(u64::to_string)
        .collect();

    let results = format!(
        "{} {} {}\n{}",
        execution_time,
        count,
        sum,
        largest_ten.join(", ")
    );

    let mut output_file = File::create(OUTPUT_FILE)
        .expect(&format!("Failed to create output file at {}", OUTPUT_FILE));

    output_file.write_all(results.as_bytes()).unwrap();
}

fn is_prime(number: u64) -> bool {
    if number <= 1 {
        return false;
    }

    if number == 2 || number == 3 {
        return true;
    }

    if number % 2 == 0 || number % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= number {
        if number % i == 0 || number % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_detect_prime() {
        let prime = 83;
        assert_eq!(is_prime(prime), true);
    }

    #[test]
    fn can_detect_non_prime() {
        let non_prime = 80;
        assert_eq!(is_prime(non_prime), false);
    }
}
