use std::{fmt::Debug, str::FromStr};
use std::{fmt::Display, num::ParseIntError};

#[derive(Default, Debug, PartialEq)]
struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

// assignment: implement this
impl FromStr for SemVer {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // remember filter_map
        todo!()
    }
}

fn main() -> Result<(), ParseIntError> {
    let sv: SemVer = SemVer::from_str("1.2.3")?;
    println!("parse successful! {:?}", sv);

    // the rest from today, found below
    other_stuff();

    // exit successfully
    Ok(())
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "SemVer {}.{}.{}",
            self.major, self.minor, self.patch
        ))
    }
}

impl SemVer {
    fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    fn new_simple(major: u16) -> Self {
        Self::new(major, u16::default(), u16::default())
    }
}

impl From<[u16; 3]> for SemVer {
    fn from(components: [u16; 3]) -> Self {
        Self::new(components[0], components[1], components[1])
    }
}

// questionable
impl From<[i32; 3]> for SemVer {
    fn from(components: [i32; 3]) -> Self {
        Self::new(
            components[0] as u16,
            components[1] as u16,
            components[1] as u16,
        )
    }
}

fn other_stuff() {
    let components = [1i32, 2, 3];
    let sv: SemVer = components.into();

    let mut default_semver = SemVer::default();
    default_semver.major = 2;

    // let default_semver = handle_semver(default_semver);
    //if semver_ref == default_semver {}

    // let a_number = 12;
    // drop(a_number);
    // print_number(a_number);
    // print_number(a_number);

    let mut semvers: Vec<SemVer> = vec![
        SemVer::new_simple(1),
        SemVer::default(),
        SemVer::new(1, 2, 3),
    ];

    for sv in &semvers {
        handle_semver(sv);
        handle_semver(sv)
    }

    for sv in &mut semvers {
        // let mut_ref = &mut *sv;
        increment_major(sv);
        // increment_major(mut_ref);
    }

    for sv in semvers {
        handle_semver(&sv);
    }

    let numbers = vec![1, 2, 30u8].into_iter();

    let cutoff = 5;
    let add_something = 10;

    // there is also filter_map
    let numbers = numbers
        .map(add_10)
        .map(|x| {
            // multi-line closure
            let y = x * 2;
            x + y + add_something
        })
        .filter(|x| x > &cutoff) // closures capture their environment
        .map(|x| format!("number: {}", x))
        .collect::<Vec<_>>();

    println!("numbers? {:?}", numbers);

    for component in "1.xx.3".split(".") {
        let maybe_number = component.parse::<u16>();
        println!("component: {:?}", maybe_number);
    }
}

fn add_10(x: u8) -> u8 {
    x + 10
}

fn handle_semver(sv: &SemVer) {
    println!("a SemVer: {:?}", sv);
}

fn increment_major(sv: &mut SemVer) {
    sv.major += 1;
    println!("changed a SemVer: {:?}", sv);
}

fn print_number(n: u32) {
    println!("a number: {}", n);
}
