/// A simple data structure
/// for mathematical signature 
pub(crate) enum Sign {
    Positive,
    Negative
}

impl Sign {
    pub(crate) fn to_string(&self) -> String {
        match self {
            Sign::Positive => "+".to_owned(),
            Sign::Negative => "-".to_owned(),
        }
    }
}