use std::collections::HashMap;

pub struct CommonFiles;

impl CommonFiles {
    pub fn get_files() -> HashMap<&'static str, String> {
        let mut map = HashMap::new();
        map.insert("migrations/__init__.py", String::from(""));
        map.insert("__init__.py", String::from(""));
        map.insert("admin.py", String::from("from django.contrib import admin\n\n\n# Register your models here."));
        map.insert("test.py", String::from("from django.test import TestCase\n\n\n# Create your tests here."));
        map
    }
    
}

