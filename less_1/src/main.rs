use std::io::Write;

fn main() {
    println!("Hello, world!");
    let mut state = State::Menu;

    loop {
        let new_state = state.update();
        if let State::Exit = new_state {
            break;
        }

        state = new_state;
    }

    println!("Bye")
}

enum State {
    Menu,
    Game,
    Exit,
}

impl State {
    pub fn update(&self) -> Self {
        match self {
            Self::Menu => Self::run_menu(),
            Self::Game => Self::run_game(),
            Self::Exit => panic!("aaaaaaaa"),
        }
    }

    fn run_menu() -> State {
        println!("*** MENU ***");
        println!("1) start game");
        println!("2) Exit");
        let choice = Self::get_choice();
        match choice {
            Some(1) => State::Game,
            _ => State::Exit,
        }
    }

    fn get_choice() -> Option<u32> {
        std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.trim().parse().ok()
    }

    fn run_game() -> State {
        let number: u32 = rand::random();
        let number = number % 100;

        println!("");
        println!(" *** Game ***");

        loop {
            let choice = Self::get_choice();
            match choice {
                Some(x) if x < number => println!("too much small"),
                Some(x) if x > number => println!("too big small"),
                Some(x) if x == number => {
                    println!("You win");
                    return Self::Menu;
                }
                _ => return Self::Menu,
            }
        }
    }
}
