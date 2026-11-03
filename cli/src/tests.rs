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

    println!("{:?}", raw);

    assert_matches!(raw.command(), Some(RunMode::New));
    assert_eq!(raw.model().expect("model should be present"), "myModel");
    assert_eq!(
        raw.provider().expect("provider should be present"),
        "myProvider"
    );
}

#[test]
fn test_arg_parse2() {
    let args = vec!["exec name", "new", "-M", "myModel", "-p", "myProvider"]
        .into_iter()
        .map(|s| String::from(s))
        .collect::<Vec<_>>();

    let raw = RawArgs::from_args(args).expect("RawArgs should build successfully");

    println!("{:?}", raw);

    assert_matches!(raw.command(), Some(RunMode::New));
    assert_eq!(raw.model().expect("model should be present"), "myModel");
    assert_eq!(
        raw.provider().expect("provider should be present"),
        "myProvider"
    );
}

#[test]
fn test_arg_parse3() {
    let args = vec![
        "exec name",
        "resume",
        "model",
        "myModel",
        "-ProViDeR",
        "myProvider",
    ]
    .into_iter()
    .map(|s| String::from(s))
    .collect::<Vec<_>>();

    let raw = RawArgs::from_args(args).expect("RawArgs should build successfully");

    println!("{:?}", raw);

    assert_matches!(raw.command(), Some(RunMode::Resume));
    assert_matches!(raw.model(), None);
    assert_eq!(
        raw.provider().expect("provider should be present"),
        "myProvider"
    );
}

#[test]
fn test_arg_parse4() {
    let args = vec![
        "exec name",
        "-new",
        "--MODEL",
        "myModel",
        "provider",
        "myProvider",
    ]
    .into_iter()
    .map(|s| String::from(s))
    .collect::<Vec<_>>();

    let raw = RawArgs::from_args(args).expect("RawArgs should build successfully");

    println!("{:?}", raw);

    assert_matches!(raw.command(), None);
    assert_matches!(raw.model(), None);
    assert_matches!(raw.provider(), None);
}
