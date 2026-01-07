use pathdiff::diff_paths;
use std::fmt::Write;
use std::io;
use std::{fs, path::PathBuf};

use crate::CreateArgs;
use crate::error_handling::quit_with_error;
use crate::{
    empty_dir_scanner, file_scanner,
    template::{self, EXTENSION, Node},
};

pub fn create_template(args: &CreateArgs) {
    let result = if let Some(files_iter) = args.files.clone() {
        let empty_dirs_iter = vec![].into_iter();
        let files_iter: Vec<PathBuf> = files_iter;

        create_template_generic(args, files_iter.into_iter().map(Ok), empty_dirs_iter)
    } else {
        let pathbuf = &args.path;
        let files_iter = file_scanner::FileScanner::new(&pathbuf);
        let empty_dirs_iter = empty_dir_scanner::EmptyDirScanner::new(&pathbuf);

        create_template_generic(args, files_iter, empty_dirs_iter)
    };

    if result.is_none() {
        quit_with_error(1, "Error template crash".into());
        unreachable!();
    }
}

fn create_template_generic<T, R>(args: &CreateArgs, files: T, dirs: R) -> Option<()>
where
    T: Iterator<Item = io::Result<PathBuf>>,
    R: Iterator<Item = io::Result<PathBuf>>,
{
    let mut result = String::with_capacity(128 * 1024);
    let pathbuf = &args.path;

    let open = template::OPEN;
    let close = template::CLOSE;

    for dir in dirs.flatten() {
        let dir_pathbuf = dir.clone();
        let relative = diff_paths(&dir_pathbuf, &pathbuf)?;
        let path_str = relative.to_str()?;
        let new_node = create_dir_node(args, path_str);
        if let Node::Dir(path) = new_node {
            let relative = diff_paths(&path, &pathbuf)?;
            let path_str = relative.to_str()?;
            writeln!(result, "{open} DIR {path_str} {close}").unwrap()
        }
    }

    for file in files.flatten() {
        let file = file.clone();
        let file_path: &str = file.to_str()?;
        let new_node = create_node(args, file_path);
        match new_node {
            Node::File { path, content } => {
                        let relative = diff_paths(&path, &pathbuf)?;
                        let path_str = relative.to_str()?;
                        writeln!(result, "{open} FILE {path_str} {close}").unwrap();
                        result.push_str(&content);
                        result.push('\n');
                    }
            Node::Dir(path) => {
                        let relative = diff_paths(&path, &pathbuf)?;
                        let path_str = relative.to_str()?;
                        writeln!(result, "{open} DIR {path_str} {close}").unwrap()
                    }
            Node::Ext { .. } => todo!("Implement after tmplr create --appending is added"),
        }
    }
    let mut filename: String = String::new();
    filename.push_str(args.name.as_str());
    filename.push('.');
    filename.push_str(EXTENSION);

    println!("{}", result);

    _ = fs::write(filename, result);

    Some(())
}

pub fn create_dir_node(args: &CreateArgs, path: &str) -> Node {
    if args.no_replace {
        let pathbuf = template::validate_path_string(path).expect("Path error");
        Node::Dir(pathbuf)
    } else {
        let path = replace_word_bounded(path, &args.name, "{{ name }}");
        let pathbuf = template::validate_path_string(path.as_str()).expect("Path error");

        Node::Dir(pathbuf)
    }
}
pub fn create_node(args: &CreateArgs, path: &str) -> Node {
    let pathbuf = PathBuf::from(path);
    let Ok(content) = fs::read_to_string(pathbuf) else {
        quit_with_error(
            1,
            &format!("Can't read file for template creation: {}", path),
        );
        unreachable!();
    };

    if args.no_replace {
        let path_str = String::from(path);
        Node::File {
            path: path_str,
            content,
        }
    } else {
        let content = replace_word_bounded(&content, &args.name, "{{ name }}");
        let path = replace_word_bounded(path, &args.name, "{{ name }}");

        Node::File { path, content }
    }
}

fn replace_word_bounded(input: &str, target: &str, replacement: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut last_idx = 0;

    let crash_during_replacement = || {
        quit_with_error(1, "Error during replacing string".into());
        unreachable!()
    };

    for (start, _part) in input.match_indices(target) {
        let boundary_start = if start == 0 {
            true
        } else {
            let prev_char = input
                .chars()
                .nth(start - 1)
                .unwrap_or_else(crash_during_replacement);
            !prev_char.is_alphanumeric()
        };

        let end = start + target.len();
        let boundary_end = if end == input.len() {
            true
        } else {
            let next_char = input
                .chars()
                .nth(end)
                .unwrap_or_else(crash_during_replacement);
            !next_char.is_alphanumeric()
        };

        if boundary_start && boundary_end {
            output.push_str(&input[last_idx..start]);
            output.push_str(replacement);
            last_idx = end;
        }
    }
    output.push_str(&input[last_idx..]);
    output
}
