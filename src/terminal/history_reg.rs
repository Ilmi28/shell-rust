

#[derive(Default)]
pub struct HistoryRegistry {
    history: Vec<HistoryRecord>
}

#[derive(Clone)]
pub struct HistoryRecord {
    pub command: String,
    pub number: u32
}

impl HistoryRegistry {
    pub fn add(&mut self, record: &str) {
        let next = self.get_next_num();
        let record = HistoryRecord {
            command: record.to_string(),
            number: next
        };
        self.history.push(record)
    }

    pub fn get_all(&self) -> Vec<HistoryRecord> {
        self.history.clone()
    }

    fn get_next_num(&self) -> u32 {
        if let Some(max) = self.history.iter().map(|x| x.number).max() {
            max + 1
        } else {
            1
        }
    }
}