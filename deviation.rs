use std::fs::File;
use std::io::{BufRead, BufReader};
use std::f32;

fn main() {
    let filename = "inp2.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut amount = 0.0;
    let mut data: Vec<f32> = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let number = line.parse::<f32>().unwrap();
        data.push(number);
        amount += number;
        println!("{}. {}", index + 1, number);
    }
    let length = data.len() as f32;
    let average = amount / length;
    let mut amount2 = 0.0;
    for number in data{
      amount2 += (number - average) * (number - average);
    }
    let mut _result = amount2 / length;
    println!("{}", _result.sqrt());
}
