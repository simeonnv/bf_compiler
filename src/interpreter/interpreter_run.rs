use crate::{ARGS, Operation};
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    time::Instant,
};

pub fn interpeter_run(operations: &[Operation]) {
    let mut cells = vec![0_u8; ARGS.stack_size];
    let mut cell_counter: usize = 0;

    let loop_map = build_loop_map(operations);
    let mut ip: usize = 0; // instruction pointer

    let runtime_timer = Instant::now();

    while ip < operations.len() {
        match &operations[ip] {
            Operation::Right(e) => {
                cell_counter += e;
            }
            Operation::Left(e) => {
                cell_counter -= e;
            }
            Operation::Add(e) => {
                cells[cell_counter] = cells[cell_counter].wrapping_add(*e);
            }
            Operation::Sub(e) => {
                cells[cell_counter] = cells[cell_counter].wrapping_sub(*e);
            }
            Operation::Output => {
                use std::io::Write;
                std::io::stdout()
                    .write_all(&cells[cell_counter..cell_counter + 1])
                    .expect("failed to output");
            }
            Operation::Input => {
                use std::io::Read;
                std::io::stdin()
                    .read_exact(&mut cells[cell_counter..cell_counter + 1])
                    .expect("failed to input");
            }
            Operation::LoopStart => {
                if cells[cell_counter] == 0 {
                    ip = *loop_map.get(&ip).unwrap();
                }
            }
            Operation::LoopEnd => {
                if cells[cell_counter] != 0 {
                    ip = *loop_map.get(&ip).unwrap();
                }
            }
            Operation::Zero => cells[cell_counter] = 0,
        }
        ip += 1;
    }

    let duration = runtime_timer.elapsed();

    if ARGS.time {
        println!("runtime execution time was: {:#?}", duration)
    }
}

fn build_loop_map(operations: &[Operation]) -> HashMap<usize, usize> {
    let mut stack = Vec::new();
    let mut map = HashMap::new();

    for (i, op) in operations.iter().enumerate() {
        match op {
            Operation::LoopStart => stack.push(i),
            Operation::LoopEnd => {
                let start = stack.pop().expect("Unmatched ']' in code");
                map.insert(start, i);
                map.insert(i, start);
            }
            _ => {}
        }
    }

    assert!(stack.is_empty(), "Unmatched '[' in code");
    map
}
