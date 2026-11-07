//! Pattern model.

#[derive(Debug, Clone)]
pub enum MatchCondition {
    Begin,
    End,
    Regex(String),
    Expression(String),
}

#[derive(Debug, Clone)]
pub struct AwkPattern {
    pub condition: MatchCondition,
}

impl AwkPattern {
    pub fn new(condition: MatchCondition) -> Self {
        Self { condition }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn awk_pattern() {
        let p = AwkPattern::new(MatchCondition::Begin);
        assert!(matches!(p.condition, MatchCondition::Begin));
    }
}
