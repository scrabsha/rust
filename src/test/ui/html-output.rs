// Allows us to test that the compiler emits a correct html output.
//
// The error in this test allows us to test the following features:
//   - the output is colored as it is when displayed on the terminal,
//   - the margin between each <pre> element is explicitely removed,
//   - text is in bold where expected.

// html-output

fn main(_: ()) {}
//~^ ERROR `main` function has wrong type [E0580]
