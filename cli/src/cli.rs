use cli_derive::{ArgParsing, RunCommand};
use cli_error::CliError;

pub(crate) mod cli_error;

#[derive(Debug, RunCommand)]
pub enum RunMode {
    New,
    Resume,
}

#[derive(Debug, ArgParsing)]
pub struct RawArgs {
    command: Option<RunMode>,
    #[opt]
    model: Option<String>,
    #[opt]
    provider: Option<String>,
}

impl RawArgs {
    fn new() -> Self {
        Self {
            command: None,
            model: None,
            provider: None,
        }
    }

    fn set_command(&mut self, new: Option<RunMode>) -> () {
        self.command = new;
    }

    pub fn command(&self) -> Option<&RunMode> {
        self.command.as_ref()
    }

    pub fn model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    pub fn provider(&self) -> Option<&String> {
        self.provider.as_ref()
    }

    pub fn from_args(arg_vec: Vec<String>) -> Result<Self, CliError> {
        let mut res = Self::new();

        let mut arg_iter = arg_vec.into_iter();
        arg_iter.next();
        while let Some(arg) = arg_iter.next() {
            let arg = format_arg(arg);

            if let Some(r) = RunMode::from_name(&arg) {
                res.set_command(Some(r));
            }

            if Self::is_opt(&arg) {
                res.set_opt(
                    &arg,
                    arg_iter.next().ok_or(CliError::NoValue(format!(
                        "No value given for arg '{}'",
                        arg
                    )))?,
                );
            };

            if Self::is_flag(&arg) {
                res.set_flag(&arg);
            }
        }

        Ok(res)
    }
}

fn format_arg(arg: String) -> String {
    match arg.strip_prefix('-') {
        Some(a) => a.to_lowercase(),
        None => arg.to_lowercase(),
    }
}
