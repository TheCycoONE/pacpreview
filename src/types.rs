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
