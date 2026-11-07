//! Action model.

#[derive(Debug, Clone)]
pub enum PrintAction {
    PrintAll,
    PrintFields(Vec<usize>),
}

#[derive(Debug, Clone)]
pub enum AwkAction {
    Print(PrintAction),
    Assignment(String, String),
    IfThen(Box<AwkAction>, Box<AwkAction>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_action() {
        let a = AwkAction::Print(PrintAction::PrintAll);
        assert!(matches!(a, AwkAction::Print(_)));
    }
}
