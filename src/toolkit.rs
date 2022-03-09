use rand::{seq::SliceRandom, Rng, thread_rng};


pub fn create_rando_arr(size: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = (0..size).collect();
    vec.shuffle(& mut thread_rng());
    vec
}

/// Non recursive, non functional like function.
/// Returns the length of a particular tour for the tabel / map.
pub fn find_tour_len(arr: &[usize], table: &[Vec<f64>]) -> f64 {
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
pub fn find_tour_rec(arr: &[usize], table: &[Vec<f64>], pos: usize) -> f64 {
    if pos == arr.len() - 1 {
        table[arr[pos]][arr[0]]
    } else {
        table[arr[pos]][arr[pos + 1]] + find_tour_rec(arr, table, pos + 1)
    }
}

/// Returns true if arr1 tour is smaller than arr2.
/// Returns false otherwise.
pub fn is_better(arr1: &[usize], arr2: &[usize], table: &[Vec<f64>]) -> bool {
    find_tour_rec(arr1, table, 0) <= find_tour_rec(arr2, table, 0)
}

pub fn decision(prob: f64) -> bool {
    thread_rng().gen::<f64>() < prob
}

#[test]
fn decision_test() {
    let mut count = 0;
    for _ in 0..1000 {
        if decision(0.5) {
            count += 1;
        }
    }
    assert!(count < 550);
    assert!(count > 450);
}