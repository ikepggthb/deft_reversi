extern crate termion;
use std::io::{stdout, Write, stdin};
use bevy::transform::commands;
use termion::*;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

mod board;
use board::*;

type TermOut = AlternateScreen<raw::RawTerminal<std::io::Stdout>>;
struct BoardCursor {
    x: i32,y: i32
}
impl BoardCursor {
    fn new() -> BoardCursor {BoardCursor { x: 0, y: 0 }}
    fn up(&mut self){
        if self.y == 0 {self.y = 7} else {self.y -= 1};
    }
    fn right(&mut self){
        if self.x == 7 {self.x = 0} else {self.x += 1};
    }
    fn down(&mut self){
        if self.y == 7 {self.y = 0} else {self.y += 1};
    }
    fn left(&mut self){
        if self.x == 0 {self.x = 7} else {self.x -= 1};
    }
}

fn init_terminal(output: &mut TermOut) -> std::io::Result<()> {
    write!(output, "{}{}", clear::All, cursor::Goto(1, 1))?;
    output.flush()?;
    Ok(())
}

pub fn start() -> std::io::Result<()>{

    let mut stdin: std::io::Stdin = stdin();
    let mut output = 
        AlternateScreen::from( stdout().into_raw_mode().unwrap());
        
    init_terminal(&mut output)?;
    title_screen(&mut output, &mut stdin);
    
    let mut board = Board::new();
    let mut board_cursor = BoardCursor::new();
    //print_board(&board, board_cursor.y, board_cursor.x, &mut output)?;
    
    


    for evt in stdin.events() {
        match evt? {
            // Ctrl-cでプログラム終了
            Event::Key(Key::Ctrl('c')) =>  {
                return Ok(());
            }
            Event::Key(Key::Char(key_char)) => {
                match key_char {
                    'w' => board_cursor.up(),
                    'a' => board_cursor.left(),
                    's' => board_cursor.down(),
                    'd' => board_cursor.right(),
                    'z' => {board.put_eval_one_simple();},
                    'x' => {board.put_eval_zero_simple();},
                    'c' => {board.put_random_piece();},
                    'v' => {board.put_piece_from_coord(board_cursor.y, board_cursor.x);},
                    'p' => board.turns ^= 1,    
                    //{board.put_piece_from_coord(board_cursor.y, board_cursor.x);},//
                    'q' => return Ok(()),
                    _ => ()
                }
                print_board(&board, board_cursor.y, board_cursor.x, &mut output)?;
            }
            _ => ()
        }
    }

    Ok(())
}


enum TitleScreenOption {
    Start,
    Exit,
    None
}

const TITLE: &str = "Deft Reversi";

fn title_screen(output: &mut TermOut, input: &mut std::io::Stdin)-> std::io::Result<()>{
    init_terminal(output)?;
    print_title_screen(output, input)?;

    Ok(())
}

struct TitleScreenObject {
    name: String,
    x: i32,
    y: i32,
    label : String,
    option: TitleScreenOption
}

fn print_title_screen(output: &mut TermOut, input: &mut std::io::Stdin)-> std::io::Result<()>{
    let title_label = TitleScreenObject {
        name: "title label".to_string(),
        x: 1,
        y: 1,
        label: TITLE.to_string(),
        option: TitleScreenOption::None
    };
    let game_start_button = TitleScreenObject {
        name: "start button".to_string(),
        x: 1,
        y: 3,
        label: "Game Start".to_string(),
        option: TitleScreenOption::Start
    };
    let exit_button = TitleScreenObject {
        name: "exit button".to_string(),
        x: 1,
        y: 5,
        label: "Exit".to_string(),
        option: TitleScreenOption::Exit
    };

    let title_object = [&title_label, &game_start_button, &exit_button];

    for x in 0..8 {
        for y in 0..8 {
            for ob in title_object {
                if ob.x == x && ob.y == y {
                    write!(output, "{}", ob.label)?;
                }
            }
        }
    }
    output.flush()?;
    
    Ok(())
}


pub fn print_board(board: &Board, y_now: i32, x_now: i32, output: &mut TermOut) -> std::io::Result<()>{
    init_terminal(output)?;
    write!(output, "black: {}\n", board.black_pieces_count)?;

    write!(output, "{}", cursor::Goto(1, 2))?;
    write!(output, "white: {}\n", board.white_pieces_count)?;

    write!(output, "{}", cursor::Goto(1, 3))?;
    for y in 0..8 {
        for x in 0..8 {
            let value: char = {
                if y == y_now && x == x_now {'+'}
                else {
                    let mask: u64 = 1 << y * 8 + x;
                    let put_able_bit = board.put_able();
                    
                    if put_able_bit & mask != 0 {'*'}
                    else if board.bit_board[0] & mask != 0 {'●'}
                    else if board.bit_board[1] & mask != 0 {'○'}
                    else {'.'}
                }
            };
            write!(output, "{} ", value)?;
        }

        write!(output, "\n")?;
        write!(output, "{}", cursor::Goto(1, y as u16 +4))?;
        
    }
    write!(output, "{}", cursor::Goto(1, 13))?;
    write!(output, "{} turn", if board.turns == 0 {"Black"} else {"White"})?;

    output.flush()?;
    Ok(())
}



fn main() -> std::io::Result<()> {
    start();
    Ok(())
}

