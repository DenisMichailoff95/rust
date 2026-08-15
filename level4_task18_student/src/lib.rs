#![allow(dead_code)]
pub struct Student {
    pub name: String,
    pub grades: Vec<u8>,
}

impl Student {
    pub fn average_grade(&self) -> f64 {
        self.grades.iter().map(|&g| g as f64).sum::<f64>() / self.grades.len() as f64
    }

    pub fn add_grade(&mut self, grade: u8) {
        self.grades.push(grade);
    }
}
