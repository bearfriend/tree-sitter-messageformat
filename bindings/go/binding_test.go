package tree_sitter_messageformat_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_messageformat "github.com/bearfriend/tree-sitter-messageformat/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_messageformat.Language())
	if language == nil {
		t.Errorf("Error loading messageformat grammar")
	}
}
