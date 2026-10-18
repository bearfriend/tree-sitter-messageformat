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
  - Injection queries for embedding MessageFormat in other languages (see [Embedding in other languages](#embedding-in-other-languages))

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
tree-sitter parse test/manual/test-working.mf

# Test syntax highlighting
tree-sitter highlight test/manual/test-working.mf

# Generate HTML with highlighting
tree-sitter highlight test/manual/test-working.mf --html > output.html
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
- `.mf`
- `.mf.js`

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

## Embedding in other languages

MessageFormat strings usually live inside another language's string literals. This repo ships what an editor needs to highlight them there.

**1. A second grammar, `messageformat_message`** (in [`message/`](message)). The main grammar is rooted at a `key: "value"` resource file, so a bare string such as `You have {n, plural, ...}` doesn't parse with it. The message grammar reuses every rule but is rooted at one bare message. Its queries are in `message/queries/` (`highlights.scm`, `brackets.scm`). In Rust it is `tree_sitter_messageformat::LANGUAGE_MESSAGE`.

**2. Host injection queries** in [`injections/`](injections). Each file is meant to be *added to* the host language's own `injections` query (Neovim `; extends`, Helix `; inherits`, ...), not to replace it:

| File | Host | Triggers |
|------|------|----------|
| `javascript.scm` | JavaScript, TypeScript, TSX | `{arg, plural\|select\|...}` in a string; `new IntlMessageFormat(...)`; `formatMessage(...)`; `{ defaultMessage: ... }`; `/* icu */ "..."` |
| `jsx.scm` | JavaScript, TSX (not plain TypeScript) | `defaultMessage="..."` and ICU-shaped JSX attribute values |
| `json.scm` | JSON (FormatJS, Chrome `messages.json`, Flutter `.arb`) | ICU-shaped values; `"message"` / `"defaultMessage"` members |
| `html.scm` | HTML | ICU-shaped text and attribute values |
| `java.scm` | Java (`java.text`, ICU4J, `android.icu`) | ICU-shaped strings; `new MessageFormat(...)`; `MessageFormat.format(...)` |
| `kotlin.scm` | Kotlin (Android, Multiplatform) | same as Java |
| `swift.scm` | Swift (iOS / macOS) | ICU-shaped strings only; Swift has no built-in ICU API |

A plain `"Hello {name}"` is **not** injected unless a known call or key makes the intent clear, because such strings are too common in ordinary code. "ICU-shaped" means containing `{name, plural|select|selectordinal|number|date|time ...`.

Every query is tested against the real host grammar (`cargo test --test injections`): the exact set of injected strings is asserted using the fixtures in `test/host-injections/`, and every injected string must parse without errors.

### Zed

Zed cannot use these yet. Zed reads injections only from the *host* language's own `injections.scm`, and an extension can't add rules to a built-in language (see [zed#8795](https://github.com/zed-industries/zed/issues/8795) and [zed#65382](https://github.com/zed-industries/zed/issues/65382)). The files in `injections/` are written so they can be proposed to Zed's built-in languages, or adopted if Zed adds a mechanism for extensions.

### Not the same as `queries/injections.scm`

`queries/injections.scm` is for injections *inside* a MessageFormat file (for example, what to do with a template string). It is unrelated to the host injections above.

## Editor Integration

### Zed

Use the [MessageFormat extension](https://github.com/bearfriend/messageformat-zed). It registers both grammars from this repo (`messageformat` and, via `path = "message"`, `messageformat_message`), pins one commit for both, and ships its own copies of `highlights.scm` and `brackets.scm` from `queries/` and `message/queries/`; see that repository's README for the sync process. A Zed extension cannot add injections to built-in languages, so the host queries in `injections/` don't work in Zed yet.

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
npm run generate       # regenerate src/parser.c and message/src/parser.c after editing grammar.js
npm test               # Node binding tests + corpus + highlight tests
cargo test             # Rust binding tests
```

Tests live in:
- `test/corpus/` &mdash; parse-tree tests (`tree-sitter test`)
- `test/highlight/` &mdash; highlight assertions (run by `tree-sitter test`)
- `message/test/corpus/` &mdash; parse-tree tests for the bare-message grammar (`npm run test:message`)
- `test/host-injections/` &mdash; host-language fixtures for the injection queries (`cargo test --test injections`)
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
