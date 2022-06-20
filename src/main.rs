fn main() {
    println!("Hello, world!");
}
/*-----------------------------*/

use std::collections::HashMap;

pub struct School {
    students: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let entry = self.students.entry(grade).or_insert_with(Vec::new);
        entry.push(student.to_string());
        entry.sort_unstable();
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut s = self.students.keys().cloned().collect::<Vec<u32>>();
        s.sort_unstable();
        s
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.students
            .get(&grade)
            .map(|v| v.to_vec())
            .unwrap_or_default()
    }
}









