//! Functions for writing package information to the console

use crate::types::{DepInstalled, Installed};
use std::env;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use textwrap::termwidth;

pub struct Output {
    width: usize,
    out: StandardStream,
}

fn out_width() -> usize {
    if let Some(cols) = env::var("FZF_PREVIEW_COLUMNS")
        .ok()
        .and_then(|c| c.parse::<usize>().ok())
    {
        cols
    } else {
        termwidth()
    }
}

impl Output {
    pub fn new() -> Self {
        Self {
            width: out_width(),
            out: StandardStream::stdout(ColorChoice::Auto),
        }
    }

    fn print_installed(&mut self) -> std::io::Result<()> {
        self.out
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(&mut self.out, "[installed]")?;
        self.out.reset()
    }

    fn print_outdated(&mut self) -> std::io::Result<()> {
        self.out
            .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        write!(&mut self.out, "[~installed]")?;
        self.out.reset()
    }

    fn print_satisifed_by(&mut self, pkg_name: &str) -> std::io::Result<()> {
        self.out
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(&mut self.out, "[satisfied by {}]", pkg_name)?;
        self.out.reset()
    }

    pub fn println(&mut self) -> std::io::Result<()> {
        writeln!(&mut self.out)
    }

    pub fn print_title(
        &mut self,
        database: &str,
        pkg_name: &str,
        version: &str,
        installed: Installed,
    ) -> std::io::Result<()> {
        write!(self.out, "{}/{} {}", database, pkg_name, version)?;
        write!(self.out, " ")?;
        match installed {
            Installed::Installed => self.print_installed(),
            Installed::Outdated => self.print_outdated(),
            Installed::NotInstalled => Ok(()),
        }?;
        writeln!(self.out)
    }

    pub fn print_description(&mut self, desc: &str) -> std::io::Result<()> {
        let wrap_opts = textwrap::Options::new(self.width);
        writeln!(&mut self.out, "{}", textwrap::fill(desc, wrap_opts))
    }

    pub fn print_installed_version(&mut self, ver: &str) -> std::io::Result<()> {
        writeln!(&mut self.out, "Installed Version: {}", ver)
    }

    pub fn print_installed_reason(&mut self, reason: &str) -> std::io::Result<()> {
        writeln!(&mut self.out, "Installed Reason: {}", reason)
    }

    pub fn print_section_header(&mut self, header: &str) -> std::io::Result<()> {
        writeln!(&mut self.out, "{}:", header)
    }

    pub fn print_dependency(
        &mut self,
        pkg_name: &str,
        version: Option<&str>,
        desc: &str,
        satisfied: DepInstalled,
    ) -> std::io::Result<()> {
        write!(&mut self.out, "  {}", pkg_name)?;
        if let Some(ver) = version {
            write!(&mut self.out, " {}", ver)?;
        }
        if !desc.is_empty() {
            write!(&mut self.out, ": {}", desc)?;
        }
        write!(&mut self.out, " ")?;
        match satisfied {
            DepInstalled::Installed => self.print_installed(),
            DepInstalled::SatisfiedBy(x) => self.print_satisifed_by(x),
            DepInstalled::NotSatisfied => Ok(()),
        }?;
        self.println()
    }
}
