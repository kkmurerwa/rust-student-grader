use std::fmt;
use std::cmp::Ord;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Student {
    pub name: String,
    pub(crate) score: i32,
    pub grade: char,
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Student: {}, Score: {}, Grade: {}", self.name, self.score, self.grade)
    }
}

impl Student {
    pub fn build(args: &[String]) -> Result<Student, &'static str>{
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let name = args[0].clone();
        let score: i32 = args[1].clone().parse().unwrap_or(0);
        let grade: char = Self::get_grade(&score).unwrap_or_else(|msg| {
            println!("{}", msg);
            'F'
        });

        Ok(Student {
            name,
            score,
            grade,
        })
    }

    fn get_grade(score: &i32) -> Result<char, &'static str> {
        match score {
            70..=100 => Ok('A'),
            60..=69 => Ok('B'),
            50..=59 => Ok('C'),
            40..=49 => Ok('D'),
            0..=39 => Ok('F'),
            _ => Err("The score provided is not valid")
        }
    }
}