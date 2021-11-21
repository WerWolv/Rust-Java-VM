#![allow(dead_code)]

use crate::java;
use crate::java::opcodes::Opcode;

pub struct Scope {
    pub program_counter: u32,

    pub locals: Vec<u32>,

    pub stack: Vec<u32>,
    pub stack_pointer: u32
}

pub struct VirtualMachine {
    pub jar: java::Jar,
}

impl VirtualMachine {

    pub fn new(jar: java::Jar) -> Self {
        VirtualMachine {
            jar
        }
    }

    fn execute_byte_code(&self, byte_code: &Vec<u8>, scope: &mut Scope) {
        let mut pc = 0;
        while pc < byte_code.len() {
            let value = byte_code.get(pc).unwrap();
            let opcode : Opcode = unsafe { std::mem::transmute(*value) };

            println!("    0x{:02X} {} ", value, opcode);

            match opcode {
                Opcode::nop => { },
                _ => {}//panic!("Invalid opcode!")
            }

            pc += opcode.instruction_length() + 1;
        }
    }

    fn execute_method(&self, method: &java::Method) {
        println!("Executing method '{}' [ {} ]", method.name, method.descriptor);
        for attribute in &method.attributes {
            if let java::Attribute::Code(code_attribute) = attribute {
                println!("  Stack Size:  {}", code_attribute.max_stack);
                println!("  Locals Size: {}", code_attribute.max_locals);

                let mut scope = Scope {
                    program_counter: 0,

                    locals: vec![0u32; code_attribute.max_locals as usize],

                    stack: vec![0u32; code_attribute.max_stack as usize],
                    stack_pointer: 0
                };

                self.execute_byte_code(&code_attribute.code, &mut scope);

                return;
            }
        }

        println!("Method '{}' does not have a Code attribute!", method.name);
    }

    pub fn run(&self) {
        if let Some(main_class) = self.jar.get_main_class() {
            if let Some(main_method) = main_class.methods.get("main") {
                self.execute_method(main_method);
            } else {
                println!("Cannot find main method!");
            }
        } else {
            println!("Cannot find main class!");
        }
    }

}