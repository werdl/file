mod defs;


mod tests {
    use crate::defs::{File, FileOptions};
    use std::io::Read;


    #[test]
    fn open() {
        let file = (FileOptions::Create | FileOptions::Write).open("file.txt");
        assert_eq!(file.is_ok(), true);
    }
    
    #[test]
    fn open_and_attempt_read() {
        let file = (FileOptions::Read).open("README.md");
        assert_eq!(file.is_ok(), true);
        let mut file: File = file.unwrap();
        
        // read the file
        let contents = file.read();
    }
}