#[allow(dead_code)]
pub enum RuleString {
    NotEmpty,
    MinLength(usize),
    MaxLength(usize),
    FixLength(usize),
    UUID,
}

#[allow(dead_code)]
pub enum RuleStringOption {
    MinLength(usize),
    MaxLength(usize),
    FixLength(usize),
}

#[allow(dead_code)]
pub enum RuleNumber {
    NotZero,
}


impl RuleString {
    pub fn validate(&self, field: &str, value: &str) -> Option<String> {       
        match self {
            RuleString::NotEmpty if value.is_empty() => {
                Some(format!("{}: cannot be empty", field))
            }
            RuleString::MinLength(min) if value.len() < *min => {
                Some(format!("{}: must be at least {} characters", field, min))
            }
            RuleString::MaxLength(max) if value.len() > *max => {
                Some(format!("{}: must be at max {} characters", field, max))
            }
            RuleString::FixLength(len) if value.len() > *len => {
                Some(format!("{}: must be {} characters", field, len))
            }
            RuleString::UUID => None, // TODO: not implemented
            _ => None,
        }
    }
}

impl RuleStringOption {
    #[allow(dead_code)]
    pub fn validate(&self, field: &str, value: &Option<String>) -> Option<String> {
        match value {
            None => None, // None always passes
            Some(value) => {
                match self {
                    RuleStringOption::MinLength(min) => RuleString::MinLength(*min).validate(field, value),
                    RuleStringOption::MaxLength(max) => RuleString::MaxLength(*max).validate(field, value),
                    RuleStringOption::FixLength(len) => RuleString::FixLength(*len).validate(field, value)
                }
            }
        }
    }
}

impl RuleNumber {
    fn _validate<T: PartialEq + Default>(&self, field: &str, value: T) -> Option<String> {
        match self {
            RuleNumber::NotZero if value == T::default() => {
                Some(format!("{}: cannot be zero", field))
            }
            _ => None,
        }
    }
}


#[macro_export]
macro_rules! validate {
    ($($field:expr, $value:expr, $rule:expr);* $(;)?) => {{
        let mut errors = Vec::new();
        $(
            if let Some(error) = $rule.validate($field, $value) {
                errors.push(error);
            }
        )*
        // returns the HTTP response 4xx
        if !errors.is_empty() {
            return response_validation_errors(errors);
        }
    }};
}


#[macro_export]
macro_rules! _get_errors {
    ($($field:expr, $value:expr, $rule:expr);* $(;)?) => {{
        let mut errors = Vec::new();
        $(
            if let Some(error) = $rule.validate($field, $value) {
                errors.push(error);
            }
        )*
        // returns the errors
        errors
    }};
}
