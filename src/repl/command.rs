use super::*;

type CommandAction<T,Data> = for<'data> fn(
	Repl<'data, ManualPrint, T,Data>,
	&str,
) -> Result<Repl<'data, Print, T,Data>, ()>;

/// A command definition.
pub struct Command<Term: Terminal,Data> {
	/// The command name.
	pub name: &'static str,
	/// Arguments expected type.
	pub arg_type: CmdArgs,
	/// Help string.
	pub help: &'static str,
	/// Action to take.
	pub action: CommandAction<Term,Data>,
}

/// Command arguments variants.
#[derive(Clone, PartialEq, Debug)]
pub enum CmdArgs {
	/// No arguments.
	None,
	/// Command accepts a local filename.
	Filename,
	/// Optional unprocessed text may be accepted.
	Text,
	/// A Rust expression.
	Expr,
}

impl<Term: Terminal,Data> Command<Term,Data> {
	/// Create a new `Command`.
	pub fn new(
		name: &'static str,
		arg_type: CmdArgs,
		help: &'static str,
		action: CommandAction<Term,Data>,
	) -> Self {
		Command {
			name: name,
			arg_type: arg_type,
			help: help,
			action: action,
		}
	}
}

impl<Term: Terminal,Data> Clone for Command<Term,Data> {
	fn clone(&self) -> Self {
		Command {
			name: self.name,
			arg_type: self.arg_type.clone(),
			help: self.help,
			action: self.action,
		}
	}
}

pub trait Commands<Term: Terminal,Data> {
	/// Builds the help string of the commands.
	fn build_help_response(&self, command: Option<&str>) -> String;
	/// Conveniance function to lookup the Commands and return if found.
	fn find_command(&self, command: &str) -> Result<Command<Term,Data>, String>;
}

impl<Term: Terminal,Data> Commands<Term,Data> for Vec<Command<Term,Data>> {
	fn build_help_response(&self, command: Option<&str>) -> String {
		let mut ret = String::new();

		let write_cmd_line = |cmd: &Command<Term,Data>, str_builder: &mut String| {
			str_builder.push_str(&cmd.name);

			match cmd.arg_type {
				CmdArgs::None => (),
				CmdArgs::Filename => str_builder.push_str(" <filename>"),
				CmdArgs::Text => str_builder.push_str(" [text]"),
				CmdArgs::Expr => str_builder.push_str(" <expr>"),
			}
			str_builder.push(' ');
			str_builder.push_str(cmd.help);
			str_builder.push('\n');
		};

		if let Some(cmd) = command {
			match self.find_command(cmd) {
				Err(e) => ret.push_str(&e),
				Ok(cmd) => write_cmd_line(&cmd, &mut ret),
			}
		} else {
			ret.push_str("Available commands:\n");
			self.iter().for_each(|cmd| write_cmd_line(cmd, &mut ret));
		}
		ret
	}

	fn find_command(&self, command: &str) -> Result<Command<Term,Data>, String> {
		match self.iter().find(|c| c.name == command) {
			None => Err(format!("unrecognized command: {}", command)),
			Some(cmd) => Ok(cmd.clone()),
		}
	}
}
