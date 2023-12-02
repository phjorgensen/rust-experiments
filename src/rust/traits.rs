mod shapes;

use std::{str::FromStr, fmt::Display};

use crate::{
    rust::traits::shapes::collisions::Collidable,
    traits::shapes::{area::Area, circle::Circle, rect::Rect},
};
use anyhow::Result;

use self::shapes::collisions::{Contains, Points};

enum Shape {
    Circle(Circle),
    Rect(Rect),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").unwrap_or(("", ""));

        match shape {
            "rect" => return Ok(Shape::Rect(data.parse()?)),
            "circle" => return Ok(Shape::Circle(data.parse()?)),
            _ => return Err(anyhow::anyhow!("Bad shape")),
        }
    }
}

impl Points for &Shape {
    fn points(&self) -> shapes::collisions::PointIter {
        match self {
            Shape::Circle(c) => return c.points(),
            Shape::Rect(r) => return r.points(),
        }
    }
}

impl Contains for &Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Circle(c) => return c.contains_point(point),
            Shape::Rect(r) => return r.contains_point(point),
        }
    }
}

impl Display for &Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Circle(c) => return write!(f, "{}", c),
            Shape::Rect(r) => return write!(f, "{}", r),
        }
    }
}

pub fn main() -> Result<()> {
    let rect = Rect {
        x: 10.0,
        y: 10.0,
        width: 10.0,
        height: 10.0,
    };
    let rect2 = Rect::default();

    let circ = Circle {
        x: 10.0,
        y: 10.0,
        radius: 10.0,
    };

    println!("circle: {}", circ.area());
    println!("rectrangle: {}", rect.area());
    println!("rectrangle 2: {}", rect2.area());
    println!("{}", rect);
    rect.to_string();
    println!("f64: {}", 6.4.area());
    println!("{}", rect);

    let first_rect = Rect::default();
    let second_rect = Rect::default();

    let first_circle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 1.0,
    };

    let second_circle = Circle {
        x: 1.5,
        y: 1.5,
        radius: 4.0,
    };

    println!("{}", first_rect.collide(&second_rect));
    println!("{}", first_rect.collide(&second_circle));
    println!("{}", first_circle.collide(&second_circle));
    println!("{}", first_circle.collide(&second_rect));

    let shapes = std::fs::read_to_string("shapes")?
        .lines()
        .filter_map(|x| x.parse::<Shape>().ok())
        .collect::<Vec<_>>();

    shapes
        .iter()
        .skip(1)
        .zip(shapes.iter().take(shapes.len() - 1))
        .filter(|(a, b)| a.collide(b))
        .for_each(|(a, b)| {
            println!("{} collides with {}", a, b);
        });



    return Ok(());
}
