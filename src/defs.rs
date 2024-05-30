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
                underlying_error: e,
            }),
        }
    
    }
}

pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

impl From<u64> for SeekFrom {
    fn from(pos: u64) -> Self {
        SeekFrom::Start(pos)
    }
}

impl From<SeekFrom> for std::io::SeekFrom {
    fn from(pos: SeekFrom) -> Self {
        match pos {
            SeekFrom::Start(pos) => std::io::SeekFrom::Start(pos),
            SeekFrom::End(pos) => std::io::SeekFrom::End(pos),
            SeekFrom::Current(pos) => std::io::SeekFrom::Current(pos),
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
    underlying_error: std::io::Error,
}

impl From<std::io::Error> for FileError {
    fn from(e: std::io::Error) -> Self {
        FileError {
            message: e.to_string(),
            underlying_error: e,
        }
    }
}

pub trait Writer {
    fn fwrite(&mut self, buf: String) -> Result<usize, FileError>;
    fn fwrite_u8(&mut self, buf: &[u8]) -> Result<usize, FileError>;
    fn fflush(&mut self) -> Result<(), FileError>;
}

pub trait Reader {
    fn fread(&mut self) -> Result<String, FileError>;
    fn fread_u8(&mut self) -> Result<Vec<u8>, FileError>;
}

pub trait Seeker {
    fn fseek(&mut self, pos: SeekFrom) -> Result<u64, FileError>;
}

// first, implement our traits for anything implementing std::io::Write
impl<T> Writer for T
where
    T: Write,
{
    fn fwrite(&mut self, buf: String) -> Result<usize, FileError> {
        self.write(buf.as_bytes()).map_err(FileError::from)
    }

    fn fwrite_u8(&mut self, buf: &[u8]) -> Result<usize, FileError> {
        self.write(buf).map_err(FileError::from)
    }

    fn fflush(&mut self) -> Result<(), FileError> {
        Write::flush(self).map_err(FileError::from)
    }
}

// next, implement our traits for anything implementing std::io::Read
impl<T> Reader for T
where
    T: Read,
{
    fn fread(&mut self) -> Result<String, FileError> {
        let mut buf = String::new();
        self.read_to_string(&mut buf).map_err(FileError::from)?;
        Ok(buf)
    }

    fn fread_u8(&mut self) -> Result<Vec<u8>, FileError> {
        let mut buf = Vec::new();
        self.read_to_end(&mut buf).map_err(FileError::from)?;
        Ok(buf)
    }
}

// finally, implement our traits for anything implementing std::io::Seek
impl<T> Seeker for T
where
    T: Seek,
{
    fn fseek(&mut self, pos: SeekFrom) -> Result<u64, FileError> {
        Seek::seek(self, pos.into()).map_err(FileError::from)
    }
}

// now, implement our traits for our File struct
impl Writer for File {
    fn fwrite(&mut self, buf: String) -> Result<usize, FileError> {
        self.underlying_file.write(buf.as_bytes()).map_err(FileError::from)
    }

    fn fwrite_u8(&mut self, buf: &[u8]) -> Result<usize, FileError> {
        self.underlying_file.write(buf).map_err(FileError::from)
    }

    fn fflush(&mut self) -> Result<(), FileError> {
        self.underlying_file.flush().map_err(FileError::from)
    }
}

impl Reader for File {
    fn fread(&mut self) -> Result<String, FileError> {
        let mut buf = String::new();
        self.underlying_file.read_to_string(&mut buf).map_err(FileError::from)?;
        Ok(buf)
    }

    fn fread_u8(&mut self) -> Result<Vec<u8>, FileError> {
        let mut buf = Vec::new();
        self.underlying_file.read_to_end(&mut buf).map_err(FileError::from)?;
        Ok(buf)
    }
}

impl Seeker for File {
    fn fseek(&mut self, pos: SeekFrom) -> Result<u64, FileError> {
        self.underlying_file.seek(pos.into()).map_err(FileError::from)
    }
}