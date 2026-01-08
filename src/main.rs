use std::{collections::HashMap, path::PathBuf};

#[cfg(debug_assertions)]
use std::{env::current_dir, str::FromStr};

mod empty_dir_scanner;
mod file_scanner;
mod gen_template;
mod render_template;
mod template;
mod error_handling;
mod list_templates;

#[derive(Debug)]
struct CreateArgs {
    path: PathBuf,
    name: String,
    files: Option<Vec<PathBuf>>,
    no_replace: bool,
}
#[derive(Debug)]
struct MakeArgs {
    template_path: PathBuf,
    variables: HashMap<String, String>,
    dry_run: bool,
}
#[derive(Debug)]
enum AppArgs {
    List,
    Create(CreateArgs),
    Make(MakeArgs),
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
        AppArgs::Make(make_args) => render_template::make(&make_args),
        AppArgs::List => list_templates::run_list(),
        AppArgs::Create(create_args) => gen_template::create_template(&create_args),
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
            println!("{}", render_template::render(content, &ctx));
        }
    }

    for f in file_scanner::FileScanner::new_with_extension(current_dir().unwrap(), "rs".into()) {
        println!("{:?}", f);
    }
    let create_args = CreateArgs {
        path: PathBuf::from("src"),
        name: "main".into(),
        no_replace: false,
        files: None,
    };
    gen_template::create_template(&create_args);
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
            let dry_run = pargs.contains(["-n", "--dry-run"]);
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

            let cmd = AppArgs::Make(MakeArgs {
                template_path,
                variables: ctx,
                dry_run,
            });

            Ok(cmd)
        }
        "create" => {
            let name: String = pargs.free_from_str()?;
            let listed_files_only = pargs.contains("--files");
            let no_replace = pargs.contains("--simple");
            let working_dir: String = pargs
                .opt_value_from_str("--change-dir")?
                .or_else(|| pargs.opt_value_from_str("-C").expect("Can't unwrap"))
                .or(Some(String::from(".")))
                .unwrap();

            if listed_files_only {
                let mut files: Vec<PathBuf> = Vec::new();
                for var in pargs.finish() {
                    let pathbuf = PathBuf::from(var);
                    if !pathbuf.exists() {
                        continue;
                    }
                    files.push(pathbuf);
                }
                Ok(AppArgs::Create(CreateArgs {
                    path: PathBuf::from(working_dir),
                    files: Some(files),
                    name,
                    no_replace,
                }))
            } else {
                Ok(AppArgs::Create(CreateArgs {
                    path: PathBuf::from(working_dir),
                    name,
                    files: None,
                    no_replace,
                }))
            }
        }
        "list" => Ok(AppArgs::List),
        _ => {
            print_help_and_exit(1);
            unreachable!();
        }
    }
}

fn print_help_and_exit(code: i32) {
    println!("{}", HELP.trim());
    std::process::exit(code);
}


const HELP: &str = "
tmplr (v0.0.7)

	https://github.com/exlee/tmplr
	A simple template instantiation utility.

Usage:

	make    <TEMPLATE_FILE/TEMPLATE_NAME> <NAME> VAR=VAL...

	        Instantiate template. Partial names supported
	        for local templates.

	        --dry-run/-n	don't materialize, only print to stdout

	create  <TEMPLATE_FILE> <NAME>

	        Create new template.

	        -C/--change-dir <DIR>	change directory before creating template
	        --files              	only read files listed in args
	        --simple             	don't replace file contents

	list    List available templates
";
