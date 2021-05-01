use alpm::{Alpm, AlpmList, Db, Dep, Package, SigLevel};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use textwrap::{termwidth};
use std::env;

fn print_installed(out: &mut impl WriteColor) -> std::io::Result<()> {
    out.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(out, "[installed]")?;
    out.reset()?;
    Ok(())
}

fn print_outdated(out: &mut impl WriteColor) -> std::io::Result<()> {
    out.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    write!(out, "[~installed]")?;
    out.reset()?;
    Ok(())
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

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let res = print_pkg_with_name(&mut stdout, &pkg_name, &alpm);
    if let Err(e) = res {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn out_width() -> usize {
    if let Some(cols) = env::var("FZF_PREVIEW_COLUMNS").ok().and_then(|c| c.parse::<usize>().ok()) {
        cols
    } else {
        termwidth()
    }
}

fn print_pkg_with_name(
    out: &mut impl WriteColor,
    pkg_name: &str,
    alpm: &Alpm,
) -> std::io::Result<()> {
    let installed_pkg = alpm.localdb().pkg(pkg_name);
    let db_list = alpm.syncdbs();
    for db in db_list {
        if let Ok(pkg) = db.pkg(pkg_name) {
            print_package_details(out, alpm, &db, &pkg, &installed_pkg.ok())?;
            break;
        }
    }
    Ok(())
}

fn print_dep_list(
    out: &mut impl WriteColor,
    alpm: &Alpm,
    dep_list: AlpmList<Dep>,
) -> std::io::Result<()> {
    for dep in dep_list {
        write!(out, "    {}", dep.name())?;
        if let Some(ver) = dep.version() {
            write!(out, " {}", ver)?;
        }
        if !dep.desc().is_empty() {
            write!(out, ": {}", dep.desc())?;
        }
        let ip = alpm.localdb().pkgs().find_satisfier(dep.to_string());
        if let Some(p) = ip {
            write!(out, " ")?;
            if p.name() == dep.name() {
                print_installed(out)?;
            } else {
                write!(out, " [satisfied by {}]", p.name())?;
            }
        }
        writeln!(out)?;
    }
    Ok(())
}

fn print_package_details(
    out: &mut impl WriteColor,
    alpm: &Alpm,
    db: &Db,
    pkg: &Package,
    installed_pkg: &Option<Package>,
) -> std::io::Result<()> {
    write!(out, "{}/{} {}", db.name(), pkg.name(), pkg.version())?;
    if let Some(ip) = installed_pkg {
        write!(out, " ")?;
        if ip.version() != pkg.version() {
            print_outdated(out)?;
        } else {
            print_installed(out)?;
        }
    }
    writeln!(out)?;
    if let Some(desc) = pkg.desc() {
        let wrap_opts = textwrap::Options::new(out_width());

        writeln!(out)?;
        writeln!(out, "{}", textwrap::fill(desc, wrap_opts))?;
    }
    writeln!(out)?;
    if let Some(ip) = installed_pkg {
        if ip.version() != pkg.version() {
            writeln!(out, "Installed Version: {}", ip.version())?;
        }
        let reason = match ip.reason() {
            alpm::PackageReason::Depend => "dependency",
            alpm::PackageReason::Explicit => "explicit",
        };
        writeln!(out, "Installed Reason: {}", reason)?;
    }
    writeln!(out, "Opt Depends:")?;
    print_dep_list(out, alpm, pkg.optdepends())?;
    writeln!(out, "Depends:")?;
    print_dep_list(out, alpm, pkg.depends())?;
    Ok(())
}
