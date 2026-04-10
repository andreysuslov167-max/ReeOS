use spin::Mutex;
use lazy_static::lazy_static;

const MAX_FILES: usize = 16;
const MAX_FILENAME: usize = 32;
const MAX_FILE_SIZE: usize = 512;

pub struct RamFile {
    name: [u8; MAX_FILENAME],
    name_len: usize,
    data: [u8; MAX_FILE_SIZE],
    size: usize,
}

pub struct RamDisk {
    files: [RamFile; MAX_FILES],
    count: usize,
}

impl RamDisk {
    pub fn new() -> Self {
        let mut disk = RamDisk {
            files: [
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
                RamFile { name: [0; MAX_FILENAME], name_len: 0, data: [0; MAX_FILE_SIZE], size: 0 },
            ],
            count: 0,
        };
        
       
        disk.create(b"/readme.txt", b"Welcome to ReeOS RAM Disk!\nFiles are lost on reboot.");
        disk.create(b"/hello.txt", b"Hello World!");
        
        disk
    }
    
    
    fn str_to_array(s: &[u8]) -> [u8; MAX_FILENAME] {
        let mut arr = [0; MAX_FILENAME];
        let len = if s.len() > MAX_FILENAME { MAX_FILENAME } else { s.len() };
        for i in 0..len {
            arr[i] = s[i];
        }
        arr
    }
    
    pub fn create(&mut self, name: &[u8], data: &[u8]) -> bool {
      
        for i in 0..self.count {
            if self.files[i].name_len == name.len() {
                let mut same = true;
                for j in 0..name.len() {
                    if self.files[i].name[j] != name[j] {
                        same = false;
                        break;
                    }
                }
                if same {
                   
                    let data_len = if data.len() > MAX_FILE_SIZE { MAX_FILE_SIZE } else { data.len() };
                    for j in 0..data_len {
                        self.files[i].data[j] = data[j];
                    }
                    self.files[i].size = data_len;
                    return true;
                }
            }
        }
        
        
        if self.count < MAX_FILES {
            let name_len = if name.len() > MAX_FILENAME { MAX_FILENAME } else { name.len() };
            self.files[self.count].name = Self::str_to_array(name);
            self.files[self.count].name_len = name_len;
            
            let data_len = if data.len() > MAX_FILE_SIZE { MAX_FILE_SIZE } else { data.len() };
            for j in 0..data_len {
                self.files[self.count].data[j] = data[j];
            }
            self.files[self.count].size = data_len;
            
            self.count += 1;
            true
        } else {
            false
        }
    }
    
    pub fn read(&self, name: &[u8]) -> Option<&[u8]> {
        for i in 0..self.count {
            if self.files[i].name_len == name.len() {
                let mut same = true;
                for j in 0..name.len() {
                    if self.files[i].name[j] != name[j] {
                        same = false;
                        break;
                    }
                }
                if same {
                    return Some(&self.files[i].data[..self.files[i].size]);
                }
            }
        }
        None
    }
    
    pub fn delete(&mut self, name: &[u8]) -> bool {
        for i in 0..self.count {
            if self.files[i].name_len == name.len() {
                let mut same = true;
                for j in 0..name.len() {
                    if self.files[i].name[j] != name[j] {
                        same = false;
                        break;
                    }
                }
                if same {
                 
                    for j in i..self.count-1 {
                        self.files[j] = RamFile {
                            name: self.files[j+1].name,
                            name_len: self.files[j+1].name_len,
                            data: self.files[j+1].data,
                            size: self.files[j+1].size,
                        };
                    }
                    self.count -= 1;
                    return true;
                }
            }
        }
        false
    }
    
    pub fn list(&self, buffer: &mut [u8]) -> usize {
        let mut pos = 0;
        for i in 0..self.count {
            if pos + self.files[i].name_len + 2 < buffer.len() {
                for j in 0..self.files[i].name_len {
                    buffer[pos] = self.files[i].name[j];
                    pos += 1;
                }
                buffer[pos] = b'\n';
                pos += 1;
            }
        }
        pos
    }
}

lazy_static! {
    pub static ref DISK: Mutex<RamDisk> = Mutex::new(RamDisk::new());
}
pub fn count_files() -> usize {
    DISK.lock().count
}

pub fn create_file(name: &[u8], data: &[u8]) -> bool {
    DISK.lock().create(name, data)
}

pub fn read_file(name: &[u8]) -> Option<[u8; MAX_FILE_SIZE]> {
    DISK.lock().read(name).map(|data| {
        let mut arr = [0; MAX_FILE_SIZE];
        let len = data.len();
        for i in 0..len {
            arr[i] = data[i];
        }
        arr
    })
}

pub fn read_file_len(name: &[u8]) -> Option<usize> {
    DISK.lock().read(name).map(|data| data.len())
}

pub fn delete_file(name: &[u8]) -> bool {
    DISK.lock().delete(name)
}

pub fn list_files(buffer: &mut [u8]) -> usize {
    DISK.lock().list(buffer)
}