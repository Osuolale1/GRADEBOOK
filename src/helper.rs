use crate::traits::*;
use crate::types::*;
use std::io::stdin;

//////////////////// HELPER FUNCTION //////////
pub fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Invalid Input");

    let user_input = input.trim();

    user_input.to_string()
}

pub fn get_subject() -> Option<Subject> {
    let subj = get_input().to_lowercase();

    match subj.as_str() {
        "maths" => Some(Subject::Maths),
        "eng" => Some(Subject::Eng),
        "chm" => Some(Subject::Chm),
        "phy" => Some(Subject::Phy),
        "bio" => Some(Subject::Bio),
        _ => {
            println!("Invalid subject");
            None
        }
    }
}

pub fn print_student_stat(main: &impl DisplayStats) {
    let new = main.display();
    println!("{}", new)
}

pub fn print_class_average(main: &impl ClassAverageGrade) {
    let new = main.average();
    println!("The OVERALL class average IS: {}", new)
}

pub fn print_class_average_per_subject(main: &impl ClassAverageGradePerSubject, sub: Subject) {
    let new = main.average(&sub);
    println!("The class average for {:?} is: {}", sub, new)
}

pub fn print_class_min_per_subject(main: &impl ClassAverageGradePerSubject, sub: &Subject) {
    let new: f64 = main.minimum(&sub);
    println!("The class minimum for {:?} is: {}", sub, new)
}

pub fn print_class_max_per_subject(main: &impl ClassAverageGradePerSubject, sub: &Subject) {
    let new: f64 = main.maximum(&sub);
    println!("The class maximum for {:?} is: {}", sub, new)
}
