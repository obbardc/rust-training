#[derive(PartialEq, Debug)]
enum FarmAnimal {
    Worm,
    Cow,
    Bull,
    Chicken { num_eggs: usize },
    Dog { name: String },
}

fn what_does_the_animal_say(animal: &FarmAnimal) {

    /* TODO: fill in the match statement below to make this code compile */

    /*
    Expected output:

    Dog { name: "Lassie" } says: "woof, woof! I am Lassie!"
    ​Cow says: "moo"
    ​Bull says: "moo"
    ​Chicken { num_eggs: 3 } says: "cluck, cluck!"
    ​Worm says: "-- (silence)
    */

    let noise = match animal {
        /* Cow and Bull */
        FarmAnimal::Cow | FarmAnimal::Bull => "moo".to_string(),

        /* Chicken */
        FarmAnimal::Chicken { num_eggs: _ } => "cluck, cluck!".to_string(),

        /* Dog */
        FarmAnimal::Dog { name } => match name.as_str() {
            "Lassie" => String::from("I am special and do not tell you my name."),
            _ => format!("woof, woof! I am {}!", name),
        },

        /* Worm– or all silent animals? */
        _ => "-- (silence)".to_string(),
    };

    /* Bonus task: Give Dogs named Lassie a different output */
    println!("{:?} says: {:?}", animal, noise);
}

fn main() {
    what_does_the_animal_say(
        &FarmAnimal::Dog {
            name: "Lassie".to_string()
    });
    what_does_the_animal_say(
        &FarmAnimal::Dog {
            name: "Babe".to_string()
    });
    what_does_the_animal_say(&FarmAnimal::Cow);
    what_does_the_animal_say(&FarmAnimal::Bull);
    what_does_the_animal_say(&FarmAnimal::Chicken{num_eggs: 3});
    what_does_the_animal_say(&FarmAnimal::Worm);
}
