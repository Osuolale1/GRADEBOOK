use crate::types::{Class, Student, Subject};

pub trait DisplayStats {
    fn display(&self) -> String;
    fn min(&self) -> f64;
    fn max(&self) -> f64;
}

pub trait ClassAverageGrade {
    fn average(&self) -> f64;
}
pub trait ClassAverageGradePerSubject {
    fn average(&self, subject: &Subject) -> f64;
    fn minimum(&self, subject: &Subject) -> f64;
    fn maximum(&self, subject: &Subject) -> f64;
}

impl DisplayStats for Student {
    fn display(&self) -> String {
        format!("The grades of {:?} are {:?}", self.name, self.grades)
    }

    fn min(&self) -> f64 {
        self.grades
            .values()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }

    fn max(&self) -> f64 {
        self.grades
            .values()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }
}

impl ClassAverageGrade for Class {
    fn average(&self) -> f64 {
        if self.class.is_empty() {
            println!("No Class found!");
            return 0.0;
        }

        let mut total_grade_sum: f64 = 0.0;
        let mut total_grade_count = 0;

        for student in &self.class {
            for grade in student.grades.values() {
                total_grade_sum += grade;
                total_grade_count += 1;
            }
        }

        if total_grade_count == 0 {
            0.0
        } else {
            total_grade_sum / total_grade_count as f64
        }
    }
}

impl ClassAverageGradePerSubject for Class {
    fn average(&self, sub: &Subject) -> f64 {
        if self.class.is_empty() {
            println!("No Class found!");
            return 0.0;
        }

        let mut total_grade_sum: f64 = 0.0;
        let mut total_grade_count = 0;

        for student in &self.class {
            if let Some(grade) = student.grades.get(&sub) {
                total_grade_sum += grade;
                total_grade_count += 1;
            }
        }

        if total_grade_count == 0 {
            0.0
        } else {
            total_grade_sum / total_grade_count as f64
        }
    }

    fn minimum(&self, sub: &Subject) -> f64 {
        if self.class.is_empty() {
            return 0.0;
        }

        self.class
            .iter()
            .filter_map(|student| student.grades.get(sub))
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }

    fn maximum(&self, sub: &Subject) -> f64 {
        if self.class.is_empty() {
            return 0.0;
        }

        self.class
            .iter()
            .filter_map(|student| student.grades.get(sub))
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }
}
