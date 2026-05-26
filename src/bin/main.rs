use calculation::{error::Result, parser::Parser};
use clap::{Arg, app_from_crate, crate_authors, crate_description, crate_name, crate_version};
use rustyline::{Editor, error::ReadlineError};
#[derive(Debug)]
pub struct Cli {
    debug: bool,
    prompt: Editor<(), rustyline::history::DefaultHistory>,
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl Cli {
    pub fn new() -> Self {
        Self {
            debug: false,
            prompt: Editor::new().expect("REASON"),
        }
    }

    fn evaluate(&mut self, input: &str) -> Result<Option<f64>> {
        match input {
            "exit" => std::process::exit(1),
            _ => {
                if !input.is_empty() {
                    let expr = Parser::new(input).parse()?;
                    println!("expr:{:?}", expr);
                    if self.debug {
                        println!("{:#?}", expr);
                    }
                    Ok(Some(expr.evaluate()))
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn prompt(&mut self) -> Result<Option<String>> {
        match self.prompt.readline("> ") {
            Ok(input) => Ok(Some(input)),
            Err(ReadlineError::Eof) => Ok(None),
            Err(ReadlineError::Interrupted) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    /// Runs the CLI application
    pub fn run(&mut self) -> Result<()> {
        let opts = app_from_crate!()
            .arg(
                Arg::with_name("debug")
                    .short("d")
                    .long("debug")
                    .help("Enables debug output"),
            )
            .get_matches();
        self.debug = opts.is_present("debug");

        while let Some(input) = self.prompt()? {
            println!("input:{}", input);
            match self.evaluate(&input) {
                Ok(Some(result)) => println!("{}", result),
                Err(err) => println!("Error: {}", err),
                Ok(None) => {}
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    Cli::new().run()
}
