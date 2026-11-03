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
    #[args(opt, alt = "m")]
    model: Option<String>,
    #[args(opt, alt = "p")]
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
            match ArgType::check(&arg) {
                Some(ArgType::Command) => {
                    if let Some(r) = RunMode::from_name(&arg) {
                        res.set_command(Some(r));
                    }
                }
                Some(ArgType::Option) => {
                    res.try_opt_or_flag(&format_arg(arg), &mut arg_iter)?;
                }
                None => (),
            }
        }

        Ok(res)
    }

    fn try_opt_or_flag(
        &mut self,
        arg: &str,
        arg_iter: &mut impl Iterator<Item = String>,
    ) -> Result<(), CliError> {
        if Self::is_opt(arg) {
            if !(self.set_opt(
                arg,
                arg_iter.next().ok_or(CliError::NoValue(format!(
                    "No value given for arg '{}'",
                    arg
                )))?,
            )) {
                return Err(CliError::BadParse("Given option could not be parsed"));
            }
        }
        if Self::is_flag(arg) {
            if !(self.set_flag(arg)) {
                return Err(CliError::BadParse("Given flag could not be parsed"));
            }
        }
        Ok(())
    }
}

enum ArgType {
    Command,
    Option,
}

impl ArgType {
    fn check(arg: &str) -> Option<Self> {
        match arg.chars().nth(0) {
            Some('-') => Some(Self::Option),
            Some(_) => Some(Self::Command),
            None => None,
        }
    }
}

fn format_arg(arg: String) -> String {
    match arg.strip_prefix('-') {
        Some(a) => a.to_lowercase(),
        None => arg.to_lowercase(),
    }
}
