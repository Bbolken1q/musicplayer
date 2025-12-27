pub struct Error {
    pub code: u8
}

impl Error {
    pub fn read(self) -> String {
        match self.code {
            1 => "File not found".to_string(),
            2 => "Unsupported format".to_string(),
            _ => "Unknown error".to_string(),
        }
    }
}