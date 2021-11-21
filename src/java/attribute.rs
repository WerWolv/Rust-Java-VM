#![allow(dead_code)]

use std::io::Cursor;
use binrw::binrw;
use binrw::BinRead;
use crate::java::ClassFile;

#[binrw]
#[derive(Debug)]
#[br(big)]
pub struct ExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16
}

#[binrw]
#[derive(Debug)]
#[br(big)]
pub struct AttributeCode {
    pub max_stack: u16,
    pub max_locals: u16,

    pub code_length: u32,
    #[br(count = code_length)]
    pub code: Vec<u8>,

    pub exception_table_length: u16,
    #[br(count = exception_table_length)]
    pub exception_table: Vec<ExceptionTable>,

    pub attributes_count: u16,
    #[br(count = attributes_count)]
    pub attributes: Vec<AttributeInfo>
}

#[derive(Debug)]
pub enum Attribute {
    ConstantValue(),
    Code(AttributeCode)
}

#[binrw]
#[br(big)]
#[derive(Debug)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,

    #[br(count = attribute_length)]
    pub info: Vec<u8>
}

impl Attribute {

    pub fn new(class_file: &ClassFile, attribute_info: &AttributeInfo) -> Option<Self> {
        if let Some(type_string) = class_file.get_constant_pool_string(attribute_info.attribute_name_index as usize) {
            if type_string == "Code" {
                if let Ok(attribute) = AttributeCode::read(&mut Cursor::new(&attribute_info.info)) {
                    return Some(Attribute::Code(attribute));
                }
            } else {
                println!("Unimplemented attribute '{}'!", type_string);
            }
        }

        None
    }

}