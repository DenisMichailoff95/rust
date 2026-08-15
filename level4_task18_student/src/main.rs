struct Student {
    name: String,
    grades: Vec<u8>,
}

impl Student {
    fn average_grade(&self) -> f64 {
        self.grades.iter().map(|&g| g as f64).sum::<f64>() / self.grades.len() as f64
    }

    fn add_grade(&mut self, grade: u8) {
        self.grades.push(grade);
    }
}

fn main() {
    let mut s = Student {
        name: String::from("Alice"),
        grades: vec![5, 4, 3],
    };
    s.add_grade(5);
    println!("{} average: {}", s.name, s.average_grade());
}
