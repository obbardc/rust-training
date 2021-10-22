use std::{
    convert::{TryFrom, TryInto},
    error::Error,
    fmt::Debug,
    fs::File,
    str::FromStr,
    thread::panicking,
};
use std::{fmt::Display, num::ParseIntError};

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

// #[derive(Debug)]
// pub enum ParseError {
//     Underlying(ParseIntError),
//     WrongNumberOfComponents(usize),
// }

// impl std::fmt::Display for ParseError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ParseError::Underlying(_e) => f.write_str("unexpected content"),
//             ParseError::WrongNumberOfComponents(n) => {
//                 write!(f, "expected 3 components, got {}", n)
//             }
//         }
//     }
// }

// impl std::error::Error for ParseError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match self {
//             ParseError::Underlying(e) => Some(e),
//             ParseError::WrongNumberOfComponents(_) => None,
//         }
//     }
// }

// impl FromStr for SemVer {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let components_vec: Vec<u16> = s.split(".").filter_map(|s| s.parse().ok()).collect();
//         let components_arr: [u16; 3] = components_vec
//             .try_into()
//             .map_err(|e: Vec<u16>| format!("WrongNumberOfComponents: {}", e.len()))?;

//         Ok(SemVer::from(components_arr))
//     }
// }

impl FromStr for SemVer {
    //type Err = ParseError;
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Result<Vec<u16>, _> =
            s.split(".").map(|item| item.parse::<u16>()).collect();

        // let components = components.map_err(|e| ParseError::Underlying(e));

        // either
        let components_vec = components?;

        // or
        // let components_vec = match components {
        //     Ok(v) => v,
        //     Err(e) => return Err(e),
        // };

        // let arr = <[u16; 3] as TryFrom<Vec<u16>>>::try_from(components_vec);

        // either
        // let components_arr: [u16; 3] = components_vec
        //     .try_into()
        //     .map_err(|e: Vec<u16>| Self::Err::WrongNumberOfComponents(e.len()))?;

        // or
        // let components_arr: [u16; 3] = match components_vec.try_into() {
        //     Ok(arr) => arr,
        //     Err(e) => return Err(Self::Err::WrongNumberOfComponents(e.len())),
        // };

        // let silly = File::open("/tmp")?;

        let components_arr: [u16; 3] = components_vec
            .try_into()
            .map_err(|_| anyhow::anyhow!("wrong number of components"))?;
        //     .try_into()
        //     .map_err(|e: Vec<u16>| Self::Err::WrongNumberOfComponents(e.len()))?;

        Ok(SemVer::from(components_arr))

        // Ok(components_arr
        //     .map_err(|e| Self::Err::WrongNumberOfComponents(e.len()))?
        //     .into())
    }
}

impl From<[u16; 3]> for SemVer {
    fn from(components: [u16; 3]) -> Self {
        Self::new(components[0], components[1], components[2])
    }
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

pub trait Crate {
    fn name(&self) -> &str;
    fn release_history(&self) -> &Vec<SemVer>;
}

#[derive(Debug, Clone)]
pub struct Program {
    pub name: String,
    pub release_history: Vec<SemVer>,
}

#[derive(Debug, Clone)]
pub struct Library {
    pub name: String,
    pub release_history: Vec<SemVer>,
}

impl Crate for Library {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn release_history(&self) -> &Vec<SemVer> {
        &self.release_history
    }
}

impl Crate for Program {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn release_history(&self) -> &Vec<SemVer> {
        &self.release_history
    }
}

#[derive(Debug, Clone)]
pub struct CrateData {
    pub name: String,
    pub release_history: Vec<SemVer>,
}

impl Crate for CrateData {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn release_history(&self) -> &Vec<SemVer> {
        &self.release_history
    }
}

enum CrateEnum {
    Program(CrateData),
    Library(CrateData),
}

impl Crate for CrateEnum {
    fn name(&self) -> &str {
        match self {
            CrateEnum::Program(p) => p.name(),
            CrateEnum::Library(l) => l.name(),
        }
    }

    fn release_history(&self) -> &Vec<SemVer> {
        todo!()
    }
}

struct Repository {
    crates: Vec<CrateEnum>,
}

struct DynRepository {
    crates: Vec<Box<dyn Crate>>,
}

// questionable
impl From<[i32; 3]> for SemVer {
    fn from(components: [i32; 3]) -> Self {
        Self::new(
            components[0] as u16,
            components[1] as u16,
            components[2] as u16,
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

mod tests {}

#[test]
#[should_panic]
fn test_semver_parse_garbage() {
    SemVer::from_str("1.2.x.3.x").unwrap();
}

#[test]
#[should_panic]
fn test_semver_parse_missing_patch() {
    SemVer::from_str("1.2").unwrap();
}

#[test]
fn test_semver_parse_ok() {
    match SemVer::from_str("1.2.3") {
        Ok(sv) => assert_eq!(sv, SemVer::new(1, 2, 3)),
        Err(e) => panic!("unexpected error: {:?}", e),
    }
}

// #[test]
// fn test_semver_parse_ok_2() -> Result<(), ParseError> {
//     SemVer::from_str("1.2.3")?;
//     Ok(())
// }
