# Tree-sitter MessageFormat

A tree-sitter parser for MessageFormat (ICU MessageFormat) syntax with comprehensive syntax highlighting support.

## Overview

This parser provides syntax highlighting and parsing support for MessageFormat files, commonly used for internationalization (i18n) in web applications. It supports the full ICU MessageFormat specification including plurals, selects, date/time formatting, and nested messages.

## Features

- **Complete MessageFormat syntax support**
  - Simple placeholders: `{name}`
  - Number, date, and time formatting with skeleton patterns
  - Plural and select messages
  - Selectordinal support
  - Nested complex messages
  - Escaped characters and literals

- **Rich syntax highlighting** with semantic token types:
  - Comments
  - String literals (quoted and template strings)
  - Property keys
  - Placeholders and variables
  - Keywords (`plural`, `select`, `selectordinal`)
  - Format types (`number`, `date`, `time`)
  - Plural case keywords (`zero`, `one`, `two`, `few`, `many`, `other`)
  - Numbers and operators
  - Skeleton format patterns
  - Tags and escaped characters

- **Additional query support**:
  - Code folding for complex messages
  - Symbol navigation and tagging
  - Local variable scoping
  - Language injection queries (experimental; `queries/injections.scm`)

## Installation

### Node.js

```bash
npm install tree-sitter-messageformat
```

### From source

```bash
git clone https://github.com/bearfriend/tree-sitter-messageformat
cd tree-sitter-messageformat
npm install
```

## Usage

### With tree-sitter CLI

```bash
# Parse a file
tree-sitter parse test/manual/test-working.msg

# Test syntax highlighting
tree-sitter highlight test/manual/test-working.msg

# Generate HTML with highlighting
tree-sitter highlight test/manual/test-working.msg --html > output.html
```

### With Node.js

```javascript
const Parser = require('tree-sitter');
const MessageFormat = require('tree-sitter-messageformat');

const parser = new Parser();
parser.setLanguage(MessageFormat);

const code = 'greeting: "Hello {name}!",';
const tree = parser.parse(code);
console.log(tree.rootNode.toString());
```

## File Extensions

`tree-sitter.json` associates these file types with the grammar:
- `.msg`
- `.msg.js`
- `.msg.json`
- `.messages`

## Syntax Examples

### Basic Placeholders
```messageformat
greeting: "Hello {name}!",
farewell: "Goodbye {user}."
```

### Number and Date Formatting
```messageformat
price: "Cost: {amount, number, ::currency/USD}",
birthday: "Born on {date, date, ::MMM d, y}",
meeting: "Meeting at {time, time, ::h:mm a}"
```

### Plural Messages
```messageformat
items: "{count, plural, =0 {no items} =1 {one item} other {# items}}",
guests: "{count, plural, offset:1 =0 {no guests} =1 {one guest} other {# guests}}"
```

### Select Messages
```messageformat
gender: "{person, select, male {He} female {She} other {They}} is here"
```

### Selectordinal
```messageformat
position: "You finished {place, selectordinal, =1 {1st} =2 {2nd} =3 {3rd} other {#th}}"
```

### Template Strings
```messageformat
welcome: `Welcome {name}! 
You have {msgCount, plural, =0 {no messages} =1 {one message} other {# messages}}.`
```

### Nested Complex Messages
```messageformat
status: "{user, select, admin {Admin {name} has {count, plural, =0 {no tasks} other {# tasks}}} user {User {name}} other {Guest}}"
```

### Escaped Characters
```messageformat
escaped: "Use '{' and '}' for literal braces, '#' for pound"
```

## Editor Integration

### Zed

Use the [MessageFormat extension](https://github.com/bearfriend/messageformat-zed). It pins a commit of this grammar and ships its own copies of `queries/highlights.scm` and `queries/brackets.scm`; see that repository's README for the sync process.

## Highlighting Classes

| Element | Capture | Notes |
|---------|---------|-------|
| Comments | `@comment` | Line comments starting with `//` |
| Resource keys | `@property` | Identifiers and quoted strings |
| Strings | `@string` | Quoted and template strings (quotes are not captured separately) |
| Variables | `@variable` | Selector, placeholder and complex-message arguments |
| Pound placeholder | `@variable.special` | `#` inside plural / selectordinal cases |
| Keywords | `@keyword` | `plural`, `select`, `selectordinal`, `offset:`, `export`, `default` |
| Format types | `@type` | `number`, `date`, `time` |
| Plural keys | `@constant.builtin` | `zero`, `one`, `two`, `few`, `many`, `other` |
| Select keys | `@constant` | `other` is `@constant.builtin` |
| Style formats | `@constant.builtin` | e.g. `short`, `long`, `currency` |
| Numbers | `@number` | Including `=N` plural keys and offsets |
| Skeleton patterns | `@string.special` | The part after `::` |
| Escapes | `@string.escape` | `'{'`, `''`, ... |
| Tags | `@tag` | `<b>`, `</b>`, `<br/>` |
| Operators | `@operator` | `=`, `::`, `/` |
| Brackets | `@punctuation.bracket` | `{ } < >` |
| Delimiters | `@punctuation.delimiter` | `:`, `,`, `"`, `` ` ``, module `;` |

## Development

```bash
npm install            # builds the Node binding from source
npm run generate       # regenerate src/parser.c after editing grammar.js
npm test               # Node binding tests + corpus + highlight tests
cargo test             # Rust binding tests
```

Tests live in:
- `test/corpus/` &mdash; parse-tree tests (`tree-sitter test`)
- `test/highlight/` &mdash; highlight assertions (run by `tree-sitter test`)
- `test/fixtures/` &mdash; fixtures used by the binding tests
- `test/manual/` &mdash; larger sample files for trying things out with `tree-sitter parse` / `tree-sitter highlight`; not run automatically

After changing `grammar.js`, always regenerate with the CLI version pinned in `package.json` (`npx tree-sitter generate`) and commit `src/`.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

MIT &copy; 2026 Danny Gleckler. See [LICENSE](LICENSE).

## Links

- [Tree-sitter](https://tree-sitter.github.io/)
- [ICU MessageFormat](https://unicode-org.github.io/icu/userguide/format_parse/messages/)
- [MessageFormat.js](https://messageformat.github.io/)