use std::fs::File;
use std::io::{BufRead, BufReader};
use marathon592::data_structs::BinaryTree;


pub fn main() {
    let mut tree: BinaryTree<u64> = BinaryTree::new();
    let mut input_file = BufReader::new(File::open("data/p0018.txt").unwrap());

    let mut s = String::new();
    input_file.read_line(&mut s).unwrap();

    // handle error in case of bad file ?
    input_file.lines().for_each(|l| l.unwrap().split(char::is_whitespace)
        .for_each(|number| {
            tree.insert(number.parse().unwrap());
        }));

    println!("{}", tree);
}
