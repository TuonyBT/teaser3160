use std::{collections::{HashSet, HashMap}, usize};
use itertools::Itertools;

fn main() {

    let primes = primes(4000);

    let mut triplets = HashMap::<Vec<usize>, HashSet<usize>>::new();
    let mut first_digits = HashMap::<usize, Vec<Vec<usize>>>::new();

    for s in 11usize..32 {
        let ss = s.pow(2);

        let ds = digi_set(ss);
        if ds.len() == 3 {
            triplets.entry(ds).or_insert(HashSet::<usize>::new()).insert(ss);

        }
    }
    for (k, v) in triplets {
        if v.len() > 1 {
            println!("Triplet with more than one square {:?}", (&k, &v));
            let mut nsq = HashSet::<usize>::new();
            for pm in k.iter().permutations(3) {
                nsq.insert(dotp(pm, vec![100, 10, 1]));
            }
            println!("Non-squares from this triplet {:?}", nsq.difference(&v));
            println!();
            for first in &v {
                for mut rest in nsq.difference(&v).combinations(3) {
                    rest.insert(0, first);
                    let hymn_sum = &rest.iter().map(|&&x| x).sum::<usize>();
                    if primes.contains(&hymn_sum) {
                        let fd = first / 100;
                        first_digits.entry(fd).or_insert(Vec::<Vec<usize>>::new())
                        .push(rest.iter().map(|&&z| z).collect());
                    }
                }
            }
        }

    }
    println!("Sequences that match the requirement to sum to a prime, grouped by first digit of first hymn: {:?}", first_digits);
    for fd in first_digits {
        if fd.1.len() == 1 {
            println!("The number of the first hymn was {:?}.", fd.1[0][0]);
        }
    }

}

fn digi_set(n: usize) -> Vec<usize> {
    let mut ds = n;
    let mut st = HashSet::<usize>::new();
    let mut order: usize = 10usize.pow((n.ilog(10)) / 1);
    while order >= 1 {
        let d = ds / order;
        ds = ds % order;
        st.insert(d);
        order /= 10;
    }
    let mut st_v = st.iter().map(|&z| z).collect::<Vec<usize>>();
    st_v.sort();
    st_v
}

fn dotp(v: Vec<&usize>, ords: Vec<usize>) -> usize {
    let dot: usize = v.iter().zip(ords).map(|(&&x, y)| x * y).sum();
    dot
}

fn primes(x: usize) -> Vec<usize> {
    let mut sieve = vec![true; x + 1];
    let mut lp: usize = 2;
    while lp <= (x as f64).sqrt().floor() as usize {
        let fnp = lp.pow(2);
        for idx in (fnp..sieve.len()).step_by(lp) {
            sieve[idx] = false;
        }
        lp = match sieve[lp + 1..].iter().position(|z| z == &true) {
            Some(y) => y + lp + 1,
            None => x,
        };
    }
    let primes = sieve.iter().enumerate().filter(|z| z.1 == &true).map(|z| z.0).collect::<Vec<usize>>();
    primes
}