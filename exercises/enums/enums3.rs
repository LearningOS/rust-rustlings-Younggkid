// enums3.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a hint.

//This exercise has several points:
//1.  The definition of enum must point out the explict types, for example, the ChangeColor((u8, u8, u8))
//can not use the color behind, the color has to follow the enum
//2.   In the impl block, if wants to call other functions which are also
// in the impl block, try self.fun()

enum Message {
    ChangeColor((u8, u8, u8)),
    Echo(String),
    Move(Point),
    Quit,
    // TODO: implement the message variant types based on their usage below
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message{
            Message::ChangeColor(color) => self.change_color(color),
            Message::Echo(str) =>  self.echo(str),
            Message::Move(p) => self.move_position(p),
            Message::Quit => self.quit(),
        }
        // TODO: create a match expression to process the different message variants
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
        };
        state.process(Message::ChangeColor((255, 0, 255)));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }
}
