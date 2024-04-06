//! [The Bridge of Death](https://www.youtube.com/watch?v=RbTaP0_Galg)
//#![allow(dead_code)]

mod bridge_of_death {
    enum Outcome {
        Pass,
        Peril,
    }

    struct Last();

    impl Last {
        #[allow(dead_code)]
        const PROMPT: &str = "What is the average flight speed of an unladen swallow?";

        // const PROMPT: &str = "What is your quest?";
        // const PROMPT: &str = "What is the capital of Assyria?";
        // const PROMPT: &str = "What is your favorite color?";

        #[allow(dead_code)]
        pub fn answer(answer: &str) -> Outcome {
            match answer {
                "Assur" => Outcome::Pass,
                _ => Outcome::Peril, // "I don't know that!"
            }
        }
    }

    // The module-private member prevents First from being constructed directly
    // outside this module.  The only way callers can get an instance of this
    // type is by calling one of this module's public functions.  TODO: Blog
    // post comparing this with C++, where you're allowed to leak private types?
    pub struct First(());

    impl First {
        #[allow(dead_code)]
        pub fn prompt() -> &'static str {
            "What is your name?"
        }

        // fn answer()
    }

    pub fn init() -> First {
        First(())
    }
}

fn main() {
    // let _first = bridge_of_death::First();
    let _state = bridge_of_death::init();
    println!("Hello, world!");
}
