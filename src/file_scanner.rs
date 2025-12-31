use std::{
    fs::{self, ReadDir},
    io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct FileScanner {
    stack: Vec<PathBuf>,
    current_dir: Option<ReadDir>,
    extension: Option<String>,
}

impl FileScanner {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self {
            stack: vec![root.as_ref().to_path_buf()],
            current_dir: None,
            extension: None,
        }
    }
    pub fn new_with_extension<P: AsRef<Path>>(root: P, extension: String) -> Self {
        Self {
            stack: vec![root.as_ref().to_path_buf()],
            current_dir: None,
            extension: Some(extension),
        }
    }
}

impl Iterator for FileScanner {
    type Item = io::Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        #[derive(Debug)]
        enum ExtCheck {
            NoCheck,
            Match,
            NoMatch,
        }
        fn check_ext(search_ext_opt: &Option<String>, path: &Path) -> ExtCheck {
            let Some(search_ext) = search_ext_opt else {
                return ExtCheck::NoCheck;
            };
            // Should it handle empty extension case (e.g. files that end with dot)?
            let Some(path_ext) = path.extension().and_then(|p| p.to_str()) else {
                return ExtCheck::NoMatch;
            };

            if search_ext.as_str() == path_ext {
                ExtCheck::Match
            } else {
                ExtCheck::NoMatch
            }
        }

        loop {
            // Active Iterator
            if let Some(ref mut entries) = self.current_dir {
                match entries.next() {
                    Some(Ok(entry)) => {
                        let path = entry.path();
                        if path.is_dir() {
                            self.stack.push(path)
                        } else {
                            match check_ext(&self.extension, &path) {
                                ExtCheck::NoCheck => return Some(Ok(path)),
                                ExtCheck::Match => return Some(Ok(path)),
                                ExtCheck::NoMatch => (),
                            }
                        }
                        continue;
                    }
                    Some(Err(e)) => return Some(Err(e)),
                    None => self.current_dir = None,
                }
            }

            // New iterator
            match self.stack.pop() {
                Some(dir_path) => match fs::read_dir(dir_path) {
                    Ok(read_dir) => self.current_dir = Some(read_dir),
                    Err(e) => return Some(Err(e)),
                },
                None => return None,
            }
        }
    }
}
