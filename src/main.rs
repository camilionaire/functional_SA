use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Hello, world!");

    let path = "datasets/five.txt";
    let input = File::open(path).expect("whoops!");
    let buffered = BufReader::new(input);

    // let spots = buffered.lines().into_iter().count();
    let lines: Vec<_> = buffered.lines().filter_map(io::Result::ok).collect();
    let spots = lines.len();

    let mut table = vec![vec![0f64; spots]; spots];

    // println!("Here is our table:\n{:?}", table);

    for (i, line) in lines.iter().enumerate() {
        // println!("{:?} {}", line, i);
        let items: Vec<_> = line.split_whitespace().collect();
        for (j, val) in items.iter().enumerate() {
            table[i][j] = val.parse().unwrap();
        }
    }

    for (i, val) in table.iter().enumerate() {
        println!("{:?}, {}", val, i);
    }

    // let data = fs::read_to_string(path).expect("whoops");
    // println!("The data:\n{}", data);

    println!("Random number: {}", rng.gen_range(0..10));

    let test = vec![0, 3, 2, 1, 4];
    let test2 = vec![0, 1, 2, 4, 3];

    let total = find_tour_len(&test, &table);
    let total2 = find_tour_len(&test2, &table);
    println!("Our path total was: {}, should be: 21?", total);
    println!("Our path total was: {}, should be: 23?", total2);

    let total3 = find_tour_rec(&test, &table, 0);
    let total4 = find_tour_rec(&test2, &table, 0);
    println!("Our path total was: {}, should be: 21?", total3);
    println!("Our path total was: {}, should be: 23?", total4);

    assert!(is_better(&test, &test2, &table));
}

/// Non recursive, non functional like function.
/// Returns the length of a particular tour for the tabel / map.
fn find_tour_len(arr: &[usize], table: &[Vec<f64>]) -> f64 {
    let mut total: f64 = 0.0;
    for i in 0..arr.len() {
        if i == (arr.len() - 1) {
            total += table[arr[i]][arr[0]];
        } else {
            total += table[arr[i]][arr[i + 1]];
        }
    }
    total
}

/// Recursive functional style of finding the tour length
/// of a any TSP path.  Not sure if there is a better way that only
/// takes two arguments.
fn find_tour_rec(arr: &[usize], table: &[Vec<f64>], pos: usize) -> f64 {
    if pos == arr.len() - 1 {
        table[arr[pos]][arr[0]]
    } else {
        table[arr[pos]][arr[pos + 1]] + find_tour_rec(arr, table, pos + 1)
    }
}

/// Returns true if arr1 tour is smaller than arr2.
/// Returns false otherwise.
fn is_better(arr1: &[usize], arr2: &[usize], table: &[Vec<f64>]) -> bool {
    find_tour_rec(arr1, table, 0) <= find_tour_rec(arr2, table, 0)
}

// fn create_rando_arr(size: usize) -> Vec<usize> {
//     todo()!;
// }