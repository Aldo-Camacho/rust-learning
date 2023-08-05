#![allow(unused)]

use std::process::Output;
use std::{io, path};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    smart_pointers();
}

fn smart_pointers() {
    //BOX is for data in heap with pointers in stack
    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1);

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl <T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode { left: None, right: None, key: key }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(1)
    .left(TreeNode::new(2))
    .right(TreeNode::new(3));
    
}

fn closures() {
    let can_vote = |age: i32| {
        age >= 18
    };
    println!("Can vote: {}", can_vote(18));

    let mut sample1 = 5;
    let print_var = || println!("Sample 1: {}", sample1);
    print_var();
    sample1 = 10;
    let mut change_var = || sample1 += 1;
    change_var();
    println!("Sample 1: {}", sample1);

    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where T: Fn(i32, i32) -> i32 {
        func(a, b)
    }

    let sum = |a, b| a + b;
    let prod= |a, b| a * b;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));
}

fn age_and_name() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age += 1;

    age = rand::thread_rng().gen_range(1..101);

    println!("What's your name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Didn't recieve input");
    println!("Hello {}! {}", name.trim_end(), greeting);
}

fn manipulate_files() {
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error)
        }
    };
    write!(output, "Just some\nRandom words").expect("Failed to write");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    
    for line in buffered.lines(){
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            },
            _other_error => panic!("Problem opening file: {:?}", error),
        },
    };
}