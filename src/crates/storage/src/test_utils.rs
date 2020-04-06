use std::fs;
use std::io::ErrorKind;

/// A handle to hold to ensure you automatically delete files after a test (success/failure).
pub struct TestFileHandle {
    pub file_path: String,
}

impl TestFileHandle {
    pub fn new(file_path: String) -> Self {
        TestFileHandle { file_path }
    }

    pub fn rm(&self, panic_msg: &str) {
        match fs::remove_file(&self.file_path) {
            Ok(_) => {},
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {},
                _ => panic!("fs::remove_file failed - {}: Debug={:?} Display={}", panic_msg, e, e)
            },
        }
    }
}

impl Drop for TestFileHandle {
    fn drop(&mut self) {
        self.rm("drop");
    }
}

pub fn rand_str() -> String {
    format!("{:x}", rand::random::<u64>())
}
