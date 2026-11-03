use cli_derive::{ArgParsing, RunCommand};
use cli_error::CliError;

use crate::cli_error;

#[derive(Debug, RunCommand)]
enum RunMode {
    New,
    Resume,
}

#[derive(Debug, ArgParsing)]
struct RawArgs {
    command: Option<RunMode>,
    #[opt]
    model: Option<String>,
    #[opt]
    provider: Option<String>,
}

impl RawArgs {
    fn from_args(arg_vec: Vec<String>) -> Result<Self, CliError> {
        todo!()
    }
}

fn strip_dashes(arg: &str) -> &str {
    arg.trim_start_matches('-')
}
