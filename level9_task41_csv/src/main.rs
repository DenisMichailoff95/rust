struct Person {
    name: String,
    age: u32,
    city: String,
}

fn parse_csv(text: &str) -> Vec<Person> {
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

fn main() {
    let csv = "name,age,city\nAlice,30,Paris\nBob,25,Lyon";
    let people = parse_csv(csv);
    for p in people {
        println!("{} {} {}", p.name, p.age, p.city);
    }
}
