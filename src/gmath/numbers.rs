pub fn get_primes_number( limit: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    let mut counter = 3;
    primes.push(2); // 2 is the first prime number
    while counter <= limit {
        let mut is_prime = true;
        for val in &primes {
            if counter % val == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(counter);
        }
        counter += 2;
    }
    primes
}

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u32 + 1;
    let prime_list = get_primes_number(limit);
    if prime_list.contains(&n) {
        return true;
    }
    false
}

pub fn factorize(mut n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let primes = get_primes_number((n as f64).sqrt() as u32 + 1);
    for prime in primes {
        while n % prime == 0 {
            factors.push(prime);
            n /= prime;
        }
    }
    factors
}

pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub fn lcm(a: u32, b: u32) -> u32 {
    (a * b) / gcd(a, b)
}