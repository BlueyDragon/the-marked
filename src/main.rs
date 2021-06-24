// Imports; equivalent to include, import, using, etc.
use rltk::{Rltk, GameState};

// Structs are classes; they hold data and can have methods
struct State{}

// This makes the State struct implement the GameState trait.
// Traits are like interfaces or base classes; they set up
// a structure for another author to implement in their own code
// which can then interact with the library that provides them.
impl GameState for State{
    // Function definition. This one has to match the type
    // required by the trait. Doesn't end with a type, so it
    // does not return data (void). &mut self means it requires
    // access to the parent structure and may change it (mutable).
    // & means pass a reference - a pointer to an existing copy
    // of the variable. The variable isn't copied in this instance;
    // if any changes are made, the original is changed.
    fn tick(&mut self, context : &mut Rltk){
        // Clear screen
        context.cls();

        // Print to the console at location (1,1)
        context.print(1, 1, "Hello Rust World!");
    }
}

// Every program has a main function; it tells the operating system
// where to start the program.
fn main() -> rltk::BError{
    // Calling a function from inside a struct where that struct
    // doesn't take a self function. This is basically a constructor
    // in other languages. Create a terminal eighty characters wide
    // by fifty characters high and name the window Roguelike Tutorial
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Rust Roguelike Tutorial")
        .build()?;

    // Variable assignment - variable named gs that is set to be a
    // copy of the State struct defined above.
    let gs = State{};

    // Run the main_loop function in the rltk namespace and pass in
    // the context and gamestate variables we created earlier.
    rltk::main_loop(context, gs)
}
