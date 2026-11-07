//! Program model.

use super::{AwkPattern, AwkAction};

#[derive(Debug, Clone)]
pub struct Rule {
    pub pattern: Option<AwkPattern>,
    pub action: AwkAction,
}

impl Rule {
    pub fn new(action: AwkAction) -> Self {
        Self { pattern: None, action }
    }

    pub fn with_pattern(mut self, pattern: AwkPattern) -> Self {
        self.pattern = Some(pattern);
        self
    }
}

#[derive(Debug, Clone)]
pub struct AwkProgram {
    pub rules: Vec<Rule>,
}

impl AwkProgram {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn awk_program() {
        let p = AwkProgram::new();
        assert_eq!(p.rules.len(), 0);
    }
}
