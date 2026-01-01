# tmplr

tmplr (pronounced "templar") is a small and simple utility to kickstart any file from `.tmplr` format:
```
tmplr

	https://github.com/exlee/tmplr
	A simple template instantiation utility.

Usage:

	make   <TEMPLATE_FILE/TEMPLATE_NAME> <NAME> VAR=VAL...
	create <TEMPLATE_FILE> <NAME>
	list   List available templates
```

## Installation

```
git clone git@github.com:exlee/tmplr.git
cargo install --path .
```

Or download [binary built by CI](https://github.com/exlee/tmplr/tags)

## Usage

### .tmplr files

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

### CLI

Once you have your .tmplr you can simply:

```
tmplr make /path/to/example1.tmplr John
```

which would create `hello_John.txt` with `Hello, dear John!` in the current directory.

Note that:
- 2nd positional (name), always have to be present, even if there is no `{{name}}` in the template!
- remaining arguments should be provided in `key=value` form, e.g. `tmplr /path/to/some.tmplr MyFile a=1 b=9`

### Templates directory

If you run `tmplr list` you'll get (probably empty) listing of your local templates directory along with a line indicating where your template directory is (_spoiler alert: XDGConfig directory_).

E.g. for me it's:
```
[~]> tmplr list
Listing template dir: /Users/xlii/.config/tmplr
- script.tmplr
```

This "script" is a `script.tmplr` template that I can use to easily produce files from this template using a simple `tmplr script MyScriptName` command.

## TODO

- Add feedback regarding creation of the templates
- Add current tmplr version
- Add "preview" command for templates
- Add own templates as examples
- Add filters for variables (upper/lower)
- Add special variables (for creating things based on paths, e.g. for Elixir `alpha/bravo/charlie/delta.ex` would be `defmodule Alpha.Bravo.Charlie.Delta`)
