use std::{
    collections::HashMap,
    env::current_dir,
    error::Error,
    fs::{self, File},
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Clone, Debug)]
pub enum Node {
    Dir(PathBuf),
    File { path: PathBuf, content: String },
}
type Template = Vec<Node>;

struct TemplateContext {
    vars: HashMap<String, String>,
    filters: HashMap<String, fn(&str) -> String>,
}

pub struct TemplateRequest {
    path: PathBuf,
    instance_name: String,
    context: HashMap<String, String>,
}

impl TemplateRequest {
    pub fn make(
        path: PathBuf,
        instance_name: String,
        context: HashMap<String, String>,
    ) -> TemplateRequest {
        TemplateRequest {
            path,
            instance_name,
            context,
        }
    }
}

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

const FILE_START: &str = "FILE";
const FILE_END: &str = "END_FILE";
const DIR_PREFIX: &str = "DIR";

pub fn read_template(path: &Path) -> Result<Template, String> {
    let mut result: Template = Vec::new();
    let mut cursor = 0;
    let mut current_node: Option<Node> = None;

    const OPEN: &str = "{###";
    const CLOSE: &str = "###}";

    fn push_output(s: &str, current_node: &mut Option<Node>) {
        if let Some(Node::File { content, .. }) = current_node {
            content.push_str(s);
        }
    }

    let file_string = fs::read_to_string(path).unwrap_or("Can't read file".to_string());

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
                    let new_dir = Node::Dir(file_path);
                    result.push(new_dir);
                }
                "FILE" => {
                    if let Some(node) = current_node {
                        result.push(node);
                    }
                    let file_path = validate_path_string(params)?;
                    current_node = Some(Node::File {
                        path: file_path,
                        content: String::new(),
                    })
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
fn validate_path(target_root: &Path, relative_path: &Path) -> Result<PathBuf, String> {
    let joined = target_root.join(relative_path);
    let canonical_root = target_root.canonicalize().map_err(|e| e.to_string())?;

    if relative_path
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err("Target reaches outside parent directory".into());
    };

    Ok(joined)
}

fn validate_path_string(str_path: &str) -> Result<PathBuf, String> {
    let curdir = current_dir().map_err(|_| "Can't get current dir")?;
    let pathbuf_result = PathBuf::from_str(str_path);
    let pathbuf: PathBuf = pathbuf_result.map_err(|_| "Not a path")?;
    let path = pathbuf.as_path();
    validate_path(curdir.as_path(), path)
}

pub(crate) fn make(request: TemplateRequest) {
    let template_result = read_template(&request.path);
    let Ok(template_entities) = template_result else {
        eprintln!("Error: {}", template_result.unwrap_err());
        return;
    };
    for entity in template_entities {
        match entity {
            Node::File { path, content } => render_to_file(&path, &content, &request.context),
            Node::Dir(path) => create_directory(path),
        }
    }
}

fn create_directory(path: PathBuf) {
    println!("Create Directory: {}", path.to_string_lossy());
    //todo!()
}

fn render_to_file(path: &PathBuf, content: &String, context: &HashMap<String, String>) {
    let result = render(content, context);
    println!("Path: {}", path.to_string_lossy());
    print!("{}", result);
    // write_to_file(path, content);
}
