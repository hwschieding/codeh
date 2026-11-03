use std::assert_matches;

use crate::cli::*;

#[test]
fn test_arg_parse() {
    let args = vec![
        "exec name",
        "new",
        "-model",
        "myModel",
        "-provider",
        "myProvider",
    ]
    .into_iter()
    .map(|s| String::from(s))
    .collect::<Vec<_>>();

    let raw = RawArgs::from_args(args).expect("RawArgs should build successfully");

    assert_matches!(raw.command(), Some(RunMode::New));
    assert_eq!(raw.model().expect("model should be present"), "myModel");
    assert_eq!(
        raw.provider().expect("provider should be present"),
        "myProvider"
    );
}
