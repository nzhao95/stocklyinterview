use std::collections::BinaryHeap;
use std::io;


pub fn extract(input : &str) -> (usize, Vec<usize>){
    let mut lines = input.lines();
    let n = lines.next()
    .expect("No first line")
    .parse::<usize>()
    .unwrap_or_else(|_| panic!("First line is not an integer"));

    let list : Vec<usize> = lines.next()
    .expect("No second line on input")
    .split(' ')
    .map(|x| x.parse::<usize>().expect("Found something that is not an int"))
    .collect();

    (n, list)
}

pub fn format(vector : Vec<usize>) -> String {
    let mut result = String::new();
    //Might be slow could find better way
    for i in vector.iter() {
        result = format!("{} {}", result, *i);
    }

    result.strip_prefix(" ").unwrap().to_owned()
}

#[derive(Debug, Eq, PartialEq) ]
struct Intersection {
    index : usize,
    distance : usize
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn compute_dist(shortcuts : Vec<usize>, n : usize) -> Vec<usize> {
    let mut distances: Vec<usize> = vec![usize::MAX; n];
    distances[0] = 0;
    let mut min_heap : BinaryHeap<Intersection> = BinaryHeap::new();
    min_heap.push(Intersection{ index : 0, distance : 0});

    while let Some(Intersection{index : curr_index, distance : curr_dist}) = min_heap.pop() {
        //If this happens it means we found a shorted path
        if curr_dist > distances[curr_index] {
            continue;
        }

        //Test the destination which should become the next shortest destination
        let destination = shortcuts[curr_index] - 1;

        if curr_dist + 1 < distances[destination] {
            min_heap.push(Intersection{index : destination, distance : curr_dist + 1});
            distances[destination] = curr_dist + 1;
        }

        //But also the next value
        if curr_index < n-1 && curr_dist + 1 < distances[curr_index + 1] {
            min_heap.push(Intersection{index : curr_index + 1, distance : curr_dist + 1});
            distances[curr_index + 1] = curr_dist + 1;
        }

    }

    distances
}

fn main() {
}


#[test]
fn test_extract() {
    let input = "\
3
2 2 3";

    assert_eq!(extract(input), (3, vec![2,2,3]));
}

#[test]
fn test_to_owned() {
    let input = vec![0,1,2];
    assert_eq!(format(input), "0 1 2".to_owned()) 
}

#[test]
fn test_distances_1() {
    
    let n = 3;
    let shortcuts = vec![2,2,3];

    assert_eq!(compute_dist(shortcuts, n), vec![0,1,2]);
}
#[test]
fn test_distances_2() {
    
    let n = 5;
    let shortcuts = vec![1,2,3,4,5];

    assert_eq!(compute_dist(shortcuts, n), vec![0,1,2,3,4]);
}
#[test]
fn test_distances_3() {
    
    let n = 7;
    let shortcuts = vec![4,4,4,4,7,7,7];

    assert_eq!(compute_dist(shortcuts, n), vec![0,1,2,1,2,3,3]);
}