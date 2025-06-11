import XCTest
import SwiftTreeSitter
import TreeSitterMessageformat

final class TreeSitterMessageformatTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_messageformat())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading messageformat grammar")
    }
}
