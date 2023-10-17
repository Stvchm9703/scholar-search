use sled;

pub struct StateMach {
    db: sled::Db,
}

impl StateMach {
    pub fn new() -> Self {
        let db = sled::open("data/state_mach").unwrap();
        Self { db }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.db
            .get(key)
            .unwrap()
            .map(|x| String::from_utf8(x.to_vec()).unwrap())
    }

    pub fn set(&self, key: &str, value: &str) {
        if let Some(_) = self.get(key) {
          
            self.db.insert(key, value).unwrap();
            return;
        }
        self.db.insert(key, value).unwrap();
    }

    pub fn delete(&self, key: &str) {
        self.db.remove(key).unwrap();
    }

    pub fn get_all(&self) -> Vec<(String, String)> {
        self.db
            .iter()
            .map(|x| {
                let (k, v) = x.unwrap();
                (
                    String::from_utf8(k.to_vec()).unwrap(),
                    String::from_utf8(v.to_vec()).unwrap(),
                )
            })
            .collect()
    }

    pub fn clear(&self) {
        self.db.clear().unwrap();
    }

    pub fn snapshot(&self) {
        self.db.flush().unwrap();
    }

    pub fn println(&self) {
        println("StateMach");
        println("---------");
        for (k, v) in self.get_all() {
            println!("{}: {}", k, v);
        }
        println();
        println(self.db.stats().unwrap());
        println("---------");
    }
}

impl Default for StateMach {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StateMach {
    fn drop(&mut self) {
        self.db.flush().unwrap();
    }
}

impl Display for StateMach {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StateMach")
    }
}

pub enum PdfFileStatus {
    None(Result::None),
    Accpeted,
    Downloaded,
    Converted,
    Indexed,
    Patched,
}

pub trait PdfFileState {
    pub fn check_file_status(paper_id: &str);
}

impl PdfFileState for StateMach {
    fn check_file_status(self, paper_id: &str) -> PdfFileStatus {
        match self.get(paper_id) {
            Ok(paper_val) => match paper_val {
                "accpeted" => PdfFileStatus::Accpeted,
                "downloaded" => PdfFileStatus::Downloaded,
                "converted" => PdfFileStatus::Converted,
                "indexed" => PdfFileStatus::Indexed,
                "patched" => PdfFileStatus::Patched,
                _ => PdfFileStatus::None,
            },
            None => PaperFileStatus::None,
        }
    }

    fn set_file_status(self, paper_id: &str, status: &str) {
        self.set(paper_id, status);
    }
}


mod test {
    use super::*;
    #[tokio::test]
    async fn test_set_file_status() {
        let state_mach = StateMach::new();
        state_mach.set_file_status("10.1145/3292500.3330648", "accpeted");
        assert_eq!(
            state_mach.check_file_status("10.1145/3292500.3330648"),
            PaperFileStatus::Accpeted
        );
    }
}
