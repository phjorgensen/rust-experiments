enum Colour {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Colour {
    fn is_green(&self) -> bool {
        if let Colour::Green = self {
            return true;
        }

        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Colour::Red => return false,
            Colour::Green => return false,
            Colour::Blue => return true,
            Colour::Yellow => return true,
        }
    }
}

pub fn print_enum_values() {
    print_colour(Colour::Red);
    print_colour(Colour::Green);
    print_colour(Colour::Blue);
    print_colour(Colour::Yellow);
}

pub fn use_impl() {
    let foo = Colour::Green;

    let bar = foo.is_green();
    println!("{}", bar);

    let baz = foo.is_green_parts();
    println!("{}", baz);
}

fn print_colour(colour: Colour) {
    match colour {
        Colour::Red => println!("red"),
        Colour::Green => println!("green"),
        Colour::Blue => println!("blue"),
        Colour::Yellow => println!("yellow"),
    }
}
