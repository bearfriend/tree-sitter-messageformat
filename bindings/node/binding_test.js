const assert = require("node:assert");
const { test } = require("node:test");

const Parser = require("tree-sitter");
const { Query } = Parser;

const language = require(".");

function parse(source) {
  const parser = new Parser();
  parser.setLanguage(language);
  return parser.parse(source);
}

test("can load grammar", () => {
  const parser = new Parser();
  assert.doesNotThrow(() => parser.setLanguage(language));
});

test("parses a plural message into the expected structure", () => {
  const tree = parse('a: "{n, plural, one {x} other {y}}"');
  assert.strictEqual(tree.rootNode.hasError, false);
  assert.strictEqual(
    tree.rootNode.toString(),
    "(source_file (resource (key (identifier)) (value (quoted_string (complex_message " +
      "(selector (identifier) (plural) (plural_rules " +
      "(plural_case (plural_key) (plural_case_body)) " +
      "(plural_case (plural_key) (plural_case_body)))))))))"
  );
});

test("parses the export default wrapper and self-closing tags", () => {
  const tree = parse('export default {\n\ta: "x<br/>y"\n};\n');
  assert.strictEqual(tree.rootNode.hasError, false);
  assert.deepStrictEqual(
    tree.rootNode.children.map((c) => c.type),
    ["module_start", "resource", "module_end"]
  );
});

test("reports errors for invalid input", () => {
  for (const source of ['a: "unterminated', 'a: "{n, plural, one x}"', "a: unquoted"]) {
    assert.strictEqual(parse(source).rootNode.hasError, true, source);
  }
});

test("highlights query compiles and captures keys and keywords", () => {
  const fs = require("node:fs");
  const path = require("node:path");
  const source = fs.readFileSync(
    path.join(__dirname, "../../queries/highlights.scm"),
    "utf8"
  );
  const query = new Query(language, source);
  const tree = parse('greeting: "{n, plural, other {x}}"');
  const captures = query
    .captures(tree.rootNode)
    .map((c) => `${c.name}:${c.node.text}`);
  assert.ok(captures.includes("property:greeting"), captures.join());
  assert.ok(captures.includes("keyword:plural"), captures.join());
});
