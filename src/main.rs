use std::{
    collections::HashMap, path::{Path, PathBuf}, env, str::FromStr
};

mod template;

pub fn main() {
  	let parsed = parse_args();
  	let args = parsed.unwrap_or_else(|err| {
    	println!("Error: {}", err);
    	print_help_and_exit(0);
      unreachable!();
    });
    match args {
      AppArgs::Debug => run_debug(&args),
      AppArgs::Make{template_path, instance_name, variables} =>
        template::make(
          template::TemplateRequest::make(template_path, instance_name, variables)
        ),
      _ => todo!(),
    }
}

fn run_debug(args: &AppArgs) {
  	dbg!(&args);
    let example = PathBuf::from_str("assets/example.template").unwrap();
    let template = template::read_template(example.as_path());
    let mut ctx: HashMap<String, String> = HashMap::new();
    ctx.insert("project_name".into(), "[example_project]".into());
    dbg!(&template);
    for node in template.as_ref().unwrap() {
        if let template::Node::File { path, content } = node {
            println!("{}", path.to_str().unwrap());
            println!("{}", template::render(content, &ctx));
        }
    }
}
#[derive(Debug)]
enum AppArgs {
	List,
	Create,
	Make{template_path: PathBuf, instance_name: String, variables: HashMap<String,String>},
	Debug,
}
fn parse_args() -> Result<AppArgs, pico_args::Error> {
  let mut pargs = pico_args::Arguments::from_env();

	let Ok(Some(subcommand)) = pargs.subcommand() else {
  	print_help_and_exit(0);
  	unreachable!();
	};
  match String::from(subcommand).to_lowercase().as_str() {
    "dbg" => {
      Ok(AppArgs::Debug{})
    }
    "make" => {
      let template_path: PathBuf = pargs.free_from_str()?;
      let instance_name: String = pargs.free_from_str()?;
      let mut ctx : HashMap<String,String> = HashMap::new();

      for var in pargs.finish() {
       	let str: String = var.into_string().unwrap_or("".into());
       	if str.contains("=") {
         	if let Some((key, value)) = str.split_once("=") {
           	ctx.insert(key.into(), value.into());
         	}
       	}
      }

			let cmd = AppArgs::Make { template_path, instance_name, variables: ctx };

			Ok(cmd)
     },
    "create" => todo!(),
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


const HELP: &str = "
templar
  make   <TEMPLATE_FILE/TEMPLATE_NAME> <NAME> VAR=VAL...
  create
  list   List available templates

";
