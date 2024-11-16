mod models;

use std::error::Error;
use std::{fs, process};
use models::student::Student;

pub fn run() {
    let mut students: Vec<Student> = Vec::new();
    let strings = fetch_students_from_file().unwrap_or_else(|error| {
        println!("{}", error);
        process::exit(1);
    });
    for args in strings {
        match Student::build(&args) {
            Ok(student) => {
                students.push(student);
            }
            Err(msg) => {
                println!("{}", msg);
                process::exit(1);
            }
        };
    }

    print_students_in_performing_order(&students);
}

fn fetch_students_from_file() -> Result<Vec<Vec<String>>, Box< dyn Error>> {
    let file_path = "src/student_grades.txt";
    let contents = fs::read_to_string(file_path)?;

    let mut strings: Vec<Vec<String>> = Vec::new();

    for (line_number, line) in contents.lines().enumerate() {
        let parts: Vec<&str> = line.split(';').collect();

        if parts.len() < 2 {
            return Err(Box::from(format!("Error reading {} on line {}: {}", file_path, line_number, line)));
        }

        let name = parts[0].trim().to_string();
        let score = parts[1].trim().to_string();
        strings.push(vec![name, score])
    }

    Ok(strings)
}

pub fn print_students_in_performing_order(students: &Vec<Student>) {
    println!("**** Students in performing order ****");
    let mut sorted_students: Vec<Student> = students.clone();
    sorted_students.sort_by(|a, b| b.score.cmp(&a.score));

    if sorted_students.is_empty() {
        println!("No students found!");
        return;
    }

    for student in sorted_students {
        println!("{student}");
    }
}