package documentation

help: {
	text: """
tmplr (v\(version))

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
"""
}

help: this = {
	code: """
  const HELP: &str = "
  \(this.text)
  ";
  """
}
