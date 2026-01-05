# tmplr
Zero-friction project scaffolding for your terminal.

tmplr (pronounced "templar") turns a single text file into a complex directory structure. It's a lightweight, logic-free alternative to heavy tools like Cookiecutter. No python dependencies, no complex configuration—just simple substitution.

## Why tmplr?
Single Source of Truth: Define multiple files and directories in one readable .tmplr file.
Dead Simple: No logic loops or complex functions. Just {{ variable }} substitution.
Fast: Written in Rust for instant execution.

# Quick Start
1. Create a template (script.tmplr)
```plain
{### FILE {{name}}.sh ###}
#!/bin/bash
echo "Running {{name}}..."
```

2. Run it

```bash
tmplr make script.tmplr MyScript
```

3. Result: A new file `MyScript.sh` is created instantly.

# CLI Help

```
tmplr (v0.0.5)

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
```

# Installation

```
git clone git@github.com:exlee/tmplr.git
cargo install --path .
```

Or download [binary built by CI](https://github.com/exlee/tmplr/releases)

# Usage (extended)

## .tmplr files

In order to start, you need to have a `.tmplr` file. It's syntax is simple and is similar to format of [txtar ](https://pkg.go.dev/golang.org/x/tools/txtar#hdr-Txtar_format). For example `example0.tmplr` could look like this:

```
{### DIR empty_dir ###}
{### FILE some_file.txt ###}
Here is a a content of text file
{### FILE some_other_file.txt ###}
Here is a a content of other text file
```

Such file doesn't provide that much benefit, so you can add any variable in double braces.
This variable can be both in filename and in file content, e.g. `example1.tmplr`:

```
{### FILE hello_{{ name }}.txt ###}
Hello, dear {{ name }}!
```

There is no template functions or anything whatsoever. Only simple substitution!

## CLI

Once you have your .tmplr you can simply:

```
tmplr make /path/to/example1.tmplr John
```

which would create `hello_John.txt` with `Hello, dear John!` in the current directory.

Note that:
- 2nd positional (name), always have to be present, even if there is no `{{name}}` in the template!
- remaining arguments should be provided in `key=value` form, e.g. `tmplr /path/to/some.tmplr MyFile a=1 b=9`

## Templates directory

If you run `tmplr list` you'll get (probably empty) listing of your local templates directory along with a line indicating where your template directory is (_spoiler alert: XDGConfig directory_).

E.g. for me it's:
```
[~]> tmplr list
Listing template dir: /Users/xlii/.config/tmplr
- script.tmplr
```

This "script" is a `script.tmplr` template that I can use to easily produce files from this template using a simple `tmplr script MyScriptName` command.

# TODO

- [ ] Add filters for variables (upper/lower)
- [ ] Add "append" command (for inserting data in existing file)
- [ ] Make listing nicer
- [ ] Add special variables (for creating things based on paths, e.g. for Elixir `alpha/bravo/charlie/delta.ex` would be `defmodule Alpha.Bravo.Charlie.Delta`)
- [ ] Add feedback during creation of the templates
- [x] ~Add current tmplr version~
- [x] ~Add own templates as examples~
- [x] ~Add "preview" command for templates~
- [x] ~Add partial matching upon creation~
