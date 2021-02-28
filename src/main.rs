use alpm::{Alpm, AlpmList, Db, Dep, Package, SigLevel};

const ROOT_DIR: &'static str = "/";
const PKG_DB_DIR: &'static str = "/var/lib/pacman/";


fn main() {
    let mut args = std::env::args();
    args.next();
    let pkg_name = args.next().expect("no argument?");

    // setup alpm
    let alpm = Alpm::new(ROOT_DIR, PKG_DB_DIR).unwrap();
    alpm.set_arch("aarch64");
    alpm.register_syncdb("core", SigLevel::USE_DEFAULT).unwrap();
    alpm.register_syncdb("extra", SigLevel::USE_DEFAULT).unwrap(); 
    alpm.register_syncdb("community", SigLevel::USE_DEFAULT).unwrap();
    alpm.register_syncdb("alarm", SigLevel::USE_DEFAULT).unwrap();
   
    print_pkg_with_name(&pkg_name, &alpm);
}

fn print_pkg_with_name(pkg_name: &str, alpm: &Alpm) {
    let installed_pkg = alpm.localdb().pkg(pkg_name.clone());
    let db_list = alpm.syncdbs();
    for db in db_list {
        if let Ok(pkg) = db.pkg(pkg_name.clone()) {
            print_package_details(alpm, &db, &pkg, &installed_pkg.ok());
            break;
        }
    }
}

fn print_dep_list(alpm: &Alpm, dep_list: AlpmList<Dep>) {
    for dep in dep_list {
        print!("    {}", dep.name());
        if let Some(ver) = dep.version() {
            print!(" {}", ver);
        }
        if !dep.desc().is_empty() {
            print!(": {}", dep.desc());
        }
        let ip = alpm.localdb().pkgs().find_satisfier(dep.name());
        if let Some(p) = ip {
            if p.name() == dep.name() {
                print!(" [installed]");
            } else {
                print!(" [satisfied by {}]", p.name());
            }
        }
        println!();
    }
}

fn print_package_details(alpm: &Alpm, db: &Db, pkg: &Package, installed_pkg: &Option<Package>) {
    print!("{}/{} {}", db.name(), pkg.name(), pkg.version());
    if let Some(ip) = installed_pkg {
        if ip.version() != pkg.version() {
            print!(" [~installed]");
        } else {
            print!(" [installed]");
        }
    }
    println!();
    if let Some(desc) = pkg.desc() {
        println!();
        println!("{}", desc);
    }
    println!();
    if let Some(ip) = installed_pkg {
        if ip.version() != pkg.version() {
            println!("Installed Version: {}", ip.version());
        }
        let reason = match ip.reason() {
            alpm::PackageReason::Depend => "dependency",
            alpm::PackageReason::Explicit => "explicit"
        };
        println!("Installed Reason: {}", reason);
    }
    println!("Opt Depends:");
    print_dep_list(alpm, pkg.optdepends());
    println!("Depends:");
    print_dep_list(alpm, pkg.depends());
}
