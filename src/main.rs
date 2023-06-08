use std::io::{self, Read};
use hex::FromHex;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug, PartialEq)]
pub enum Operation {
  ADD, // 0x01
  MUL, //0x02
  SUB, //0x03
  DIV, //0x04
  MOD, //0x06
  EXP, //0x0A
  POP, //0x50
  PUSH32(u32), //0x7F
  STOP, //0x00
  RETURN, //0xF3
}

fn main() {
    println!("Input your program");
    
    let mut input = String::new();

    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    let input = Vec::from_hex(input.trim()).expect("Invalid Hex String");
    println!("{:x?}", input);

    let mut operations = Vec::new();
    parse_loop(&mut operations, &input, 0);
    let output = execute_program(&operations);
    println!("Program output: {}",output);

}
// https://etherscan.io/address/0x3363bae2fc44da742df13cd3ee94b6bb868ea376#code
// 608060405234801561001057600080fd5b50613b77806100206000396000f3fe60806040523480156100105760

// 3 + 4
// 7F 00 00 00 03
// 7F 00 00 00 04
// 01
// 00
// 7F000000037F0000000401F3

// 5*(137+44) - 37 = 868
// 7F000000257F000000897F0000002C017F000000050203F3


pub fn parse_loop(ops: &mut Vec<Operation>, rem: &[u8], i: usize) -> () {
  if i == rem.len() {
    ()
  } else if i <= rem.len() - 1 {
    match rem[i] {
      0x00 => {ops.push(Operation::STOP); parse_loop(ops, rem, i+1)},
      0x01 => {ops.push(Operation::ADD);  parse_loop(ops, rem, i+1)},
      0x02 => {ops.push(Operation::MUL);  parse_loop(ops, rem, i+1)},
      0x03 => {ops.push(Operation::SUB);  parse_loop(ops, rem, i+1)},
      0x04 => {ops.push(Operation::DIV);  parse_loop(ops, rem, i+1)},
      0x06 => {ops.push(Operation::MOD);  parse_loop(ops, rem, i+1)},
      0x0A => {ops.push(Operation::EXP);  parse_loop(ops, rem, i+1)},
      0x50 => {ops.push(Operation::POP);  parse_loop(ops, rem, i+1)},
      0x7F => {
                let mut buf = &rem[i+1..=i+4];
                let num = buf.read_u32::<BigEndian>().unwrap();
                ops.push(Operation::PUSH32(num)); 
                parse_loop(ops, rem, i+5);
              },              
      0xF3 => {ops.push(Operation::RETURN); parse_loop(ops,rem,i+1)}
      _    => panic!("Invalid Program: operation {} not accepted", rem[i]),
    };
  } else {
    panic!("Requested to take more bytes than available")
  }
}


pub fn execute_program(operations: &Vec<Operation>) -> u32 {
  let mut queue: Vec<u32> = Vec::new();
  for operation in operations {
    match operation {
      Operation::PUSH32(x) => queue.push(*x),
      Operation::ADD => binary_op(&mut queue,|a, b| a + b), 
      Operation::MUL => binary_op(&mut queue, |a,b| a * b), 
      Operation::SUB => binary_op(&mut queue, |a,b| a - b), 
      Operation::DIV => binary_op(&mut queue, |a,b| a / b),
      Operation::MOD => binary_op(&mut queue, |a,b| a % b),
      Operation::EXP => binary_op(&mut queue, |a,b| a.pow(b)),
      Operation::POP => {queue.pop(); ()},
      Operation::STOP => break,
      Operation::RETURN => return queue.pop().expect("Error: Nothing to return"),
    }
  }
  panic!("No return from program found!");
}

pub fn binary_op(queue: &mut Vec<u32>, f: fn(u32,u32)->u32) {
  let a = queue.pop().expect("No a found in queue");
  let b = queue.pop().expect("No b found in queue");
  let c = f(a,b);
  queue.push(c);
}
