use std::io::Stdin;

fn playground() {

    /// step 1. declare mutable string
    let mut guess = String::new();

    /// step 2. access stdin
    let in0: Stdin = std::io::stdin();

    /// step 3. readline
    let r: std::io::Result<usize> = in0.read_line(&mut guess);

    /// step 4. handle errors (just text for exception)
    let x: usize = r.expect("Failed to read line");

}
