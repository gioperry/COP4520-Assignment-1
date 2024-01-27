
## To compile & run

```bash
cargo run
```

## How it works
- Eight prime number checking threads are spawned from the main thread.
- An atomic integer (`AtomicU64`) and a vector guarded by a Mutex (`Mutex<Vec<u64>>`) are shared between all prime number checking threads.
- The atomic integer determines the next number to be checked for primality and the vector of primes holds all the prime numbers to be found by the threads.
- Each thread requests a number by reading & iterating the atomic integer. It checks if the number is prime and if it is pushes the number onto the vector of primes.
- Each thread runs the above in a loop until all the numbers are checked. When a thread requests a number that exceeds $10^8$ it returns and terminates itself.
- The main thread waits for all 8 prime number checking threads to terminate and then calculates the results from the vector of primes.

## Correctness

```rust 
let maybe_prime = thread_local_counter.fetch_add(1, Ordering::Relaxed);
```

- When a thread requests a number from the shared atomic integer (named `thread_local_counter`) it iterates it by 1 as well. This is done in one atomic operation so no interleaving is possible and no two threads will ever request the same number. No numbers will be skipped as well.
- This is lock-free as no locks are used but it isn't wait-free.


```rust
if is_prime(maybe_prime) {
	let mut primes = thread_local_primes.lock().unwrap();
	primes.push(maybe_prime);
}
```

- Only one thread can read or write to the vector of primes at a time since it's guarded by a Mutex.
- The third line is a single line critical section since only one thread can execute it at a time. It's the only line where the thread has the lock guarding the vector of primes acquired.

## Possible improvements
- I'm thinking it may be possible to use a lock-free (or maybe even wait-free) array of some sort to store the primes rather than the `Mutex<Vec<u64>>`. This would allow the threads to spend more time checking if numbers are prime rather than waiting to acquire a lock.
- I believe a more efficient algorithm for the larger prime numbers could be used to speed up the runtime.
