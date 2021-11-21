#![allow(dead_code)]

use std::collections::HashMap;
use binrw::BinRead;
use std::io::Cursor;
use binrw::binrw;

use crate::java;
use crate::java::Method;

#[binrw]
#[br(big)]
#[derive(Debug)]
pub enum ConstantPoolEntry {
    #[br(magic(1u8))]
    String {
        length: u16,

        #[br(count = length)]
        string: Vec<u8>
    },
    #[br(big, magic(3u8))]
    Integer(i32),
    #[br(big, magic(4u8))]
    Float(f32),
    #[br(big, magic(5u8))]
    Long(i64),
    #[br(big, magic(6u8))]
    Double(f64),
    #[br(big, magic(7u8))]
    ClassReference(u16),
    #[br(big, magic(8u8))]
    StringReference(u16),
    #[br(big, magic(9u8))]
    FieldReference(u16, u16),
    #[br(big, magic(10u8))]
    MethodReference(u16, u16),
    #[br(big, magic(11u8))]
    InterfaceMethodReference(u32),
    #[br(big, magic(12u8))]
    NameAndTypeDescriptor(u16, u16),
    #[br(big, magic(15u8))]
    MethodHandle(u8, u16),
    #[br(big, magic(16u8))]
    MethodType(u32),
    #[br(big, magic(17u8))]
    Dynamic(u32),
    #[br(big, magic(18u8))]
    InvokeDynamic(u32),
    #[br(big, magic(19u8))]
    Module(u16),
    #[br(big, magic(20u8))]
    Package(u16),
}

#[binrw]
#[br(big)]
#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,

    #[br(count = attributes_count)]
    pub attributes: Vec<java::AttributeInfo>
}

#[binrw]
#[br(big)]
#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,

    #[br(count = attributes_count)]
    pub attributes: Vec<java::AttributeInfo>
}

#[binrw]
#[derive(Debug)]
#[br(big, magic = b"\xCA\xFE\xBA\xBE")]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,

    pub constant_pool_count: u16,

    #[br(big, count = constant_pool_count - 1)]
    pub constant_pool: Vec<ConstantPoolEntry>,

    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,

    pub interface_count: u16,
    #[br(count = interface_count)]
    pub interface_table: Vec<u16>,

    pub field_count: u16,
    #[br(count = field_count)]
    pub field_table: Vec<FieldInfo>,

    pub method_count: u16,
    #[br(count = method_count)]
    pub method_table: Vec<MethodInfo>,

    pub attribute_count: u16,
    #[br(count = attribute_count)]
    pub attribute_table: Vec<java::AttributeInfo>
}

#[derive(Debug)]
pub struct Class {
    pub class_file: ClassFile,
    pub methods: HashMap<String, java::Method>
}

impl Class {

    fn parse_methods(class_file: &ClassFile) -> HashMap<String, java::Method> {
        let mut result = HashMap::new();

        for method_info in &class_file.method_table {
            let method = Method::new(&class_file, method_info);
            if let Some(method) = method {
                result.insert(method.name.clone(), method);
            }

        }

        result
    }

    pub fn new(data: &Vec<u8>) -> Option<Self> {
        let class_file = ClassFile::read(&mut Cursor::new(&data));
        if let Ok(class_file) = class_file {
            let methods = Self::parse_methods(&class_file);
            Some(Class {
                class_file,
                methods
            })
        } else {
            None
        }
    }

}

impl ClassFile {

    pub fn get_constant_pool_string(&self, index: usize) -> Option<String> {
        if let Some(attribute_type) = self.constant_pool.get((index - 1) as usize) {
            if let ConstantPoolEntry::String { length: _, string } = attribute_type {
                if let Ok(string) = String::from_utf8(string.to_vec()) {
                    return Some(string);
                }
            }
        }

        None
    }

}