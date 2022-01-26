mod output;
mod types;

use alpm::{Alpm, AlpmList, Dep, Package, SigLevel};
use output::Output;
use types::{DepInstalled, Installed};

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

fn print_package_details(
    out: &mut Output,
    alpm: &Alpm,
    pkg: &PackageExtra
) -> std::io::Result<()> {
    print_title_line(out, pkg)?;

    if let Some(desc) = pkg.sync_pkg.desc() {
        out.println()?;
        out.print_description(desc)?;
    }

    out.println()?;
    if let Some(ip) = pkg.local_pkg {
        print_local_pkg_info(out, &pkg.sync_pkg, &ip)?;
    }

    print_dep_list(out, alpm, pkg.sync_pkg.optdepends(), "Opt Depends")?;
    print_dep_list(out, alpm, pkg.sync_pkg.depends(), "Depends")
}

fn print_dep_list(
    out: &mut Output,
    alpm: &Alpm,
    dep_list: AlpmList<Dep>,
    header: &str
) -> std::io::Result<()> {
    out.print_section_header(header)?;
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
        out.print_dependency(dep.name(), dep.version().map(|v| v.as_ref()), dep.desc().unwrap_or(""), dep_satisfied)?;
    }
    Ok(())
}

fn print_title_line(out: &mut Output, pkg: &PackageExtra) -> std::io::Result<()> {
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
    out.print_title(spkg.db().expect("found in a db").name(), spkg.name(), spkg.version(), installed)
}

fn print_local_pkg_info(
    out: &mut Output,
    sync_pkg: &Package,
    local_pkg: &Package
) -> std::io::Result<()> {

    if local_pkg.version() != sync_pkg.version() {
        out.print_installed_version(local_pkg.version())?;
    }
    let reason = match local_pkg.reason() {
        alpm::PackageReason::Depend => "dependency",
        alpm::PackageReason::Explicit => "explicit",
    };
    out.print_installed_reason(reason)
}
