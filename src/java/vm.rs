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
    pub program_counter: usize,

    pub locals: Vec<Value>,

    pub stack: Vec<Value>,
    pub stack_pointer: usize
}

pub struct Executor {

}

pub struct VirtualMachine {
    pub main_jar: java::Jar,
    pub library_jars: Vec<java::Jar>,

    pub curr_class_name: String,

    executor: Executor
}

impl Executor {

    fn get_constant_pool_entry<'a>(&self, class: &'a java::Class, index: usize) -> Option<&'a ConstantPoolEntry> {
        Some(&class.class_file.constant_pool[(index - 1) as usize])
    }

    fn execute_byte_code(&self, class: &java::Class, byte_code: &Vec<u8>, scope: &mut Scope) {
        let option = |index: usize| -> u32 { *byte_code.get(index).unwrap() as u32 };

        while scope.program_counter < byte_code.len() {
            let value = option(scope.program_counter) as u8;
            let opcode : Opcode = unsafe { std::mem::transmute(value) };

            println!("    0x{:02X} {}", value, opcode);

            match opcode {
                Opcode::getstatic => {
                    let index = option(scope.program_counter + 1) << 8 | option(scope.program_counter + 2);
                    if let Some(ConstantPoolEntry::FieldReference(class_index, name_and_type_index)) = self.get_constant_pool_entry(class, (index) as usize) {
                        if let Some(ConstantPoolEntry::ClassReference(name_index)) = self.get_constant_pool_entry(class, *class_index as usize) {
                            println!("{}", class.class_file.get_constant_pool_string(*name_index as usize).unwrap());
                        }
                        if let Some(ConstantPoolEntry::NameAndTypeDescriptor(name_index, type_index)) = self.get_constant_pool_entry(class, *name_and_type_index as usize) {
                            println!("{}", class.class_file.get_constant_pool_string(*name_index as usize).unwrap());
                            println!("{}", class.class_file.get_constant_pool_string(*type_index as usize).unwrap());
                        }
                    } else {
                        println!("Invalid constant pool entry!");
                    }
                },
                Opcode::nop => { },
                _ => {}//panic!("Invalid opcode!")
            }

            scope.program_counter += opcode.instruction_length() + 1;
        }
    }

    pub fn execute_method(&self, class : &java::Class, method: &java::Method) {
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

                self.execute_byte_code(class, &code_attribute.code, &mut scope);

                return;
            }
        }

        println!("Method '{}' does not have a Code attribute!", method.name);
    }

}

impl VirtualMachine {

    pub fn new(jar: java::Jar) -> Option<Self> {
        if let Some((name, _)) = jar.get_main_class() {
            return Some(VirtualMachine {
                main_jar: jar,
                library_jars: vec![],
                curr_class_name: name,
                executor: Executor { }
            });
        } else {
            println!("Cannot find main class!");
        }

        None
    }

    pub fn add_library_jar(&mut self, jar: java::Jar) {
        self.library_jars.push(jar);
    }

    pub fn run(&mut self) {
        if let Some(class) = self.main_jar.classes.get_mut(&self.curr_class_name) {
            if let Some(init_method) = class.methods.get("<init>") {
                if !class.initialized {
                    self.executor.execute_method(class, init_method);
                    class.initialized = true;
                }
            } else {
                panic!("Cannot find <init> method!");
            }

            if let Some(method) = class.methods.get("main") {
                self.executor.execute_method(class, method);
            } else {
                panic!("Cannot find main method!");
            }
        } else {
            panic!("Cannot find main class!");
        }
    }

}