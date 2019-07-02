extern crate rand;
extern crate termion;

use std::fmt::Write;
use std::io::{self, stdout, Read, Write as write2};
use std::{thread, time};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{async_stdin, clear, color, cursor};

#[derive(Debug, Clone)]
enum Element {
    RGB(u8, u8, u8, char), //r,g,b
    Empty,
}

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug)]
struct Size {
    width: u16,
    height: u16,
}

#[derive(Debug)]
struct Object {
    id: u32,
    data: Vec<Vec<Element>>,
    pos: Point,
    size: Size,
}

impl Object {
    fn new(id: u32, data: Vec<Vec<Element>>, pos: Point) -> Object {
        let width = data[0].len() as u16;
        let height = data.len() as u16;
        Object {
            id,
            data,
            pos,
            size: Size { width, height },
        }
    }
    fn rectangle(size: Size, color: Element, pos: Point, priority: u32) -> Object {
        let mut x = Vec::new();
        let mut v = Vec::new();
        for _i in 0..size.width {
            x.push(color.clone());
        }
        for _i in 0..size.height {
            v.push(x.clone())
        }
        Self::new(priority, v, pos)
    }
    fn draw(&self, w: &mut String) {
        write!(w, "{}{}", cursor::Hide, color::Bg(color::Reset)).unwrap();
        for row in 0..self.size.height {
            for col in 0..self.size.width {
                match self.data[row as usize][col as usize] {
                    Element::RGB(r, g, b, c) => {
                        write!(
                            w,
                            "{}{}{}",
                            cursor::Goto(row + self.pos.x + 1, col + self.pos.y + 1),
                            color::Bg(color::Rgb(r, g, b)),
                            c
                        )
                        .unwrap();
                    }
                    Element::Empty => {}
                }
            }
        }
    }
}

#[derive(Debug)]
struct World {
    objects: Vec<Object>,
}

impl World {
    fn add(&mut self, obj: Object) {
        self.objects.push(obj);
    }

    fn draw_all(&mut self) {
        let mut stdin = async_stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(
            stdout,
            "{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            cursor::Hide
        )
        .unwrap();
        stdout.flush().unwrap();

        loop {
            let mut outbuff = String::new();
            let mut b: [u8; 1] = [0];
            let haskey = stdin.read(&mut b).is_ok();
            // let update = stdin.read_exact(&mut b).is_ok();
            for obj in self.objects.iter_mut() {
                if obj.id == 1 {
                    if haskey {
                        Self::update(b[0], obj);
                    };
                }
                obj.draw(&mut outbuff);
            }
            println!("{}", outbuff);
            thread::sleep(time::Duration::from_millis(150));

            stdout.write(b"\n\r").unwrap();
            if haskey && b[0] == b'q' {
                break;
            };

        }
    }

    fn update(c: u8, obj: &mut Object) {
        match c {
            b'a' => obj.pos.x -= 1,
            b'd' => obj.pos.x += 1,
            b'w' => obj.pos.y -= 1,
            b's' => obj.pos.y += 1,
            _ => {}
        }
    }
}
fn main() {
    let obj1 = Object::rectangle(
        Size {
            width: 30,
            height: 100,
        },
        Element::RGB(100, 100, 100, ' '),
        Point { x: 0, y: 0 },
        0,
    );
    let obj2 = Object::rectangle(
        Size {
            width: 5,
            height: 5,
        },
        Element::RGB(1, 100, 100, '*'),
        Point { x: 10, y: 0 },
        1,
    );

    let mut world = World {
        objects: Vec::new(),
    };
    world.add(obj1);
    world.add(obj2);
    world.draw_all();
}
