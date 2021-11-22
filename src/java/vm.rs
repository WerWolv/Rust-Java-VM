#![allow(dead_code)]

use crate::java;
use crate::java::class::ConstantPoolEntry;
use crate::java::opcodes::Opcode;

#[derive(Clone)]
pub enum Value {
    None,
    Reference(u32),
    Integer(u32),
    Float(u32)
}

pub struct Scope {
    pub program_counter: u32,

    pub locals: Vec<Value>,

    pub stack: Vec<Value>,
    pub stack_pointer: u32
}

pub struct VirtualMachine {
    pub main_jar: java::Jar,
    pub library_jars: Vec<java::Jar>,

    pub curr_class_name: String
}

impl VirtualMachine {

    pub fn new(jar: java::Jar) -> Option<Self> {
        if let Some((name, class)) = jar.get_main_class() {
            return Some(VirtualMachine {
                main_jar: jar,
                library_jars: vec![],
                curr_class_name: name
            });
        } else {
            println!("Cannot find main class!");
        }

        None
    }

    pub fn add_library_jar(&mut self, jar: java::Jar) {
        self.library_jars.push(jar);
    }

    fn get_constant_pool_entry(&self, index: usize) -> Option<&ConstantPoolEntry> {
        Some(&self.get_current_class().class_file.constant_pool[(index - 1) as usize])
    }

    fn get_current_class(&self) -> &java::Class {
        self.main_jar.classes.get(&*self.curr_class_name).unwrap()
    }

    fn execute_byte_code(&self, byte_code: &Vec<u8>, scope: &mut Scope) {
        let mut pc: usize = 0;

        let option = |index: usize| -> u32 { *byte_code.get(index).unwrap() as u32 };

        while pc < byte_code.len() {
            let value = option(pc) as u8;
            let opcode : Opcode = unsafe { std::mem::transmute(value) };

            println!("    0x{:02X} {}", value, opcode);

            match opcode {
                Opcode::getstatic => {
                    let index = option(pc + 1) << 8 | option(pc + 2);
                    if let Some(ConstantPoolEntry::FieldReference(class_index, name_and_type_index)) = self.get_constant_pool_entry((index) as usize) {
                        if let Some(ConstantPoolEntry::ClassReference(name_index)) = self.get_constant_pool_entry(*class_index as usize) {
                            println!("{}", self.get_current_class().class_file.get_constant_pool_string(*name_index as usize).unwrap());
                        }
                        if let Some(ConstantPoolEntry::NameAndTypeDescriptor(name_index, type_index)) = self.get_constant_pool_entry(*name_and_type_index as usize) {
                            println!("{}", self.get_current_class().class_file.get_constant_pool_string(*name_index as usize).unwrap());
                            println!("{}", self.get_current_class().class_file.get_constant_pool_string(*type_index as usize).unwrap());
                        }
                    } else {
                        println!("Invalid constant pool entry!");
                    }
                },
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

                    locals: vec![Value::None; code_attribute.max_locals as usize],

                    stack: vec![Value::None; code_attribute.max_stack as usize],
                    stack_pointer: 0
                };

                self.execute_byte_code(&code_attribute.code, &mut scope);

                return;
            }
        }

        println!("Method '{}' does not have a Code attribute!", method.name);
    }

    pub fn run(&self) {
        if let Some(main_method) = self.main_jar.classes.get(&*self.curr_class_name).unwrap().methods.get("main") {
            self.execute_method(main_method);
        } else {
            println!("Cannot find main method!");
        }
    }

}