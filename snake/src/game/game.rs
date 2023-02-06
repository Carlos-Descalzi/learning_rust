
use rand::prelude::*;
use crate::game::types::Point;
use crate::game::consts::*;
use crate::game::term::*;
use std::time::Duration;

#[derive(PartialEq)]
enum StateResult {
    Ok,
    End,
    Exit
}

trait GameState {
    fn init(&mut self);
    fn run(&mut self, terminal: &mut Terminal) -> StateResult;
    fn render(&self, terminal: &mut Terminal);
}


pub struct GamePlay{
    score: i32,
    body: Vec<Point>,
    fruits: Vec<Point>,
    extend: i32,
    direction: Point,
    lives: i32,
}

impl GamePlay{

    fn new() -> GamePlay {

        GamePlay {
            score: 0,
            extend: 0,
            fruits: vec![],
            body: vec![],
            direction: Point::RIGHT,
            lives: 3
        }
    }

    fn handle_events(&mut self, terminal: &mut Terminal) -> StateResult {

        let mut new_direction = self.direction;

        for event in terminal.events(){
            match event {
                InputEvent::Quit => {
                    return StateResult::Exit;
                }
                InputEvent::Up => {
                    new_direction = Point::UP;
                }
                InputEvent::Down => {
                    new_direction = Point::DOWN;
                }
                InputEvent::Left => {
                    new_direction = Point::LEFT;
                }
                InputEvent::Right => {
                    new_direction = Point::RIGHT;
                }
                _ => {}
            }
        }

        if !new_direction.eq(self.direction)
            && !new_direction.eq(self.direction.reverse()) {
            self.direction = new_direction;
        }

        StateResult::Ok 
    }

    fn handle_state(&mut self) -> StateResult{
        if self.extend > 0 {
            self.extend-=1;
        } else {
            self.body.remove(0);
        }
        let mut head = *self.body.last().unwrap();
        head = head.add(self.direction);

        if self.out_of_bounds(head) || self.ate_itself(head) {
            self.die();
        } else {
            self.check_ate(head);
            self.body.push(head);
        }

        if self.lives == 0 {
            return StateResult::End;
        }
        StateResult::Ok
    }

    fn out_of_bounds(&self, point: Point) -> bool {
        point.x < 0 
            || point.y < 1 
            || point.x >= BOARD_WIDTH as i32
            || point.y >= BOARD_HEIGHT as i32
    }

    fn ate_itself(&self, head: Point) -> bool {
        self.body.iter().any( | p | p.eq(head) )
    }

    fn check_ate(&mut self, point: Point) {

        let mut changed:i32 = 0;

        self.fruits.retain( | &fruit |{
            let ate = fruit.eq(point);
            if ate {
                changed +=1;
            }
            !ate
        });
        self.score += changed;
        self.extend += 3 * changed;

        if self.fruits.len() == 0 {
            self.make_fruits();
        }
    }

    fn make_fruits(&mut self) {

        let mut rng = thread_rng();
        let x:f64 = rng.gen();
        let y:f64 = rng.gen();

        self.fruits.push(
            Point{
                x:(BOARD_WIDTH as f64 * x) as i32, 
                y:(BOARD_HEIGHT as f64 * y) as i32
            }
        );
    }

    fn render_gamefield(&self, terminal: &mut Terminal) {
        for point in &self.body {
            terminal.draw(point.x, point.y, '#');//\u{1F7E2}');
        }

        for fruit in &self.fruits {
            terminal.draw(fruit.x, fruit.y,'*');//'\u{1F34E}');
        }
    }

    fn render_score_lives(&self, terminal: &mut Terminal) {
        terminal.write(0,0, format!("LIVES:{}",self.lives).as_str());
        terminal.write(40,0, format!("SCORE:{}",self.score).as_str());
    }

    fn reset(&mut self){
        self.direction = Point::RIGHT;
        self.body.clear();
        for i in 0..5 {
            self.body.push(Point {x: 5+i, y: 10});
        }
        self.fruits.clear();
        self.fruits.push(Point {x:30, y: 20});
    }

    fn die(&mut self){
        self.reset();
        self.lives-=1;
    }
}

impl GameState for GamePlay {
    fn init(&mut self){
        self.lives = 3;
        self.score = 0;
        self.extend = 0;
        self.reset();
    }
    fn run(&mut self, terminal: &mut Terminal) -> StateResult{
        let result = self.handle_events(terminal);

        if result != StateResult::Ok {
            return result;
        }
        self.handle_state()
    }
    fn render(&self, terminal: &mut Terminal){

        self.render_gamefield(terminal);
        self.render_score_lives(terminal);

    }

}
pub struct GameIntro {
}

impl GameIntro {
    fn new() -> GameIntro {
        GameIntro {}
    }
}

impl GameState for GameIntro {
    fn init(&mut self){
    }
    fn run(&mut self, terminal: &mut Terminal) -> StateResult{
        for event in terminal.events(){
            match event {
                InputEvent::Fire => {
                    return StateResult::End;
                }
                InputEvent::Quit => {
                    return StateResult::Exit;
                }
                _ => {}
            }
        }
        StateResult::Ok
    }
    fn render(&self, terminal: &mut Terminal){
        terminal.write(35, 15, "S N A K E");
        terminal.write(30,17,"Press space to start");
    }

}

pub struct GameOver {
}
impl GameOver {
    fn new() -> GameOver {
        GameOver {}
    }
}
impl GameState for GameOver {
    fn init(&mut self){
    }
    fn run(&mut self, terminal: &mut Terminal) -> StateResult{
        for event in terminal.events(){
            match event {
                InputEvent::Fire => {
                    return StateResult::End;
                }
                InputEvent::Quit => {
                    return StateResult::Exit;
                }
                _ => {}
            }
        }
        StateResult::Ok
    }
    fn render(&self, terminal: &mut Terminal){
        terminal.write(31,15, "G A M E    O V E R");
    }
}

fn run_state(state: &mut dyn GameState, terminal: &mut Terminal) -> StateResult{
    state.init();
    
    loop {
        let result: StateResult = state.run(terminal);

        terminal.clear();

        if result == StateResult::Ok {
            state.render(terminal);
            terminal.flush();
            ::std::thread::sleep(Duration::from_millis(100));
        } else {
            return result;
        }
    }
}

pub fn run(terminal: &mut Terminal) {
    
    terminal.clear_screen();

    let mut game_intro: GameIntro = GameIntro::new();
    let mut game_play = GamePlay::new();
    let mut game_over = GameOver::new();

    let mut states = Vec::<& mut dyn GameState>::new();
    states.push(&mut game_intro);
    states.push(&mut game_play);
    states.push(&mut game_over);
    let mut i = 0;
    
    loop {
        if run_state(states[i], terminal) == StateResult::Exit {
            break;
        }
        i+=1;
        if i == states.len() {
            i = 0;
        }
    }

}
