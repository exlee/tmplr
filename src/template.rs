use std::{
    collections::HashMap,
    env::{self, current_dir},
    fs::{self},
    path::{Path, PathBuf},
    str::FromStr,
};

use pathdiff::diff_paths;

use crate::file_scanner;

pub const EXTENSION: &str = "tmplr";
pub const OPEN: &str = "{###";
pub const CLOSE: &str = "###}";

#[derive(Clone, Debug)]
pub enum Node {
    Dir(PathBuf),
    File { path: String, content: String },
}
type Template = Vec<Node>;

#[derive(Debug)]
pub struct TemplateRequest {
    pub(crate) path: PathBuf,
    pub(crate) context: HashMap<String, String>,
    pub(crate) dry_run: bool,
}

impl TemplateRequest {
    pub fn make(path: PathBuf, context: HashMap<String, String>, dry_run: bool) -> TemplateRequest {
        TemplateRequest {
            path,
            context,
            dry_run,
        }
    }
}

pub fn read_template(path: &Path) -> Result<Template, String> {
    let mut result: Template = Vec::new();
    let mut cursor = 0;
    let mut current_node: Option<Node> = None;

    fn push_output(s: &str, current_node: &mut Option<Node>) {
        if let Some(Node::File { content, .. }) = current_node {
            content.push_str(s);
        }
    }

    let file_string = fs::read_to_string(path)
        .or_else(|_| fs::read_to_string(get_config_dir().join(path)))
        .or_else(|_| fs::read_to_string(get_config_dir().join(path).with_added_extension("tmplr")))
        .map_err(|_| "Can't read file".to_string())?;

    while let Some(start_offset) = file_string[cursor..].find(OPEN) {
        let tag_start = cursor + start_offset;

        push_output(&file_string[cursor..tag_start], &mut current_node);
        let content_start = tag_start + OPEN.len();
        let remaining = &file_string[content_start..];

        if let Some(end_offset) = remaining.find(CLOSE) {
            let inner = &remaining[..end_offset].trim();
            // process cmds
            let (cmd, params) = match inner.split_once(char::is_whitespace) {
                Some((c, p)) => (c.trim(), p),
                None => (*inner, "".into()),
            };

            match cmd.to_uppercase().as_str() {
                "DIR" => {
                    let file_path = validate_path_string(params)?;
                    //let file_path = file_path.canonicalize().map_err(|err| err.to_string())?;
                    let new_dir = Node::Dir(file_path);
                    result.push(new_dir);
                }
                "FILE" => {
                    if let Some(node) = current_node.clone() {
                        result.push(node);
                    }

                    if let Ok(path) = validate_path_string(params) {
                        let file_path = path
                            .to_str()
                            .ok_or(String::from("Can't convert FILE path to string"))?;
                        current_node = Some(Node::File {
                            path: file_path.into(),
                            content: String::new(),
                        });
                    };
                }
                _ => {
                    eprintln!("Unknown command: {}", cmd);
                }
            }
            cursor = content_start + end_offset + CLOSE.len();
        } else {
            push_output(&file_string[tag_start..], &mut current_node);
            cursor = file_string.len();
            break;
        }
    }
    push_output(&file_string[cursor..], &mut current_node);
    if let Some(node) = current_node {
        result.push(node);
    }
    Ok(result)
}
pub fn validate_path_string(str_path: &str) -> Result<PathBuf, String> {
    let curdir = current_dir().map_err(|_| "Can't get current dir")?;
    let pathbuf_result = PathBuf::from_str(str_path);
    let pathbuf: PathBuf = pathbuf_result.map_err(|_| "Not a path")?;
    let path = pathbuf.as_path();
    validate_path(curdir.as_path(), path)
}

pub fn validate_path(target_root: &Path, relative_path: &Path) -> Result<PathBuf, String> {
    let joined = target_root.join(relative_path);
    //let canonical_root = joined.canonicalize().map_err(|e| e.to_string())?;

    if joined
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err("Target reaches outside parent directory".into());
    };

    Ok(relative_path.to_path_buf())
}

fn get_config_dir() -> PathBuf {
    if let Ok(path) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(path).join("tmplr");
    }

    if let Ok(path) = env::var("HOME") {
        return PathBuf::from(path).join(".config").join("tmplr");
    }

    PathBuf::from(".")
}

pub fn list_templates_relative(path: &Path) -> Vec<PathBuf> {
    file_scanner::FileScanner::new_with_extension(path, EXTENSION.into())
        .flatten()
        .map(|p| diff_paths(&p, path).unwrap_or(p))
        .collect()
}

pub fn templates_dir() -> PathBuf {
    get_config_dir()
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use predicates::prelude::*;

    #[test]
    fn can_list_templates() -> Result<(), Box<dyn std::error::Error>> {
        let template_dir = assert_fs::TempDir::new()?;

        _ = template_dir.child("t1").child("ex1.tmplr").touch();
        _ = template_dir.child("t1").child("ex2.tmplr").touch();
        _ = template_dir.child("ex3.tmplr").touch();
        _ = template_dir.child("t2").child("ex4.tmplr").touch();
        _ = template_dir
            .child("t2")
            .child("sub")
            .child("ex5.tmplr")
            .touch();
        _ = template_dir
            .child("t2")
            .child("sub")
            .child("ex6.tmplr")
            .touch();

        let templates = list_templates_relative(template_dir.path());
        assert_eq!(templates.len(), 6);

        let templates_str = templates.iter().map(|p| p.to_str().unwrap());

        let predicate_iterator = predicate::in_iter(templates_str);

        assert!(predicate_iterator.eval("ex3.tmplr"));
        assert!(predicate_iterator.eval("t2/sub/ex6.tmplr"));

        Ok(())
    }
}
