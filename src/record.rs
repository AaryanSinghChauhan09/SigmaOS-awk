//! Record model.

#[derive(Debug, Clone)]
pub struct Field {
    pub index: usize,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub line: String,
    pub fields: Vec<Field>,
    pub record_number: usize,
}

impl Record {
    pub fn new(line: String, record_number: usize) -> Self {
        let fields: Vec<Field> = line.split_whitespace()
            .enumerate()
            .map(|(i, v)| Field { index: i + 1, value: v.to_string() })
            .collect();
        Self { line, fields, record_number }
    }

    pub fn field(&self, index: usize) -> Option<&Field> {
        self.fields.iter().find(|f| f.index == index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record() {
        let r = Record::new("a b c".into(), 1);
        assert_eq!(r.fields.len(), 3);
    }
}
