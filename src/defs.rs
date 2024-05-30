use std::io::{Read, Seek, Write};

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct FileOptions: u32 {
        const Read = 0b00000001;
        const Write = 0b00000010;
        const Create = 0b00000100;
        const ExclusiveCreate = 0b00001000;
        const Truncate = 0b00010000;
        const Append = 0b00100000;

        const Uninitialized = 0b10000000;
    }
}

impl FileOptions {
    fn is_initialized(&self) -> bool {
        !self.contains(FileOptions::Uninitialized)
    }

    pub fn read(self, set: bool) -> FileOptions {
        if set {
            (self - FileOptions::Uninitialized) | FileOptions::Read
        } else {
            self
        }
    }

    pub fn write(self, set: bool) -> FileOptions {
        if set {
            (self - FileOptions::Uninitialized) | FileOptions::Write
        } else {
            self
        }
    }

    pub fn create(self, set: bool) -> FileOptions {
        if set {
            (self - FileOptions::Uninitialized) | FileOptions::Create
        } else {
            self
        }
    }

    pub fn exclusive_create(self, set: bool) -> FileOptions {
        if set {
            (self - FileOptions::Uninitialized) | FileOptions::ExclusiveCreate
        } else {
            self
        }
    }

    pub fn truncate(self, set: bool) -> FileOptions {
        if set {
            (self - FileOptions::Uninitialized) | FileOptions::Truncate
        } else {
            self
        }
    }

    pub fn append(self, set: bool) -> FileOptions {
        if set {
            (self - FileOptions::Uninitialized) | FileOptions::Append
        } else {
            self
        }
    }

    pub fn new() -> FileOptions {
        FileOptions::Uninitialized
    }

    pub fn open<T>(self, file_name: T) -> Result<File, FileError>
    where
        T: ToString,
    {
        if self.contains(FileOptions::Uninitialized) {
            return Err(FileError {
                message: "FileOptions uninitialized".to_string(),
                file_options: self,
                file_name: file_name.to_string(),
                underlying_error: std::io::Error::new(std::io::ErrorKind::InvalidInput, "FileOptions uninitialized"),
            });
        }

        let openoptions = std::fs::OpenOptions::new()
            .read(self.contains(FileOptions::Read))
            .write(self.contains(FileOptions::Write))
            .create(self.contains(FileOptions::Create))
            .create_new(self.contains(FileOptions::ExclusiveCreate))
            .truncate(self.contains(FileOptions::Truncate))
            .append(self.contains(FileOptions::Append))
            .open(file_name.to_string());

        match openoptions {
            Ok(file) => Ok(File {
                file_name: file_name.to_string(),
                file_options: self,
                underlying_file: file,
            }),
            Err(e) => Err(FileError {
                message: e.to_string(),
                file_options: self,
                file_name: file_name.to_string(),
                underlying_error: e,
            }),
        }
    
    }
}

pub struct File {
    file_name: String,
    file_options: FileOptions,
    pub underlying_file: std::fs::File,
}

// error struct
#[derive(Debug)]
pub struct FileError {
    message: String,
    file_options: FileOptions,
    file_name: String,
    underlying_error: std::io::Error,
}

impl File {
    pub fn read(&mut self) -> Result<String, FileError> {
        let mut buffer = String::new();
        match self.underlying_file.read_to_string(&mut buffer) {
            Ok(_) => {
                println!("Read {} bytes from file", buffer.len());
                Ok(buffer)
            },
            Err(e) => Err(FileError {
                message: e.to_string(),
                file_options: self.file_options,
                file_name: self.file_name.clone(),
                underlying_error: e,
            }),
        }
    }

    pub fn read_u8(&mut self) -> Result<Vec<u8>, FileError> {
        let string = self.read();

        match string {
            Ok(s) => Ok(s.into_bytes()),
            Err(e) => Err(e),
        }
    }

    pub fn write<T: ToString>(&mut self, data: T) -> Result<(), FileError> {
        match self.underlying_file.write_all(data.to_string().as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(FileError {
                message: e.to_string(),
                file_options: self.file_options,
                file_name: self.file_name.clone(),
                underlying_error: e,
            }),
        }
    }

    pub fn write_u8(&mut self, data: Vec<u8>) -> Result<(), FileError> {
        let string = String::from_utf8(data);

        match string {
            Ok(s) => self.write(s),
            Err(e) => Err(FileError {
                message: e.to_string(),
                file_options: self.file_options,
                file_name: self.file_name.clone(),
                underlying_error: std::io::Error::new(std::io::ErrorKind::InvalidData, e),
            }),
        }
    }

    pub fn flush(&mut self) -> Result<(), FileError> {
        match self.underlying_file.flush() {
            Ok(_) => Ok(()),
            Err(e) => Err(FileError {
                message: e.to_string(),
                file_options: self.file_options,
                file_name: self.file_name.clone(),
                underlying_error: e,
            }),
        }
    }

    pub fn delete(self) -> Result<(), FileError> {
        match std::fs::remove_file(self.file_name.clone()) {
            Ok(_) => Ok(()),
            Err(e) => Err(FileError {
                message: e.to_string(),
                file_options: self.file_options,
                file_name: self.file_name.clone(),
                underlying_error: e,
            }),
        }
    }

    pub fn close(self) -> Result<(), FileError> {
        match self.underlying_file.sync_all() {
            Ok(_) => Ok(()),
            Err(e) => Err(FileError {
                message: e.to_string(),
                file_options: self.file_options,
                file_name: self.file_name.clone(),
                underlying_error: e,
            }),
        }
    }

    pub fn seek(&mut self, pos: u32) -> Result<u64, FileError> {
        match self.underlying_file.seek(std::io::SeekFrom::Start(pos as u64)) {
            Ok(pos) => Ok(pos),
            Err(e) => Err(FileError {
                message: e.to_string(),
                file_options: self.file_options,
                file_name: self.file_name.clone(),
                underlying_error: e,
            }),
        }
    }
}