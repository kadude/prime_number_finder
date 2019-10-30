use std::env;

// the Sieve of Eratosthenes
fn sieve_of_eratosthenes(value: usize) {
    let mut primes = vec![true; value + 1];
    primes[0] = false;
    primes[1] = false;
    for i in 2..=f64::sqrt(value as f64).floor() as usize {
        if primes[i] {
            let mut teller = 0;

            loop {
                let j = i.pow(2) + (i * teller);
                if j > value {
                    break;
                }
                primes[j] = false;
                teller += 1;
            }
        }
    }
    // #[allow(clippy::needless_range_loop)]
    // for i in 2..=value {
    //     if primes[i] {
    //         println!("Fant primtall {}", i);
    //     }
    // }

    println!("Antall primtall {}", primes.iter().filter(|x| **x).count());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().count() < 2 {
        eprintln!("Error: Expecting one argument");
        return;
    }
    sieve_of_eratosthenes(args[1].parse::<usize>().unwrap());
}
