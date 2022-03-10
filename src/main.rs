use libm::exp;
use rand::{thread_rng, Rng};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec;

mod toolkit;
use crate::toolkit::{create_rando_arr, decision, find_tour_len, find_tour_rec, is_better};

const ITERS: u32 = 200_000;
const TEMP_MOD: f64 = 400.0;

fn main() {
    println!("Hello, world!");

    let path = "datasets/att48_33523.txt";
    let input = File::open(path).expect("whoops!");
    let buffered = BufReader::new(input);

    // let spots = buffered.lines().into_iter().count();
    let lines: Vec<_> = buffered.lines().filter_map(io::Result::ok).collect();
    let spots = lines.len();
    let mut table = vec![vec![0f64; spots]; spots];

    for (i, line) in lines.iter().enumerate() {
        let items: Vec<_> = line.split_whitespace().collect();
        for (j, val) in items.iter().enumerate() {
            table[i][j] = val.parse().unwrap();
        }
    }

    let mut current = create_rando_arr(spots);
    let mut compare: Vec<usize>;
    // just to put something in best...
    let mut best: Vec<usize> = create_rando_arr(spots);
    let mut best_tour = f64::MAX;

    println!("Starting the annealing...");

    for i in 0..ITERS {
        compare = find_neighbor(&current);
        if is_better(&compare, &current, &table) {
            current = compare.to_owned();
            let current_tour = find_tour_rec(&current, &table, 0);
            if current_tour < best_tour {
                best_tour = current_tour;
                best = current.to_owned();
            }
        } else {
            let temp: f64 = (f64::from(ITERS - i)) / TEMP_MOD;
            // this will produce negative delta value.
            let delta = find_tour_len(&current, &table) - find_tour_len(&compare, &table);
            let prob = exp(delta / temp);
            if decision(prob) {
                current = compare.to_owned();
            }
        }

        if i % 10000 == 0 {
            // println!("our current 'best' tour:{:?}", current);
            println!(
                "Our current tourlength is: {}",
                find_tour_rec(&current, &table, 0)
            );
        }
    }

    println!("\nOur overall best tour:{:?}", &best);
    println!("It's tourlength is: {}", best_tour);
}

fn find_neighbor(og_arr: &[usize]) -> Vec<usize> {
    let mut rng = thread_rng();
    let r1 = rng.gen_range(0..og_arr.len());
    let r2 = rng.gen_range(0..og_arr.len());
    let mut na = og_arr.to_owned();

    match r1.cmp(&r2) {
        Less => {
            na[r1..=r2].reverse();
        }
        Greater => {
            na[r2..=r1].reverse();
        }
        Equal => {}
    }
    na
}
