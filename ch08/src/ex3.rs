use std::collections::HashMap;

pub fn main() {
    let mut company = Company::new();
    company.add_employee("E", "Kotlin");
    company.add_employee("B", "Kotlin");
    company.add_employee("C", "Kotlin");
    company.add_employee("A", "Kotlin1");
    company.add_employee("D", "Kotlin1");

    company.list_employees("Kotlin").iter().for_each(|e| {
        println!("{e}");
    });

    println!("----");
    company.list_all_employees().iter().for_each(|e| {
        println!("{e}");
    })
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

    fn list_employees(&self, department: &str) -> Vec<String> {
        self.departments
            .get(department)
            .map(|v| {
                let mut result = v.to_vec();
                result.sort();
                result
            })
            .unwrap_or(Vec::new())
    }

    fn list_all_employees(&self) -> Vec<String> {
        let mut result = self.departments.iter().fold(Vec::new(), |mut acc, e| {
            acc.append(&mut e.1.to_vec());
            acc
        });

        result.sort();
        result
    }
}
