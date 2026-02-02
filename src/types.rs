use std::collections::HashMap;

#[derive(Clone)]
pub struct Student {
    pub name: String,
    pub grades: HashMap<Subject, f64>,
}

pub struct Class {
    pub class: Vec<Student>,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Subject {
    Maths,
    Eng,
    Bio,
    Chm,
    Phy,
}

impl Class {
    pub fn new() -> Self {
        Self { class: Vec::new() }
    }

    pub fn add_student(&mut self, _student: Student) {
        self.class.push(_student);
    }
}
