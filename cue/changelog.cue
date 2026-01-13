package documentation

import (
	"strings"
)

changelog: entries: {
	"0.0.8": {
		date: "2026-01-11"
		added: [
			"Support for `magic` variables {{ $path }} and {{ $file }}",
			"Interactive mode when subcommand is not specified",
			"Interactive mode when {{ name }} variable is missing",
			"Fuzzy finder for templates in interactive mode",
		]
	}
	"0.0.7": {
		date: "2026-01-08"
		changed: [
			"Formatting of the `tmplr list` output into a tree like output",
		]
	}
	"0.0.6": {
		date: "2026-01-07"
		added: [
			"Support for `EXT` sections in templates. This allows content to be appended to existing files rather than overwriting them.",

		]
		changed: [
			"Updated README.md to document new section types.",
		]
	}
	"0.0.5": {
		date: "2026-01-06"
		added: [
			"Partial matching logic for the `create` command, improving ease of use when selecting templates.",
		]
	}

	"0.0.4": {
		date: "2026-01-05"
		added: [
			"New `--simple` flag.",
			"Added initial set of bundled templates.",
		]

		changed: [
			"Refactored and simplified internal project structure.",
		]
	}

	"0.0.3": {
		date: "2026-01-05"
		added: [
			"Added `--files` flag to the `create` command.",
			"Added `-C` flag to specify directory context for execution.",
			"Added additional example templates.",
		]
	}

	"0.0.2": {
		date: "2026-01-02"
		added: [
			"Introduced template preview functionality.",
		]

		changed: [
			"Converted `preview` into a CLI flag rather than a standalone command.",
			"Internal code reorganization and function moves.",
		]
	}

	"0.0.1": {
		date: "2026-01-01"
		added: [
			"Initial working release.",
			"GitHub Actions build configuration.",
			"LICENSE file.",
			"Initial README documentation.",
		]
	}
}

changelog: entries: [key=string]: this = {
	changed: [...string] | *["Nothing"]
	added: [...string] | *["Nothing"]
	_added_prefixed: [for k, v in added {"- \(v)"}]
	_added_full: strings.Join(_added_prefixed, "\n")
	_changed_prefixed: [for k, v in this.changed {"- \(v)"}]
	_changed_full: strings.Join(_changed_prefixed, "\n")
	generated:     """
  ## [v\(key)] - \(this.date)

  ### Added
  \(_added_full)

  ### Changed
  \(_changed_full)

  """
}
changelog: text: strings.Join([for k, v in changelog.entries {v.generated}], "\n")
