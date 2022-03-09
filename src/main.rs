use rand::{seq::SliceRandom, Rng, thread_rng};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec;
use std::cmp::Ordering::{Equal, Less, Greater};

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
    // println!("Random number: {}", rng.gen_range(0..size));


    let test1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let new1 = find_neighbor(&test1);

    println!("old: {:?}", &test1);
    println!("new: {:?}", new1);

}

fn find_neighbor(og_arr: &[usize]) -> Vec<usize>{
    let mut rng = thread_rng();
    let r1 = rng.gen_range(0..og_arr.len());
    let r2 = rng.gen_range(0..og_arr.len());
    let mut na = og_arr.to_owned();

    match r1.cmp(&r2) {
        Less => {
            na[r1..=r2].shuffle(& mut rng);
        },
        Greater => {
        na[..=r1].shuffle(& mut rng);
        na[r2..].shuffle(& mut rng);
        },
        Equal => {
            na.shuffle(& mut rng);
        },
    }
    na
}

