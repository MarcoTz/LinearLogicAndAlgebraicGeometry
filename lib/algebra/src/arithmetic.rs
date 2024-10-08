pub fn gcd(n: i32, m: i32) -> u32 {
    let n_abs = n.wrapping_abs() as u32;
    let m_abs = m.wrapping_abs() as u32;
    let euclid = euclidean_algorithm(n_abs, m_abs, vec![n_abs, m_abs]);
    *euclid.get(euclid.len() - 2).unwrap()
}

pub fn gcd_all(ns: Vec<i32>) -> u32 {
    if ns.is_empty() {
        return 0;
    }
    let mut current_gcd = *ns.first().unwrap();
    for next_n in ns.iter() {
        current_gcd = gcd(*next_n, current_gcd) as i32;
    }
    current_gcd as u32
}

fn euclidean_algorithm(n: u32, m: u32, previous_remainders: Vec<u32>) -> Vec<u32> {
    if n < m {
        let mut new_remainders = previous_remainders;
        new_remainders.reverse();
        return euclidean_algorithm(m, n, new_remainders);
    }

    if m == 0 {
        return previous_remainders;
    }

    let quot = n / m;
    let rem = n % m;
    let new_n = n - quot * m;
    let mut new_remainders = previous_remainders;
    new_remainders.push(rem);
    if rem == 0 {
        return new_remainders;
    }
    euclidean_algorithm(m, new_n, new_remainders)
}

#[cfg(test)]
mod arithmetic_tests {
    use super::{euclidean_algorithm, gcd, gcd_all};

    #[test]
    fn euclid_1() {
        let result = euclidean_algorithm(1, 2, vec![1, 2]);
        let expected = vec![2, 1, 0];
        assert_eq!(result, expected)
    }

    #[test]
    fn euclid_2() {
        let result = euclidean_algorithm(10, 18, vec![10, 18]);
        let expected = vec![18, 10, 8, 2, 0];
        assert_eq!(result, expected)
    }

    #[test]
    fn gcd_coprime() {
        let result = gcd(13, 17);
        let expected = 1;
        assert_eq!(result, expected)
    }

    #[test]
    fn gcd_10() {
        let result = gcd(10, 20);
        let expected = 10;
        assert_eq!(result, expected)
    }

    #[test]
    fn gcd_neg() {
        let result = gcd(-5, 5);
        let expected = 5;
        assert_eq!(result, expected)
    }

    #[test]
    fn gcd_2() {
        let result = gcd(26, 10);
        let expected = 2;
        assert_eq!(result, expected)
    }

    #[test]
    fn gcd_all_1() {
        let result = gcd_all(vec![5, 6, 7]);
        let expected = 1;
        assert_eq!(result, expected)
    }

    #[test]
    fn gcd_all_2() {
        let result = gcd_all(vec![2, 4, 6]);
        let expected = 2;
        assert_eq!(result, expected)
    }
}
