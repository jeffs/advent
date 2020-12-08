use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead as _, BufReader};
use std::mem;
use std::ops::RangeInclusive;
use std::path::Path;

type Passport = HashMap<String, String>;

const KEYS: [&str; 7] = [
    "byr", // Birth Year
    "iyr", // Issue Year
    "eyr", // Expiration Year
    "hgt", // Height
    "hcl", // Hair Color
    "ecl", // Eye Color
    "pid", // Passport ID
];

fn load_passports<P: AsRef<Path>>(input: P) -> io::Result<Vec<Passport>> {
    let mut passports = Vec::new();
    let mut passport = Passport::new();
    for line in BufReader::new(File::open(input)?).lines() {
        let line = line?;
        if line.is_empty() {
            passports.push(mem::take(&mut passport));
        } else {
            for pair in line.split_whitespace() {
                let mut parts = pair.splitn(2, ':');
                let key = parts.next().unwrap().to_owned();
                let value = parts.next().unwrap().to_owned();
                passport.insert(key, value);
            }
        }
    }
    if !passport.is_empty() {
        passports.push(passport);
    }
    Ok(passports)
}

fn is_valid1(passport: &Passport) -> bool {
    KEYS.iter().all(|&key| passport.contains_key(key))
}

fn solve_part1(passports: &[Passport]) -> usize {
    passports.iter().cloned().filter(is_valid1).count()
}

fn is_four_digits_in_range(s: &str, r: RangeInclusive<u32>) -> bool {
    s.len() == 4 && s.parse().ok().filter(|n| r.contains(n)).is_some()
}

fn byr_is_valid(s: &str) -> bool {
    is_four_digits_in_range(s, 1920..=2002)
}

fn iyr_is_valid(s: &str) -> bool {
    is_four_digits_in_range(s, 2010..=2020)
}

fn eyr_is_valid(s: &str) -> bool {
    is_four_digits_in_range(s, 2020..=2030)
}

fn hgt_is_valid(s: &str) -> bool {
    s.len() > 2 && {
        match s.split_at(s.len() - 2) {
            (num, "cm") => num
                .parse::<u32>()
                .ok()
                .filter(|n| (150..=193).contains(n))
                .is_some(),
            (num, "in") => num
                .parse::<u32>()
                .ok()
                .filter(|n| (59..=76).contains(n))
                .is_some(),
            _ => false,
        }
    }
}

fn hcl_is_valid(s: &str) -> bool {
    s.len() == 7 && s.starts_with('#') && s.chars().skip(1).all(|c| "0123456789abcdef".contains(c))
}

fn ecl_is_valid(s: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"][..].contains(&s)
}

fn pid_is_valid(s: &str) -> bool {
    s.len() == 9 && s.chars().all(|c| "0123456789".contains(c))
}

fn has_valid<F>(passport: &Passport, key: &str, criterion: F) -> bool
where
    F: FnOnce(&str) -> bool,
{
    passport.get(key).filter(|value| criterion(value)).is_some()
}

fn is_valid2(passport: &Passport) -> bool {
    has_valid(passport, "byr", byr_is_valid)
        && has_valid(passport, "iyr", iyr_is_valid)
        && has_valid(passport, "eyr", eyr_is_valid)
        && has_valid(passport, "hgt", hgt_is_valid)
        && has_valid(passport, "hcl", hcl_is_valid)
        && has_valid(passport, "ecl", ecl_is_valid)
        && has_valid(passport, "pid", pid_is_valid)
}

fn solve_part2(passports: &[Passport]) -> usize {
    passports.iter().cloned().filter(is_valid2).count()
}

fn main() {
    let input = "tests/day4/input";
    let passports = load_passports(input).expect("can't read input");
    println!("{}", solve_part1(&passports));
    println!("{}", solve_part2(&passports));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn criteria() {
        assert!(byr_is_valid("2002"));
        assert!(!byr_is_valid("2003"));

        assert!(hgt_is_valid("60in"));
        assert!(hgt_is_valid("190cm"));
        assert!(!hgt_is_valid("190in"));
        assert!(!hgt_is_valid("190"));

        assert!(hcl_is_valid("#123abc"));
        assert!(!hcl_is_valid("#123abz"));
        assert!(!hcl_is_valid("123abc"));

        assert!(ecl_is_valid("brn"));
        assert!(!ecl_is_valid("wat"));

        assert!(pid_is_valid("000000001"));
        assert!(!pid_is_valid("0123456789"));
    }

    #[test]
    fn valids() {
        let input = "tests/day4/valids";
        let passports = load_passports(input).expect("can't read input");
        assert!(4 == solve_part1(&passports));
        assert!(4 == solve_part2(&passports));
    }

    #[test]
    fn invalids2() {
        let input = "tests/day4/invalids2";
        let passports = load_passports(input).expect("can't read input");
        assert!(0 == solve_part2(&passports));
    }
}
