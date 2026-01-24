use std::{
    env::{self, current_dir},
    fmt::Write,
    fs::{self},
    io,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::error_handling::quit_with_error;
use crate::{error_handling::OkOrIoOther, list_templates::list_templates_relative};

pub const EXTENSION: &str = "tmplr";
pub const OPEN: &str = "{###";
pub const CLOSE: &str = "###}";

#[derive(Clone, Debug)]
pub enum Node {
    Dir(PathBuf),
    File { path: String, content: String },
    Ext { path: String, content: String },
}
type Template = Vec<Node>;

pub fn read_template(path: &Path) -> io::Result<Template> {
    let mut result: Template = Vec::new();
    let mut cursor = 0;
    let mut current_node: Option<Node> = None;

    fn push_output(s: &str, current_node: &mut Option<Node>) {
        match current_node {
            None => (),
            Some(Node::File { content, .. }) | Some(Node::Ext { content, .. }) => {
                content.push_str(s)
            }
            Some(Node::Dir { .. }) => quit_with_error(256, "Dir node shouldn't be current one"),
        }
    }

    fn push_current_node(current_node: &mut Option<Node>, result: &mut Template) {
        if let Some(node) = current_node.clone() {
            result.push(node);
            *current_node = None;
        }
    }

    let file_string = get_template_string_from_path(&path)?;

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
                    push_current_node(&mut current_node, &mut result);
                    let file_path = validate_path_string(params)?;
                    let new_dir = Node::Dir(file_path);
                    result.push(new_dir);
                }
                "FILE" => {
                    push_current_node(&mut current_node, &mut result);
                    if let Ok(path) = validate_path_string(params) {
                        let file_path = path
                            .to_str()
                            .ok_or_ioerror("Can't convert FILE path to string")?;
                        current_node = Some(Node::File {
                            path: file_path.into(),
                            content: String::new(),
                        });
                    };
                }
                "EXT" => {
                    push_current_node(&mut current_node, &mut result);
                    if let Ok(path) = validate_path_string(params) {
                        let file_path = path
                            .to_str()
                            .ok_or_ioerror("Can't convert EXT path to string")?;
                        current_node = Some(Node::Ext {
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

pub fn get_template_string_from_path(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
        .or_else(|_| fs::read_to_string(get_config_dir().join(path)))
        .or_else(|_| fs::read_to_string(get_config_dir().join(path).with_added_extension("tmplr")))
        .or_else(|_| read_partial_matched_template(path))
}
pub(crate) fn read_partial_matched_template(path: &Path) -> io::Result<String> {
    let input_path = path.to_string_lossy().to_string();
    let config_dir = get_config_dir();
    let all_templates = list_templates_relative(&config_dir);

    let mut filtered: Vec<String> = all_templates
        .into_iter()
        .map(|p| p.to_string_lossy().to_string())
        .filter(|pstr| pstr.contains(&input_path))
        .collect();

    if filtered.len() > 1 {
        let mut error_msg = String::new();
        let _ = writeln!(error_msg, "Multiple templates matched input string");
        for item in filtered {
            let _ = writeln!(error_msg, "- {}", item);
        }
        return err(&error_msg);
    }
    let m = filtered
        .pop()
        .ok_or_else(|| io::Error::other("Template not found"))?;
    println!("Expanding: {}", m);
    fs::read_to_string(config_dir.join(m))
}

pub fn validate_path_string(str_path: &str) -> io::Result<PathBuf> {
    let curdir = current_dir().map_err(|_| other_err("Can't get current dir"))?;
    let pathbuf_result = PathBuf::from_str(str_path);
    let pathbuf: PathBuf = pathbuf_result.map_err(|_| other_err("Not a path"))?;
    let path = pathbuf.as_path();
    validate_path(curdir.as_path(), path)
}

pub fn validate_path(target_root: &Path, relative_path: &Path) -> io::Result<PathBuf> {
    let joined = target_root.join(relative_path);
    //let canonical_root = joined.canonicalize().map_err(|e| e.to_string())?;

    if joined
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return err("Target reaches outside parent directory");
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

pub fn templates_dir() -> PathBuf {
    get_config_dir()
}
fn err<T>(err: &str) -> io::Result<T> {
    return Err(io::Error::other(err));
}
fn other_err(err: &str) -> io::Error {
    return io::Error::other(err);
}

#[cfg(test)]
mod tests {}
