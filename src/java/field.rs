#![allow(dead_code)]

use crate::java;
use crate::java::Attribute;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub descriptor: String,

    pub attributes: Vec<java::Attribute>
}

impl Field {

    pub fn new(class_file: &java::ClassFile, field_info: &java::class::FieldInfo) -> Option<Self> {
        if let Some(name) = class_file.get_constant_pool_string(field_info.name_index as usize) {
            if let Some(descriptor) = class_file.get_constant_pool_string(field_info.descriptor_index as usize) {
                let mut attributes = vec![];

                for attribute in &field_info.attributes {
                    if let Some(attribute) = Attribute::new(&class_file, &attribute) {
                        attributes.push(attribute);
                    }
                }

                return Some(Field {
                    name,
                    descriptor,
                    attributes
                })
            }
        }

        None
    }

}