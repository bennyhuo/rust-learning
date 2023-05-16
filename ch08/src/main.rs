use std::collections::HashMap;

fn main() {
  let mut company = Company::new();
  company.add_employee1("E", "Kotlin");
  company.add_employee1("B", "Kotlin");
  company.add_employee1("C", "Kotlin");
  company.add_employee1("A", "Kotlin1");
  company.add_employee1("D", "Kotlin1");

  let r = company.list_employees1("Kotlin");

  println!("----");
  let r2 = company.list_all_employees();

  drop(company);

  r2.iter().for_each(|e| {
    println!("{e}");
  });

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

  fn add_employee1(&mut self, name: &str, department: &str) {
    match self.departments.get_mut(department) {
      Some(value) => value.push(name.to_string()),
      None => {
        self.departments.insert(department.to_string(), vec!{name.to_string()});
      }
    }
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

  fn list_employees1(&self, department: &str) -> Vec<&str> {
    let mut result = self.departments.get(department)
      .into_iter()
      .flatten()
      .map(|s| s.as_str())
      .collect::<Vec<_>>();

    result.sort();
    return result;
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
