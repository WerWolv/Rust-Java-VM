#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::io::{ BufReader, Read };
use zip::ZipArchive;

use crate::java::class;

#[derive(Debug)]
pub struct Jar {
    pub name: String,

    pub manifest: HashMap<String, String>,
    pub classes: HashMap<String, class::Class>
}

impl Jar {

    pub fn new(jar_path: &str) -> Result<Self, &'static str> {
        let file = fs::File::open(&jar_path);
        return if let Ok(file) = file {
            let reader = BufReader::new(file);
            let archive = zip::ZipArchive::new(reader);

            if let Ok(mut archive) = archive {
                let mut classes = HashMap::new();
                for i in 0..archive.len() {
                    let file = archive.by_index(i);
                    if file.is_err() { continue; }

                    let mut file = file.unwrap();
                    let file_name = file.name().to_string();
                    if !file_name.ends_with(".class") { continue; }

                    let mut file_content: Vec<u8> = vec![];
                    if file.read_to_end(&mut file_content).is_err() { continue; }

                    if let Some(class) = class::Class::new(&file_content) {
                        classes.insert(file_name, class);
                    }

                }

                Ok(Jar {
                    name: jar_path.to_string(),

                    manifest: Self::parse_manifest(&mut archive)?,
                    classes
                })
            } else {
                Err("Failed to parse JAR file")
            }
        } else {
            Err("Failed to open JAR file")
        }
    }

    fn parse_manifest(jar_archive: &mut ZipArchive<BufReader<fs::File>>) -> Result<HashMap<String, String>, &'static str>{
        let mut manifest_content = String::new();

        let manifest_file = jar_archive.by_name("META-INF/MANIFEST.MF");
        if manifest_file.is_err() { return Err("Failed to find MANIFEST.MF"); }

        if manifest_file.unwrap().read_to_string(&mut manifest_content).is_err() {
            return Err("Failed to read MANIFEST.MF");
        }

        let mut result = HashMap::new();
        {
            for line in manifest_content.lines() {
                if line.is_empty() {
                    continue;
                }

                let key_value = line.split_once(':');
                if let Some((key, value)) = key_value {
                    result.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        }

        Ok(result)
    }

    pub fn get_main_class(&self) -> Option<&class::Class> {
        let mut main_class_name = self.manifest.get("Main-Class")?.clone();
        main_class_name.push_str(".class");

        self.classes.get(&main_class_name)
    }

}