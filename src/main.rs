
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

pub fn compute_dist(shortcuts : Vec<usize>, n : usize) -> Vec<usize> {
    let mut distances: Vec<usize> = vec![usize::MAX; n];
    distances[0] = 0;
    let mut queue : Vec<usize> = Vec::new();
    queue.push(0);

    while let Some(curr_index) = queue.pop() {
        //If this happens it means we found a shorted path
        let curr_dist = distances[curr_index];

        //Test the destination which should become the next shortest destination
        let destination = shortcuts[curr_index] - 1;

        //But also the previous value
        if curr_index > 0 && curr_dist + 1 < distances[curr_index - 1] {
            queue.push(curr_index - 1);
            distances[curr_index - 1] = curr_dist + 1;
        }
        //But also the next value
        if curr_index < n-1 && curr_dist + 1 < distances[curr_index + 1] {
            queue.push(curr_index + 1);
            distances[curr_index + 1] = curr_dist + 1;
        }
        if curr_dist + 1 < distances[destination] {
            queue.push(destination);
            distances[destination] = curr_dist + 1;
        }


    }

    distances
}

fn main() -> io::Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    let (n, shortcuts) = extract(&stdin);
    let result = compute_dist(shortcuts, n);
    let output = format(result);
    println!("{output}");
    Ok(())
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