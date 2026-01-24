use std::{collections::HashMap, path::PathBuf};

#[cfg(debug_assertions)]
use std::{env::current_dir, str::FromStr};

use crate::list_templates::fuzzy_select_template;

mod empty_dir_scanner;
mod error_handling;
mod file_scanner;
mod gen_template;
mod list_templates;
mod render_template;
mod template;

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
struct EchoArgs {
    template_path: PathBuf,
}
#[derive(Debug)]
enum AppArgs {
    List,
    Create(CreateArgs),
    Make(MakeArgs),
    Echo(EchoArgs),
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
        AppArgs::Echo(echo_args) => render_template::echo(&echo_args),
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

    if pargs.contains(["-h", "--help"]) {
        print_help_and_exit(0);
    }

    let Ok(Some(subcommand)) = pargs.subcommand() else {
        return run_interactive();
    };

    match subcommand.to_lowercase().as_str() {
        #[cfg(debug_assertions)]
        "dbg" => Ok(AppArgs::Debug {}),
        "make" => {
            let dry_run = pargs.contains(["-n", "--dry-run"]);
            let mut template_path: Option<PathBuf> = pargs.opt_free_from_str()?;
            let mut instance_name: Option<String> = pargs.opt_free_from_str()?;

            let mut ctx: HashMap<String, String> = HashMap::new();

            for var in pargs.finish() {
                let str: String = var.into_string().unwrap_or("".into());

                if let Some((key, value)) = str.split_once("=") {
                    ctx.insert(key.into(), value.into());
                }
            }

            if template_path.is_none() {
                template_path = fuzzy_select_template();
                if template_path.is_none() {
                    print_help_and_exit(1);
                }
            }

            if instance_name.is_none() {
                instance_name = dialoguer::Input::new()
                    .with_prompt("{{ name }}")
                    .interact_text()
                    .ok();
            }

            let template_path = template_path.expect("Missing template path");
            ctx.insert("name".into(), instance_name.unwrap());

            let cmd = AppArgs::Make(MakeArgs {
                template_path,
                variables: ctx,
                dry_run,
            });

            Ok(cmd)
        }
        "echo" => {
            let template_path: PathBuf = pargs.opt_free_from_str()?.ok_or(pico_args::Error::MissingArgument)?;
            let cmd = AppArgs::Echo(EchoArgs {
                template_path,
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
                .unwrap_or(String::from("."));

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

fn run_interactive() -> Result<AppArgs, pico_args::Error> {
    let mut ctx: HashMap<String, String> = HashMap::new();
    let template_path = match fuzzy_select_template() {
        Some(t) => t,
        None => {
            println!("Aborted.");
            std::process::exit(1);
        }
    };

    let instance_name: Option<String> = dialoguer::Input::new()
        .with_prompt("{{ name }}")
        .interact_text()
        .ok();

    ctx.insert("name".to_string(), instance_name.unwrap());

    let cmd = AppArgs::Make(MakeArgs {
        template_path,
        variables: ctx,
        dry_run: false,
    });

    Ok(cmd)
}

fn print_help_and_exit(code: i32) {
    println!("{}", HELP.trim());
    std::process::exit(code);
}
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/help.rs"));
