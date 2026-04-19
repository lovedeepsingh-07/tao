#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    DEV,
    PROD,
}
impl From<&str> for Environment {
    fn from(input: &str) -> Self {
        match input {
            "production" => Environment::PROD,
            _ => Environment::DEV,
        }
    }
}
