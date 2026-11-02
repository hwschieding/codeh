use std::assert_matches;

use cli_derive::*;

#[derive(ArgParsing)]
struct TestStruct {
    #[opt]
    field1: String,
    #[flag]
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
        field1: String::new(),
        field2: false,
    };

    assert!(my_struct.set_opt("field1", String::from("hello")));
    assert!(my_struct.set_flag("field2"));
    assert!(!(my_struct.set_flag("fake_flag")));
    assert!(!(my_struct.set_opt("fake_opt", String::from("fake_val"))));

    assert!(my_struct.is_opt("field1"));
    assert!(my_struct.is_flag("field2"));
    assert!(!(my_struct.is_flag("field1")));
    assert!(!(my_struct.is_opt("fake opt")));

    assert_eq!(my_struct.field1, String::from("hello"));
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
