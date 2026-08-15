#![allow(dead_code)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub city: String,
}

pub fn parse_csv(text: &str) -> Vec<Person> {
    let mut people = Vec::new();
    let mut lines = text.lines();
    if let Some(_header) = lines.next() {
        for line in lines {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3
                && let (Ok(age), name, city) = (parts[1].parse::<u32>(), parts[0], parts[2])
            {
                people.push(Person {
                    name: name.to_string(),
                    age,
                    city: city.to_string(),
                });
            }
        }
    }
    people
}
