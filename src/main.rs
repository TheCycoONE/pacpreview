use alpm::{Alpm, AlpmList, Dep, Package, SigLevel};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;
use textwrap::{termwidth};
use std::env;

pub struct Output {
    width: usize,
    out: StandardStream
}
    
fn out_width() -> usize {
    if let Some(cols) = env::var("FZF_PREVIEW_COLUMNS").ok().and_then(|c| c.parse::<usize>().ok()) {
        cols
    } else {
        termwidth()
    }
}

impl Output {
    pub fn new() -> Self {
        Self {
            width: out_width(),
            out: StandardStream::stdout(ColorChoice::Auto)
        }
    }

    fn print_installed(&mut self) -> std::io::Result<()> {
        self.out.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(&mut self.out, "[installed]")?;
        self.out.reset()
    }

    fn print_outdated(&mut self) -> std::io::Result<()> {
        self.out.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        write!(&mut self.out, "[~installed]")?;
        self.out.reset()
    }

    fn print_satisifed_by(&mut self, pkg_name: &str) -> std::io::Result<()> {
        self.out.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(&mut self.out, "[satisfied by {}]", pkg_name)?;
        self.out.reset()
    }

    pub fn println(&mut self) -> std::io::Result<()> {
        writeln!(&mut self.out)
    }

    pub fn print_title(&mut self, database: &str, pkg_name: &str, version: &str, installed: Installed) -> std::io::Result<()> {
        write!(self.out, "{}/{} {}", database, pkg_name, version)?;
        write!(self.out, " ")?;
        match installed {
            Installed::Installed => self.print_installed(),
            Installed::Outdated => self.print_outdated(),
            Installed::NotInstalled => Ok(())
        }
    }

    pub fn print_description(&mut self, desc: &str) -> std::io::Result<()> {
        let wrap_opts = textwrap::Options::new(self.width);
        writeln!(&mut self.out, "{}", textwrap::fill(desc, wrap_opts))
    }

    pub fn print_dependency(&mut self, pkg_name: &str, version: Option<&str>, desc: &str, satisfied: DepInstalled) -> std::io::Result<()> {
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
            DepInstalled::NotSatisfied => Ok(())
        }?;
        self.println()
    }
}

pub enum Installed {
    Installed,
    Outdated,
    NotInstalled
}

pub enum DepInstalled<'a> {
    Installed,
    SatisfiedBy(&'a str),
    NotSatisfied
}

struct PackageExtra<'alpm> {
    sync_pkg: Package<'alpm>,
    local_pkg: Option<Package<'alpm>>
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let pkg_name = args.next().expect("no argument?");

    let pacman = pacmanconf::Config::new().unwrap();

    // setup alpm
    let alpm = Alpm::new(pacman.root_dir, pacman.db_path).unwrap();

    for repo in pacman.repos {
        alpm.register_syncdb(repo.name, SigLevel::USE_DEFAULT)
            .unwrap();
    }

    let mut out = Output::new();
    let pkg = find_pkg_with_name(&pkg_name, &alpm);
    if pkg.is_none() {
        eprintln!("No package found");
        std::process::exit(2);
    }
    let res = print_package_details(&mut out, &alpm, &pkg.expect("tested above"));
    if let Err(e) = res {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn find_pkg_with_name<'name, 'alpm>(
    pkg_name: &'name str,
    alpm: &'alpm Alpm,
) -> Option<PackageExtra<'alpm>> {
    
    let installed_pkg = alpm.localdb().pkg(pkg_name).ok();
    let db_list = alpm.syncdbs();
    for db in db_list {
        if let Ok(pkg) = db.pkg(pkg_name) {
            return Some(PackageExtra { local_pkg: installed_pkg, sync_pkg: pkg });
        }
    }
    None
}

fn print_dep_list(
    out: &mut Output,
    alpm: &Alpm,
    dep_list: AlpmList<Dep>,
) -> std::io::Result<()> {
    for dep in dep_list {
        let ip = alpm.localdb().pkgs().find_satisfier(dep.to_string());
        let dep_satisfied = if let Some(p) = ip {
            if p.name() == dep.name() {
                DepInstalled::Installed
            } else {
                DepInstalled::SatisfiedBy(p.name())
            }
        } else {
            DepInstalled::NotSatisfied
        };
        out.print_dependency(dep.name(), dep.version().and_then(|v| Some(v.as_ref())), dep.desc(), dep_satisfied)?;
    }
    Ok(())
}

fn print_package_details(
    out: &mut Output,
    alpm: &Alpm,
    pkg: &PackageExtra
) -> std::io::Result<()> {
    let installed = if let Some(ip) = pkg.local_pkg {
        if ip.version() != pkg.sync_pkg.version() {
            Installed::Outdated
        } else {
            Installed::Installed
        }
    } else {
        Installed::NotInstalled
    };

    let spkg = pkg.sync_pkg;
    out.print_title(spkg.db().expect("found in a db").name(), spkg.name(), spkg.version(), installed)?;
    out.println()?;
    if let Some(desc) = spkg.desc() {
        out.println()?;
        out.print_description(desc)?;
    }
    out.println()?;

    if let Some(ip) = pkg.local_pkg {
        if ip.version() != spkg.version() {
            writeln!(&mut out.out, "Installed Version: {}", ip.version())?;
        }
        let reason = match ip.reason() {
            alpm::PackageReason::Depend => "dependency",
            alpm::PackageReason::Explicit => "explicit",
        };
        writeln!(&mut out.out, "Installed Reason: {}", reason)?;
    }
    writeln!(&mut out.out, "Opt Depends:")?;
    print_dep_list(out, alpm, spkg.optdepends())?;
    writeln!(&mut out.out, "Depends:")?;
    print_dep_list(out, alpm, spkg.depends())
}
