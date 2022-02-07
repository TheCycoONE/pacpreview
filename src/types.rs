#[derive(Clone, Copy)]
pub enum Installed {
    Installed,
    Outdated,
    NotInstalled,
}

#[derive(Clone, Copy)]
pub enum DepInstalled<'a> {
    Installed,
    SatisfiedBy(&'a str),
    NotSatisfied,
}
