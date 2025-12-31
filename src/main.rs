use std::{
    collections::HashMap,
    path::{PathBuf},
};

#[cfg(debug_assertions)]
use std::{
    env::current_dir,
    str::FromStr,
};


mod file_scanner;
mod empty_dir_scanner;
mod template;
mod gen_template;

#[derive(Debug)]
enum AppArgs {
    List,
    Create {
      path: PathBuf,
      name: String,
    },
    Make {
        template_path: PathBuf,
        variables: HashMap<String, String>,
    },
  	#[cfg(debug_assertions)]
    Debug,
}

pub fn main() {
    let parsed = parse_args();
    let args = parsed.unwrap_or_else(|err| {
        println!("Error: {}", err);
        print_help_and_exit(0);
        unreachable!();
    });
    match args {
      	#[cfg(debug_assertions)]
        AppArgs::Debug => run_debug(&args),
        AppArgs::Make {
            template_path,
            variables,
        } => template::make(template::TemplateRequest::make(
            template_path,
            variables,
        )),
        AppArgs::List => run_list(),
        AppArgs::Create{path, name} => gen_template::create_template(path, name),
    }
}

fn run_list() {
  let templates_dir = template::templates_dir();
  let templates = template::list_templates_relative(&templates_dir);
	let templates_dir_str = templates_dir.to_str().unwrap_or("ERROR Expanding Config Dir");

	if Vec::is_empty(&templates) {
  	println!("No templates found in: {}", templates_dir_str);
	}

	println!("Listing template dir: {}", templates_dir_str);
  for tmpl_file in templates  {
    println!("- {}", tmpl_file.to_string_lossy());
  }
}
#[cfg(debug_assertions)]
fn run_debug(_args: &AppArgs) {
    let example = PathBuf::from_str("assets/example.template").unwrap();
    let template = template::read_template(example.as_path());
    let mut ctx: HashMap<String, String> = HashMap::new();
    ctx.insert("project_name".into(), "[example_project]".into());
    for node in template.as_ref().unwrap() {
        if let template::Node::File { path, content } = node {
            println!("{}", path);
            println!("{}", template::render(content, &ctx));
        }
    }

    for f in file_scanner::FileScanner::new_with_extension(current_dir().unwrap(), "rs".into()) {
        println!("{:?}", f);
    }
    gen_template::create_template(PathBuf::from("src"), "main".into());
}
fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    let Ok(Some(subcommand)) = pargs.subcommand() else {
        print_help_and_exit(0);
        unreachable!();
    };
    match subcommand.to_lowercase().as_str() {
      	#[cfg(debug_assertions)]
        "dbg" => Ok(AppArgs::Debug {}),
        "make" => {
            let template_path: PathBuf = pargs.free_from_str()?;
            let instance_name: String = pargs.free_from_str()?;
            let mut ctx: HashMap<String, String> = HashMap::new();

            for var in pargs.finish() {
                let str: String = var.into_string().unwrap_or("".into());

								// Clippy complains, but nightly fails to compile
                if str.contains("=") {
                    if let Some((key, value)) = str.split_once("=") {
                        ctx.insert(key.into(), value.into());
                    }
                }
            }

            ctx.insert("name".into(), instance_name);

            let cmd = AppArgs::Make {
                template_path,
                variables: ctx,
            };

            Ok(cmd)
        }
        "create" => {
          let name: String = pargs.free_from_str()?;
          Ok(AppArgs::Create{
            path: PathBuf::from("."),
            name,
          })
        },
        "list" => Ok(AppArgs::List),
        _ => {
            print_help_and_exit(1);
            unreachable!();
        }
    }
}

fn print_help_and_exit(code: i32) {
    print!("{}", HELP);
    std::process::exit(code);
}

fn quit_with_error(code: i32, err: String) {
  eprintln!("Error: {}", err);
  std::process::exit(code);
}


const HELP: &str = "
tmpl
  make   <TEMPLATE_FILE/TEMPLATE_NAME> <NAME> VAR=VAL...
  create <TEMPLATE_FILE> <NAME>
  list   List available templates
";
