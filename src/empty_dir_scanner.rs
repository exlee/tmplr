use std::{
    ffi::OsStr,
    fs::{self, ReadDir},
    io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct EmptyDirScanner {
    stack: Vec<PathBuf>,
    current_dir: Option<ReadDir>,
    current_empty: bool,
    current_path: Option<PathBuf>,
    search_root: PathBuf,
}

impl EmptyDirScanner {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
      let pathbuf = root.as_ref().to_path_buf();
        Self {
            stack: vec![pathbuf.clone()],
            current_dir: None,
            current_empty: true,
            current_path: None,
            search_root: pathbuf.canonicalize().unwrap(),
            
        }
    }
}

impl Iterator for EmptyDirScanner {
    type Item = io::Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Active Iterator
            if let Some(ref mut entries) = self.current_dir {
                match entries.next()  {
                    Some(Ok(entry)) => {
                        let path = entry.path();
                        self.current_empty = false;
                        if path.is_dir() {
                            self.stack.push(path)
                        }
                        continue;
                    }
                    Some(Err(e)) => return Some(Err(e)),
                    None => {
                      if self.current_empty {
                        let dir = self.current_path.clone().expect("Can't unpack path");
                        self.current_dir = None;


                        return Some(Ok(dir));
                        self.current_dir = None;
                      }
                    }
                }
            }

            // New iterator
            match self.stack.pop() {
                Some(dir_path) => match fs::read_dir(dir_path.clone()) {
                    Ok(read_dir) => {
                      self.current_dir = Some(read_dir);
                      self.current_path = Some(dir_path);
                      self.current_empty = true
                    },
                    Err(e) => return Some(Err(e)),
                },
                None => return None,
            }
        }
    }
}
