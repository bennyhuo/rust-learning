use std::collections::HashMap;

pub fn main() {
    let mut company = Company::new();
    company.add_employee("Benny", "Kotlin UG");
    company.add_employee("Benny", "Kotlin UG");
    company.add_employee("Benny", "Kotlin UG");
    company.add_employee("Benny", "Kotlin UG");

    company
        .list_employees("Kotlin UG")
        .map(|employees| employees.iter().for_each(|e| println!("{e}")));
}

struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }

    fn add_employee(&mut self, name: &str, department: &str) {
        self.departments
            .entry(department.to_string())
            .or_insert(Vec::new())
            .push(name.to_string())
    }

    fn list_employees(&self, department: &str) -> Option<&Vec<String>> {
        self.departments.get(department)
    }
}
