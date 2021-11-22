#![allow(dead_code)]

use std::io::Cursor;
use binrw::binrw;
use binrw::BinRead;
use crate::java::ClassFile;

#[binrw]
#[derive(Debug)]
#[br(big)]
pub struct AttributeConstantValue {
    pub constantvalue_index: u16
}

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

#[binrw]
#[derive(Debug)]
#[br(big)]
pub struct AttributeException {
    pub number_of_exceptions: u16,

    #[br(count = number_of_exceptions)]
    pub exception_index_table: Vec<u16>
}

#[binrw]
#[br(big)]
#[derive(Debug)]
pub enum ElementValue {
    #[br(magic('B'))]
    Byte {
        const_value_index: u16
    },
    #[br(magic('C'))]
    Char {
        const_value_index: u16
    },
    #[br(magic('D'))]
    Double {
        const_value_index: u16
    },
    #[br(magic('F'))]
    Float {
        const_value_index: u16
    },
    #[br(magic('I'))]
    Int {
        const_value_index: u16
    },
    #[br(magic('J'))]
    Long {
        const_value_index: u16
    },
    #[br(magic('S'))]
    Short {
        const_value_index: u16
    },
    #[br(magic('Z'))]
    Boolean {
        const_value_index: u16
    },
    #[br(magic('s'))]
    String {
        const_value_index: u16
    },
    #[br(magic('e'))]
    Enum {
        type_name_index: u16,
        const_name_index: u16
    },
    #[br(magic('c'))]
    Class {
        class_info_index: u16
    },
    #[br(magic('@'))]
    AnnotationType {
        annotation_value: Annotation
    },
    #[br(magic('['))]
    Array {
        num_values: u16,

        #[br(count = num_values)]
        element_value: Vec<ElementValue>
    },
}

#[binrw]
#[br(big)]
#[derive(Debug)]
pub struct ElementValuePair {
    pub element_name_index: u16,
    pub value: ElementValue
}

#[binrw]
#[derive(Debug)]
#[br(big)]
pub struct Annotation {
    pub type_index: u16,
    pub num_element_value_pairs: u16,

    #[br(count = num_element_value_pairs)]
    pub element_value_pairs: Vec<ElementValuePair>
}

#[binrw]
#[derive(Debug)]
#[br(big)]
pub struct AttributeRuntimeVisibleAnnotations {
    pub num_annotations: u16,

    #[br(count = num_annotations)]
    pub annotations: Vec<Annotation>
}

#[binrw]
#[derive(Debug)]
#[br(big)]
pub struct AttributeRuntimeInvisibleAnnotations {
    pub num_annotations: u16,

    #[br(count = num_annotations)]
    pub annotations: Vec<Annotation>
}

#[binrw]
#[derive(Debug)]
#[br(big)]
pub struct AttributeSignature {
    pub signature_index: u16
}

#[binrw]
#[derive(Debug)]
#[br(big)]
pub struct AttributeDeprecated {

}

#[binrw]
#[derive(Debug)]
#[br(big)]
pub struct AttributeAnnotationDefault {
    pub default_value: ElementValue
}

#[binrw]
#[derive(Debug)]
#[br(big)]
pub struct MethodParameter {
    pub name_index: u16,
    pub access_flags: u16
}

#[binrw]
#[derive(Debug)]
#[br(big)]
pub struct AttributeMethodParameters {
    pub parameters_count: u8,

    #[br(count = parameters_count)]
    pub parameters: Vec<MethodParameter>
}

#[binrw]
#[derive(Debug)]
#[br(big)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16
}

#[binrw]
#[derive(Debug)]
#[br(big)]
pub struct AttributeLineNumberTable {
    pub line_number_table_length: u16,

    #[br(count = line_number_table_length)]
    pub parameters: Vec<LineNumber>
}

#[derive(Debug)]
pub enum Attribute {
    ConstantValue(AttributeConstantValue),
    Code(AttributeCode),
    Exceptions(AttributeException),
    RuntimeVisibleAnnotations(AttributeRuntimeVisibleAnnotations),
    RuntimeInvisibleAnnotations(AttributeRuntimeInvisibleAnnotations),
    Signature(AttributeSignature),
    Deprecated(AttributeDeprecated),
    AnnotationDefault(AttributeAnnotationDefault),
    MethodParameters(AttributeMethodParameters),
    LineNumberTable(AttributeLineNumberTable)
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
            match type_string.trim() {
                "ConstantValue" => {
                    if let Ok(attribute) = AttributeConstantValue::read(&mut Cursor::new(&attribute_info.info)) {
                        return Some(Attribute::ConstantValue(attribute));
                    }
                },
                "Code" => {
                    if let Ok(attribute) = AttributeCode::read(&mut Cursor::new(&attribute_info.info)) {
                        return Some(Attribute::Code(attribute));
                    }
                },
                "Exceptions" => {
                    if let Ok(attribute) = AttributeException::read(&mut Cursor::new(&attribute_info.info)) {
                        return Some(Attribute::Exceptions(attribute));
                    }
                },
                "RuntimeVisibleAnnotations" => {
                    if let Ok(attribute) = AttributeRuntimeVisibleAnnotations::read(&mut Cursor::new(&attribute_info.info)) {
                        return Some(Attribute::RuntimeVisibleAnnotations(attribute));
                    }
                },
                "RuntimeInvisibleAnnotations" => {
                    if let Ok(attribute) = AttributeRuntimeInvisibleAnnotations::read(&mut Cursor::new(&attribute_info.info)) {
                        return Some(Attribute::RuntimeInvisibleAnnotations(attribute));
                    }
                },
                "Signature" => {
                    if let Ok(attribute) = AttributeSignature::read(&mut Cursor::new(&attribute_info.info)) {
                        return Some(Attribute::Signature(attribute));
                    }
                },
                "Deprecated" => {
                    if let Ok(attribute) = AttributeDeprecated::read(&mut Cursor::new(&attribute_info.info)) {
                        return Some(Attribute::Deprecated(attribute));
                    }
                },
                "AnnotationDefault" => {
                    if let Ok(attribute) = AttributeAnnotationDefault::read(&mut Cursor::new(&attribute_info.info)) {
                        return Some(Attribute::AnnotationDefault(attribute));
                    }
                },
                "MethodParameters" => {
                    if let Ok(attribute) = AttributeMethodParameters::read(&mut Cursor::new(&attribute_info.info)) {
                        return Some(Attribute::MethodParameters(attribute));
                    }
                },
                "LineNumberTable" => {
                    if let Ok(attribute) = AttributeLineNumberTable::read(&mut Cursor::new(&attribute_info.info)) {
                        return Some(Attribute::LineNumberTable(attribute));
                    }
                },
                _ => println!("Unimplemented attribute '{}'!", type_string)
            };

            println!("Failed to parse '{}' attribute", type_string);
        }

        None
    }

}