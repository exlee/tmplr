use std::{collections::HashMap, fs};

use crate::{quit_with_error, template::{Node, TemplateRequest, read_template, validate_path_string}};


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
    let err_quit = |_| {
        quit_with_error(1, "Invalid path in template definition".into());
        unreachable!();
    };
    let content = render(content, context);
    let path_str = render(path_str, context);
    let pathbuf = validate_path_string(path_str.as_str()).unwrap_or_else(err_quit);
    if let Some(parent_dir) = pathbuf.parent() {
        _ = fs::create_dir_all(parent_dir);
    }
    let content = content.trim();
    println!("Writing: {}", path_str);
    assert!(fs::write(pathbuf.as_path(), content).is_ok());
}

pub(crate) fn make(request: TemplateRequest) {
    let template_result = read_template(&request.path);
    let Ok(template_entities) = template_result else {
        eprintln!("Error: {}", template_result.unwrap_err());
        return;
    };

    if request.dry_run {
        // Dry Run
        for entity in template_entities {
            match entity {
                Node::File { path, content } => preview_file(&path, &content, &request.context),
                Node::Dir(path) => println!("\n{{### DIR {} ###}}", path.to_str().unwrap()),
            }
        }
    } else {
        // Materialize
        for entity in template_entities {
            match entity {
                Node::File { path, content } => render_to_file(&path, &content, &request.context),
                Node::Dir(path) => {
                  println!("Creating dir: {}", path.to_str().expect("Can't create dir"));
                  _ = fs::create_dir_all(path);
                }
            }
        }
    }
}

fn preview_file(path_str: &str, content: &str, context: &HashMap<String, String>) {
    let content = render(content, context);
    let path_str = render(path_str, context);
    let content = content.trim();
    println!("\n{{### FILE {} ###}}", path_str);
    println!("{}", content);
}
