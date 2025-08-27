use std::{fs::File, io::Write, mem, time::Instant};

use dynasmrt::{DynasmApi, DynasmLabelApi, VecAssembler, dynasm, x64::X64Relocation};
use object::{
    SymbolFlags,
    write::{Relocation, Symbol},
};

use crate::{ARGS, Operation};

pub fn compile(operations: Box<[Operation]>) {
    let mut code: VecAssembler<X64Relocation> = VecAssembler::new(0);

    let compilation_timer = Instant::now();

    dynasm! { code
        ; .arch x64
        ; push rbp
        ; mov rbp, rsp
        ; xor r13, r13

        // allocates bytes on stack for the bf runtime
        ; sub rsp, ARGS.stack_size as i32
        ; mov r12, rsp

        // zero the bytes
        ; xor eax, eax
        ; mov r11, rbp
        ; loop_:
        ; add r11, -8
        ; mov QWORD [r11], rax
        ; cmp r11, r12
        ; jne <loop_
    };

    let mut loop_stack = Vec::new();
    let stack_size_i32 = ARGS.stack_size as i32;

    for operation in operations {
        match operation {
            Operation::Right(e) => {
                let e = (e as usize) % ARGS.stack_size;
                if e == 0 {
                    continue;
                }
                dynasm! { code
                    ; .arch x64
                    ; add r13, e as i32
                    ; cmp r13d, stack_size_i32
                    ; jb >done_r
                    ; sub r13d, stack_size_i32
                    ; done_r:
                };
            }

            Operation::Left(e) => {
                let e = (e as usize) % ARGS.stack_size;
                if e == 0 {
                    continue;
                }
                dynasm! { code
                    ; .arch x64
                    ; sub r13, e as i32
                    ; cmp r13d, 0
                    ; jge >done_l
                    ; add r13d, stack_size_i32
                    ; done_l:
                };
            }
            Operation::Add(e) => {
                let mut e = e as usize;
                while e >= 127 {
                    dynasm! { code
                        ; .arch x64
                        ; add BYTE [r12 + r13], 127i8
                    };
                    e -= 127;
                }
                if e > 0 {
                    dynasm! { code
                        ; .arch x64
                        ; add BYTE [r12 + r13], (e as i8)
                    };
                }
            }
            Operation::Sub(e) => {
                let mut e = e as usize;
                while e >= 127 {
                    dynasm! { code
                        ; .arch x64
                        ; sub BYTE [r12 + r13], 127i8
                    };
                    e -= 127;
                }
                if e > 0 {
                    dynasm! { code
                        ; .arch x64
                        ; sub BYTE [r12 + r13], (e as i8)
                    };
                }
            }
            Operation::Zero => dynasm! { code
                ; .arch x64
                ; mov BYTE [r12 + r13], 0
            },
            Operation::Input => {
                let after_input = code.new_dynamic_label();
                let set_zero = code.new_dynamic_label();

                dynasm! { code
                    ; .arch x64
                    ; xor rax, rax               // SYS_read = 0
                    ; mov rdi, 0                // fd = 0 (stdin)
                    ; lea rsi, [r12 + r13]      // buffer = &cell
                    ; mov rdx, 1                // count = 1
                    ; syscall
                    ; cmp rax, 0
                    ; jg =>after_input          // if read > 0, skip setting to zero
                    ; =>set_zero
                    ; mov BYTE [r12 + r13], 0
                    ; =>after_input
                };
            }
            Operation::Output => {
                dynasm! { code
                    ; .arch x64
                    ; mov rax, 1                // SYS_write = 1
                    ; mov rdi, 1                // fd = 1 (stdout)
                    ; lea rsi, [r12 + r13]      // buffer = &cell
                    ; mov rdx, 1                // count = 1
                    ; syscall
                };
            }
            Operation::LoopStart => {
                let start_label = code.new_dynamic_label();
                let end_label = code.new_dynamic_label();
                dynasm! { code
                    ; .arch x64
                    ; cmp BYTE [r12+r13], 0
                    ; je =>end_label
                    ; =>start_label
                };

                loop_stack.push((start_label, end_label));
            }
            Operation::LoopEnd => {
                let (start_label, end_label) = match loop_stack.pop() {
                    Some(x) => x,
                    None => panic!(
                        "tryed to end a loop at {}, without a starting one",
                        code.offset().0
                    ),
                };

                dynasm! { code
                    ; .arch x64
                    ; cmp BYTE [r12 + r13], 0
                    ; jne =>start_label
                    ; => end_label
                };
            }
        }
    }
    if !loop_stack.is_empty() {
        panic!("opened a loop, but never closed it!");
    }

    dynasm! { code
        ; .arch x64
        ; mov rax, 60         // SYS_exit
        ; xor rdi, rdi        // status = 0
        ; syscall
    }

    let code = code.finalize().unwrap();

    let mut obj = object::write::Object::new(
        object::BinaryFormat::Elf,
        object::Architecture::X86_64,
        object::Endianness::Little,
    );

    let start = obj.add_symbol(Symbol {
        name: b"_start".to_vec(),
        value: 0,
        size: 0,
        kind: object::SymbolKind::Text,
        scope: object::SymbolScope::Linkage,
        weak: false,
        section: object::write::SymbolSection::Undefined,
        flags: SymbolFlags::None,
    });
    let text = obj.section_id(object::write::StandardSection::Text);
    obj.add_symbol_data(start, text, &code, 16);

    let mut out = Vec::new();
    obj.emit(&mut out).unwrap();

    let duration = compilation_timer.elapsed();
    if ARGS.time {
        println!("compilation time was: {:#?}", duration)
    }

    std::fs::write(&ARGS.output, out).unwrap();
}
