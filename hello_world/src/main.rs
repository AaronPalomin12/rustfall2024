#[derive(Debug)]
enum GradeLevel 
{
    Bachelor,
    Master,
    PhD,
}

#[derive(Debug)]
enum Major 
{
    ComputerScience,
    ElectricalEngineering,
}

#[derive(Debug)]
struct Student 
{
    name: String,
    grade: GradeLevel,
    major: Major,
}

impl Student {
    fn new(name: String, grade: GradeLevel, major: Major) -> Self 
    {
        Student 
        {
            name,
            grade,
            major,
        }
    }

    fn introduce_yourself(&self) 
    {
        println!("Hi, my name is {}.", self.name);

        let grade_msg = match self.grade {
            GradeLevel::Bachelor => "I am pursuing a Bachelor's degree.",
            GradeLevel::Master => "I am pursuing a Master's degree.",
            GradeLevel::PhD => "I am pursuing a PhD.",
        };
        println!("{}", grade_msg);

        let major_msg = match self.major 
        {
            Major::ComputerScience => "My major is Computer Science.",
            Major::ElectricalEngineering => "My major is Electrical Engineering.",
        };

        println!("{}", major_msg);
    }
}

fn main() {
    let s1 = Student::new(
        "Aaron Palomin".to_string(),
        GradeLevel::Bachelor,
        Major::ComputerScience,
    );
        // I wanted to do more for the other cases just for practice
    let s2 = Student::new(
        "Aaron Palomin".to_string(),
        GradeLevel::Master,
        Major::ElectricalEngineering,
    );

    let s3 = Student::new(
        "Aaron Palomin".to_string(),
        GradeLevel::PhD,
        Major::ElectricalEngineering,

    );
    
    s1.introduce_yourself();
    println!();
    s2.introduce_yourself();
    println!();
    s3.introduce_yourself();

}
