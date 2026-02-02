// Project 6: Gradebook with Trait Extensions
//
// Summary:
//   Create a command line gradebook application that manages student
//   records and calculates statistics. This project focuses on trait
//   implementations and working with borrowed data through lifetimes.
//
// User stories:
// * Stage 1:
//   - I want to add students with their names and grades. YES
//   - I want to view all students and their grades. YES
// * Stage 2:
//   - I want to calculate average grades for the class. YES
//   - I want to see statistics (min, max, average) using traits. YES
// * Stage 3:
//   - I want to generate reports for individual students. 🫠
//   - I want to filter students by grade ranges.🙂‍↔️

// Tips:
// * Create a Student struct with name and grades fields. YES
// * Implement a DisplayStats trait for reporting functionality. YES
// * Use lifetimes when borrowing student data for reports.    😒
// * Consider using vectors to store multiple grades per student. YES

use std::{collections::HashMap, io::stdin};
use task6::helper::*;
use task6::types::*;

fn main() {
    let mut new_class = Class::new();
    loop {
        println!("\n=== GRADEBOOK ===");
        println!("1. ADD STUDENT DETAILS");
        println!("2. VIEW STUDENTS GRADES ");
        println!("3. OVERALL CLASS AVERAGE ");
        println!("4. CLASS AVERAGE PER SUBJECT ");
        println!("5. CLASS MINIMUM PER SUBJECT ");
        println!("6. CLASS MAXIMUM PER SUBJECT ");
        println!("0. EXIT");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Invalid input");

        let input: u8 = match input.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                println!("Input a number");
                continue;
            }
        };

        match input {
            1 => {
                println!("Enter Student Names");
                let name = get_input();

                let mut student_details = HashMap::new();

                loop {
                    println!("Enter Any Subject Of Your Choice");
                    println!("\n 'Maths', 'Eng', 'Chm', 'Phy', 'Bio', 'Done(if you're done)' ");
                    let subj = get_input().to_lowercase();
                    let subject = match subj.as_str() {
                        "maths" => Subject::Maths,
                        "eng" => Subject::Eng,
                        "chm" => Subject::Chm,
                        "phy" => Subject::Phy,
                        "bio" => Subject::Bio,
                        "done" => break,
                        _ => {
                            println!("Invalid subject");
                            continue;
                        }
                    };

                    println!("Enter Student Grade");
                    let grade: f64 = match get_input().trim().parse() {
                        Ok(grade) => grade,
                        Err(_) => {
                            println!("Not a valid input");
                            continue;
                        }
                    };

                    student_details.insert(subject, grade);
                }
                let student1 = Student {
                    name,
                    grades: student_details,
                };

                new_class.add_student(student1);
            }

            2 => {
                for i in &new_class.class {
                    print_student_stat(i);
                }
            }
            3 => {
                print_class_average(&new_class);
            }
            4 => {
                println!("\n ENTER THE SUBJECT YOU WANT IT AVERAGE ");
                if let Some(subject) = get_subject() {
                    print_class_average_per_subject(&new_class, subject);
                }
            }
            5 => {
                println!("\n ENTER THE SUBJECT YOU WANT IT MINIMUN SCORE ");
                if let Some(subject) = get_subject() {
                    print_class_min_per_subject(&new_class, &subject);
                }
            }
            6 => {
                println!("\n ENTER THE SUBJECT YOU WANT IT MAXIMUM SCORE ");
                if let Some(subject) = get_subject() {
                    print_class_max_per_subject(&new_class, &subject);
                }
            }
            0 => {
                println!("EXITING...");
                break;
            }
            _ => {
                println!("\n SELECT AN OPTION BETWEEN 0 - 6 ")
            }
        }
    }
}
