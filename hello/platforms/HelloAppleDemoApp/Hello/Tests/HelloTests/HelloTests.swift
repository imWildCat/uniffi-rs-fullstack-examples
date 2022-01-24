import XCTest
@testable import Hello

final class HelloTests: XCTestCase {
    func testExample() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct
        // results.
//      XCTAssertEqual(Hello.text, "Hello, World!")
      XCTAssertEqual(rustGreeting(name: "Bob"), "Hello, Bob!")
    }
}
