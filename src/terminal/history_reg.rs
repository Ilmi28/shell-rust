

#[derive(Default)]
pub struct HistoryRegistry {
    history: Vec<String>
}

impl HistoryRegistry {
    pub fn add(&mut self, record: &str) {
        self.history.push(record.to_string())
    }

    pub fn get_all(&self) -> Vec<String> {
        self.history.clone()
    }
}