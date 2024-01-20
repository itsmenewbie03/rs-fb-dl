use rand::{self, seq::SliceRandom};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Rizz {
    rizz: String,
}

pub fn get_rizz() -> Rizz {
    let rizz_vec: Vec<&str> = vec![
        "I'm like a dereferenced value: null and void without you.",
        "Your smile is like a clean build: flawless and functional. ✨",
        "You make me want to document all your beautiful traits. ",
        "My love for you is like statically safe code: secure and reliable. ",
        "Is your error handler broken? You keep crashing into my heart. ",
        "You're so hot, I need to run memory leak diagnostics after seeing you. ",
        "You're the `#[derive(soulmate)]` to my heart. Perfectly matched. ",
        "I want to be the exclusive reference to your happiness, always there for you. ",
    ];
    let mut rng = rand::thread_rng();
    let rizz = rizz_vec.choose(&mut rng).unwrap();
    Rizz {
        rizz: rizz.to_string(),
    }
}
