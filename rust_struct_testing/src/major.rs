#[derive(Debug)]
pub enum Major {
    
    CS,
    EE,
    Undefined,
}

impl Major {
    pub fn classify(major: &str) -> Self {
        match major {
            "CS" => Major::CS,
            "EE" => Major::EE,
            _ => Major::Undefined,
        }
    }
}
