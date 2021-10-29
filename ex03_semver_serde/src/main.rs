use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use semver::{Crate, Library, Program};

fn main() -> Result<(), anyhow::Error> {
    let sv = semver::SemVer::from_str("1.2.3")?;
    println!("parse successful! {:?}", sv);

    dbg!(format!("{:?}", sv));

    // exit successfully

    let program = Program {
        name: "hello_bin".to_string(),
        release_history: vec![],
    };

    let program_2 = Program {
        name: "hello_world".to_string(),
        release_history: vec![],
    };

    let library = Library {
        name: "hello_lib".to_string(),
        release_history: vec![],
    };

    let library_2 = Library {
        name: "moon_lib".to_string(),
        release_history: vec![],
    };

    //print_crate_names(&vec![program, program_2]);
    //print_crate_names(&vec![library, library_2]);

    let crates: Vec<Box<dyn Crate>> = vec![Box::new(program.clone()), Box::new(library)];
    for item in crates {
        println!("{}", item.name())
    }

    print_crate_name_dyn(&*Box::new(program));
    Ok(())
}

// dispatch on enum
enum Command {
    DoThis,
    DoThat,
}

trait CrateAndDebugAndMore: Crate + Debug {}

impl CrateAndDebugAndMore for Program {}
impl CrateAndDebugAndMore for Library {}

// invoke: print_crate_name_dyn(&*Box::new(program));
fn print_crate_name_dyn(crt: &dyn Crate) {
    println!("{}", crt.name())
}

fn print_crate_names<C: Crate>(crates: &Vec<C>) {
    for elem in crates {
        println!("{}", elem.name())
    }
}
