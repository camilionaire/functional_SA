use rand::{seq::SliceRandom, Rng, thread_rng};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec;

mod toolkit;
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
    let size:usize = 10;

    println!("Random number: {}", rng.gen_range(0..size));

    let test = create_rando_arr(5);
    let test2 = create_rando_arr(5);

    println!("rando arr1: {:?}, len is: {}", test, toolkit::find_tour_rec(&test, &table, 0));
    println!("rando arr2: {:?}, len is: {}", test2, toolkit::find_tour_rec(&test2, &table, 0));

    if toolkit::is_better(&test, &test2, &table) {
        println!("first test case is better");
    } else {
        println!("second test case is better");
    }

    // assert!(toolkit::is_better(&test, &test2, &table));
}


fn create_rando_arr(size: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = (0..size).collect();
    vec.shuffle(& mut thread_rng());
    vec
}