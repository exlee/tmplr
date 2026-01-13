package documentation

readme: {
	sections: {
		tmplr: """
			Zero-friction project scaffolding for your terminal.

			tmplr (pronounced "templar") turns a single text file into a complex directory structure. It's a lightweight, logic-free alternative to heavy tools like Cookiecutter. No python dependencies, no complex configuration—just simple substitution.
			"""
		why_tmplr: """
			Single Source of Truth: Define multiple files and directories in one readable .tmplr file.
			Dead Simple: No logic loops or complex functions. Just {{ variable }} substitution.
			Fast: Written in Rust for instant execution.
			"""
		quick_start: """
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
			"""
		todo: """
			- [ ] Add filters for variables (upper/lower)
			- [ ] Add special variables (for creating things based on paths, e.g. for Elixir `alpha/bravo/charlie/delta.ex` would be `defmodule Alpha.Bravo.Charlie.Delta`)
			- [ ] Add feedback during creation of the templates
			- [x] ~Add current tmplr version~
			- [x] ~Add own templates as examples~
			- [x] ~Add "preview" command for templates~
			- [x] ~Add partial matching upon creation~
			- [x] ~Add "append" command (for inserting data in existing file)~
			- [x] ~Make listing nicer~
			"""
		templates_directory: """
			If you run `tmplr list` you'll get (probably empty) listing of your local templates directory along with a line indicating where your template directory is (_spoiler alert: XDGConfig directory_).

			E.g. for me it's:
			```
			[~]> tmplr list
			Listing template dir: /Users/xlii/.config/tmplr
			- script.tmplr
			```

			This "script" is a `script.tmplr` template that I can use to easily produce files from this template using a simple `tmplr script MyScriptName` command.
			"""
		cli: """
			Once you have your .tmplr you can simply:

			```
			tmplr make /path/to/example1.tmplr John
			```

			which would create `hello_John.txt` with `Hello, dear John!` in the current directory.

			Note that:
			- 2nd positional (name), always have to be present, even if there is no `{{name}}` in the template!
			- remaining arguments should be provided in `key=value` form, e.g. `tmplr /path/to/some.tmplr MyFile a=1 b=9`
			"""
		magic_variables: """
			`.tmplr` supports following "magic" variables:

			* **$path** which expands to expanded file path (relative to expansion root)
			* **$file** which expands to current file name (with extension)
			"""
		installation: """
      ```
      git clone \(git_repo)
      cargo install --path .
      ```

      Or download [binary built by CI](\(releases_url))
      """
		tmplr_files: """
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
			"""
		section_types: """
			`.tmplr` files use specific headers to define how files are generated.

			* **Preamble (Comments)**
			    Any text appearing before the first section header is treated as a comment. Use this space to document your template.

			* `{### FILE path/to/name ###}`
			    **Overwrites** the target file. If the file exists, its content is completely replaced by the template content.

			* `{### EXT path/to/name ###}`
			    **Appends** to the target file.
			    * It checks for duplicates: content is only appended if it is not already present.
			    * If the file does not exist, it behaves exactly like `FILE`.

			* `{### DIR path/to/dir ###}`
			    **Creates an empty directory**. This section does not support content/body text.### .tmplr sections
			"""
	}
	full: """
  # tmplr

  \(sections.tmplr)

  ## Why tmplr?

  \(sections.why_tmplr)

  # Quick Start

  \(sections.quick_start)

  # CLI Help

  ```
  \(help.text)
  ```

  # Installation

  \(sections.installation)

  # Usage (extended)

  ## .tmplr files

  \(sections.tmplr_files)

  ### Section Types

  \(sections.section_types)

  ### Magic Variables

  \(sections.magic_variables)

  ## CLI

  \(sections.cli)

  ## Templates directory

  \(sections.templates_directory)

  # TODO

  \(sections.todo)

  """
}
