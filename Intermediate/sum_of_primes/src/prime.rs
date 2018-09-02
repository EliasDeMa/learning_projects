pub fn sieve(upper_limit: &usize) -> Vec<u32> {
    let mut is_prime: Vec<bool> = vec![true; *upper_limit];
    let mut primes: Vec<u32> = vec![];

    is_prime[0] = false;
    is_prime[1] = false;
    let mut counter = 2;
    let mut multiples: usize;

    while counter * counter < *upper_limit {
        if is_prime[counter] {
            multiples = &counter * 2;
            while multiples < *upper_limit {
                is_prime[multiples] = false;
                multiples += counter;
            }
        }
        counter += 1;
    }

    for (i, _number) in is_prime.iter().enumerate() {
        if is_prime[i] {
            primes.push(i as u32);
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sieve_test() {
        let vector: Vec<u32> = vec![2,3];
        let comparison = sieve(&4);
        
        let vector2: Vec<u32> = vec![2,3,5,7,11,13];
        let comparison2 = sieve(&14);

        assert_eq!(vector, comparison);
        assert_eq!(vector2, comparison2);
    }
}
