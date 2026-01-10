use std::{
    collections::HashMap,
    fmt::Write,
    fs::{self},
    path::PathBuf,
};

use crate::{
    MakeArgs,
    error_handling::UnwrapQuit,
    template::{Node, read_template, validate_path_string},
};

pub fn render(template: &str, ctx: &HashMap<String, String>) -> String {
    let mut output = String::with_capacity(template.len());
    let mut chars = template.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() == Some(&'{') {
            chars.next();

            let mut inner = String::new();
            while let Some(ic) = chars.next() {
                if ic == '}' && chars.peek() == Some(&'}') {
                    chars.next();
                    break;
                }
                inner.push(ic);
            }

            let parts: Vec<&str> = inner.split('|').map(|s| s.trim()).collect();

            let key = parts[0];

            if let Some(val) = ctx.get(key) {
                let mut res = val.clone();
                for filter in &parts[1..] {
                    match *filter {
                        "upper" => res = res.to_uppercase(),
                        _ => eprintln!("Unknown filter: {}", filter),
                    }
                }
                output.push_str(&res);
            } else {
                output.push_str("{{");
                output.push_str(&inner);
                output.push_str("}}");
            }
            continue;
        }
        output.push(c);
    }
    output
}

fn render_to_file(path_str: &str, content: &str, context: &HashMap<String, String>) {
    let mut context = context.clone();
    update_context_with_magic_vars(&mut context, path_str);
    let content = render(content, &context);
    let path_str = render(path_str, &context);
    let pathbuf =
        validate_path_string(path_str.as_str()).unwrap_or_quit(1, "Invalid template definition");
    if let Some(parent_dir) = pathbuf.parent() {
        _ = fs::create_dir_all(parent_dir);
    }
    let content = content.trim();
    println!("Writing: {}", path_str);
    assert!(fs::write(pathbuf.as_path(), content).is_ok());
}
fn render_or_extend(path_str: &str, content: &str, context: &HashMap<String, String>) {
    let content = render(content, context);
    let path_str = render(path_str, context);
    let pathbuf =
        validate_path_string(path_str.as_str()).unwrap_or_quit(1, "Invalid template definition");
    let content = content.trim();

    if pathbuf.exists() {
        let existing_content =
            std::fs::read_to_string(&pathbuf).unwrap_or_quit(2, "Can't read file for extension");
        if existing_content.contains(content) {
            eprintln!(
                "WARN: {} already contains identical content, not extending!",
                pathbuf.to_string_lossy()
            );
            return;
        }
        let mut new_content = String::new();
        let _ = new_content.write_str(&existing_content);
        write!(new_content, "\n{}", content).unwrap_or_quit(1, "Can't extend content");
        println!("Extending: {}", path_str);
        assert!(fs::write(pathbuf.as_path(), new_content).is_ok());
    } else {
        if let Some(parent_dir) = pathbuf.parent() {
            _ = fs::create_dir_all(parent_dir);
        }
        println!("Writing: {}", path_str);
        assert!(fs::write(pathbuf.as_path(), content).is_ok());
    }
}

pub(crate) fn make(args: &MakeArgs) {
    let template_result = read_template(&args.template_path);
    let Ok(template_entities) = template_result else {
        eprintln!("Error: {}", template_result.unwrap_err());
        return;
    };

    if args.dry_run {
        // Dry Run
        for entity in template_entities {
            match entity {
                Node::File { path, content } | Node::Ext { path, content } => {
                    preview_file(&path, &content, &args.variables)
                }
                Node::Dir(path) => println!("\n{{### DIR {} ###}}", path.to_str().unwrap()),
            }
        }
    } else {
        // Materialize
        for entity in template_entities {
            match entity {
                Node::File { path, content } => render_to_file(&path, &content, &args.variables),
                Node::Dir(path) => {
                    println!("Creating dir: {}", path.to_str().expect("Can't create dir"));
                    _ = fs::create_dir_all(path);
                }
                Node::Ext { path, content } => render_or_extend(&path, &content, &args.variables),
            }
        }
    }
}

fn preview_file(path_str: &str, content: &str, context: &HashMap<String, String>) {
    let mut context = context.clone();
    let path_str = render(path_str, &context);

    update_context_with_magic_vars(&mut context, &path_str);

    let content = render(content, &context);
    let content = content.trim();
    println!("\n{{### FILE {} ###}}", path_str);
    println!("{}", content);
}

fn update_context_with_magic_vars(context: &mut HashMap<String, String>, path_str: &str) {
    let pathbuf = PathBuf::from(&path_str);
    context.insert("$path".to_string(), path_str.to_string());
    if let Some(filename) = pathbuf.file_name() {
        let filename_str: String = filename.to_string_lossy().to_string();
        context.insert("$file".to_string(), filename_str);
    }
}
