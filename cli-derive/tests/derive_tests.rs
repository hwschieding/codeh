use std::assert_matches;

use cli_derive::*;

#[derive(ArgParsing)]
struct TestStruct {
    #[args(opt, alt = "f1", alt = "fld1")]
    field1: Option<String>,
    #[args(flag)]
    field2: bool,
}

#[derive(Debug, RunCommand)]
enum TestCommands {
    New,
    Resume,
}

#[test]
fn test_arg_parser() {
    let mut my_struct = TestStruct {
        field1: Some(String::new()),
        field2: false,
    };

    assert!(my_struct.set_opt("f1", String::from("hello")));
    assert!(my_struct.set_flag("field2"));
    assert!(!(my_struct.set_flag("fake_flag")));
    assert!(!(my_struct.set_opt("fake_opt", String::from("fake_val"))));

    assert!(TestStruct::is_opt("field1"));
    assert!(TestStruct::is_opt("f1"));
    assert!(TestStruct::is_opt("fld1"));
    assert!(TestStruct::is_flag("field2"));
    assert!(!(TestStruct::is_flag("field1")));
    assert!(!(TestStruct::is_opt("fake opt")));

    assert_eq!(
        my_struct.field1.expect("Should be some"),
        String::from("hello")
    );
    assert!(my_struct.field2);
}

#[test]
fn test_command() {
    assert_matches!(TestCommands::from_name("new"), Some(TestCommands::New));
    assert_matches!(
        TestCommands::from_name("resume"),
        Some(TestCommands::Resume)
    );
    assert_matches!(TestCommands::from_name("fake_command"), None);
}
