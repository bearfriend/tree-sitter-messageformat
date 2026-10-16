const root = require("path").join(__dirname, "..", "..");

const binding =
  typeof process.versions.bun === "string"
    // Support `bun build --compile` by being statically analyzable enough to find the .node file at build-time
    ? require(`../../prebuilds/${process.platform}-${process.arch}/tree-sitter-messageformat.node`)
    : require("node-gyp-build")(root);

// The exported object must be extensible: node-tree-sitter attaches
// `nodeSubclasses` to it, which silently fails on a bare (frozen) External.
module.exports = binding;

try {
  module.exports.nodeTypeInfo = require("../../src/node-types.json");
} catch (_) {}
