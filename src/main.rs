#![allow(unused)]

use std::io::{ Result, BufReader };
use std::io::prelude::*;
use std::fs::File;

fn read_lines() -> Result<()> {
  let file = File::open("README.md")?;
  let reader = BufReader::new(file);
  for line in reader.lines() {
    println!("{}", line?); // 改行は含まない
  }
  return Ok(())
}

fn main (){
    read_lines();
}
