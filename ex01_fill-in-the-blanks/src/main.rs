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

    let noise = match animal {
        /* Cow and Bull */ => "moo".to_string(),
        /* Chicken      */ => "cluck, cluck!".to_string(),
        /* Dog          */  => format!("woof, woof! I am {}!", name),
        /* Worm– or all silent animals?*/ => "-- (silence)".to_string(),
    };

    /* Bonus task: Give Dogs named Lassie a different output */

    println!("{:?} says: {:?}", animal, noise);
}

fn main() {
    what_does_the_animal_say(
        &FarmAnimal::Dog {
            name: "Lassie".to_string()
    });
    what_does_the_animal_say(&FarmAnimal::Cow);
    what_does_the_animal_say(&FarmAnimal::Bull);
    what_does_the_animal_say(&FarmAnimal::Chicken{num_eggs: 3});
    what_does_the_animal_say(&FarmAnimal::Worm);
}
