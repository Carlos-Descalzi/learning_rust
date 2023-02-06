extern crate termion;

use std::io::{Write, Bytes};
use termion::AsyncReader;
use crate::game::consts::{SCREEN_WIDTH, SCREEN_HEIGHT};

#[derive(PartialEq, Clone)]
pub enum InputEvent {
    Quit,
    Up,
    Down,
    Left,
    Right,
    Fire
}

pub struct Terminal<'a> {
    buffer: [[char;SCREEN_WIDTH];SCREEN_HEIGHT],
    writer: &'a mut dyn Write,
    reader: &'a mut Bytes<AsyncReader>
}

impl<'a> Terminal<'a> {

    pub fn new(writer: &'a mut dyn Write, reader: &'a mut Bytes<AsyncReader>) -> Terminal<'a> {

        write!(writer,"{}",termion::cursor::Hide).unwrap();

        Terminal {
            buffer: [[' ' ;SCREEN_WIDTH];SCREEN_HEIGHT],
            writer,
            reader
        }
    }

    pub fn draw(&mut self, x: i32, y: i32, c:char){
        self.buffer[y as usize][x as usize] = c;
    }

    pub fn write(&mut self, x: i32, y: i32, s: &str){
        for (i,c) in s.chars().enumerate() {
            self.buffer[y as usize][x as usize +i] = c;
        }
    }

    pub fn clear(&mut self){
       for i in 0..SCREEN_WIDTH {
            for j in 0 ..SCREEN_HEIGHT {
                self.buffer[j as usize][i as usize] = ' ';
            }
        }
    }

    pub fn flush(&mut self) {
        for i in 0..SCREEN_HEIGHT {
            write!(
                self.writer,
                "{}{}{}{}",
                termion::cursor::Goto(1, i as u16 +1), 
                termion::color::Fg(termion::color::White),
                termion::color::Bg(termion::color::Blue),
                String::from_iter(self.buffer[i])
            ).unwrap();
        }
    }

    pub fn clear_screen(&mut self) {
        write!(self.writer, "{}", termion::clear::All).unwrap();
    }

    pub fn events(&mut self) -> Vec<InputEvent> {

        let mut events = vec![];
        let mut next = self.reader.next();

        while next.is_some() {

            match next {
                Some(Ok(b'w')) => { 
                    events.push(InputEvent::Up); 
                }
                Some(Ok(b's')) => {
                    events.push(InputEvent::Down);
                }
                Some(Ok(b'a')) => {
                    events.push(InputEvent::Left);
                }
                Some(Ok(b'd')) => {
                    events.push(InputEvent::Right);
                }
                Some(Ok(b'q')) => {
                    events.push(InputEvent::Quit);
                }
                Some(Ok(b' ')) => {
                    events.push(InputEvent::Fire);
                }
                _ => {
                }
            }

            next = self.reader.next();
        }

        events
    } 
}

impl<'a> Drop for Terminal<'a> {

    fn drop(& mut self) {

        self.clear_screen();

        write!(
            self.writer,
            "{}{}",
            termion::cursor::Goto(1,1),
            termion::cursor::Show
        ).unwrap();

    }

}