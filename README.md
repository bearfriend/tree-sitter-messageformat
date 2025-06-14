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
  - Language injections for nested content

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
tree-sitter parse example.msg

# Test syntax highlighting
tree-sitter highlight example.msg

# Generate HTML with highlighting
tree-sitter highlight example.msg --html > output.html
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

The parser recognizes these file extensions:
- `.msg`
- `.msg.js`
- `.msg.json`
- `.messages`
- `.xml` (when containing MessageFormat content)

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

### Neovim (with nvim-treesitter)

Add to your treesitter configuration:

```lua
require'nvim-treesitter.configs'.setup {
  ensure_installed = { "messageformat" },
  highlight = {
    enable = true,
  },
}
```

### Vim (with vim-treesitter)

```vim
autocmd BufNewFile,BufRead *.msg set filetype=messageformat
autocmd BufNewFile,BufRead *.msg.js set filetype=messageformat
```

### VS Code

Install a tree-sitter extension that supports custom grammars, or use the TextMate grammar generated from this parser.

### Emacs

Use `tree-sitter-mode` with this parser for syntax highlighting.

## Highlighting Classes

The parser provides these semantic highlighting classes:

| Element | Class | Description |
|---------|-------|-------------|
| Comments | `comment` | Line comments starting with `//` |
| Keys | `property` | Resource keys (identifiers and quoted strings) |
| Strings | `string` | Quoted strings and template strings |
| Variables | `variable` | General identifiers |
| Parameters | `variable.parameter` | Variables in placeholders |
| Keywords | `keyword` | `plural`, `select`, `selectordinal`, `offset:` |
| Types | `type` | `number`, `date`, `time` |
| Constants | `constant.builtin` | Plural keywords (`zero`, `one`, etc.) |
| Numbers | `number` | Numeric literals |
| Operators | `operator` | `=`, `::`, `/` |
| Brackets | `punctuation.bracket` | `{`, `}`, `<`, `>` |
| Delimiters | `punctuation.delimiter` | `:`, `,`, `"`, `'`, `` ` `` |
| Special | `string.special` | Skeleton format patterns |
| Escapes | `string.escape` | Escaped characters and literals |
| Tags | `tag` | HTML-like tags |
| Pound | `variable.builtin` | `#` placeholder in plurals |

## Development

### Building

```bash
# Generate parser
npm run generate

# Build for Node.js
npm run build

# Run tests
npm test

# Test highlighting
npm run test:highlight
```

### Testing

The parser includes comprehensive tests for:
- Basic syntax elements
- Complex nested structures
- Edge cases and error handling
- Escaping and special characters
- All MessageFormat features

```bash
tree-sitter test
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

MIT License. See LICENSE file for details.

## Links

- [Tree-sitter](https://tree-sitter.github.io/)
- [ICU MessageFormat](https://unicode-org.github.io/icu/userguide/format_parse/messages/)
- [MessageFormat.js](https://messageformat.github.io/)