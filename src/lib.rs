use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use queenfish::board::{Board, Move, Turn};

struct Engine {
    path: String,
    name: String,
    child_process: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
} //

struct Game {
    white: Engine,
    black: Engine,
    moves_list: Vec<String>,
    board: Board,
}

#[derive(Debug)]
struct GameResult {
    white: String,
    black: String,
    moves_list: Vec<String>,
    white_won: bool,
}

impl Game {
    pub fn new(white: Engine, black: Engine) -> Self {
        Game {
            white,
            black,
            moves_list: Vec::new(),
            board: Board::new(),
        }
    } //

    pub fn play(&mut self) -> GameResult {
        loop {
            let valid_moves = self.board.generate_moves();
            if valid_moves.is_empty() {
                match self.board.turn {
                    Turn::WHITE => println!("{} wins as black", self.black.name),
                    Turn::BLACK => println!("{} wins as white", self.white.name),
                }
                return GameResult {
                    white: self.white.name.clone(),
                    black: self.black.name.clone(),
                    moves_list: self.moves_list.clone(),
                    white_won: self.board.turn == Turn::BLACK,
                }
            }
            let engine = match self.board.turn {
                Turn::WHITE => &mut self.white,
                Turn::BLACK => &mut self.black,
            };
            if self.moves_list.is_empty() {
                engine.send_command(format!("position startpos\n").as_str());
            } else {
                engine.send_command(
                    format!("position startpos moves {}\n", self.moves_list.join(" ")).as_str(),
                );
            }
            engine.send_command("go movetime 10\n");

            loop {
                if let Some(line) = engine.read_line() {
                    if line.starts_with("bestmove") {
                        let best_move = line.split_whitespace().nth(1).unwrap();
                        self.moves_list.push(best_move.to_string());
                        self.board.make_move(Move::from_uci(best_move, &self.board));
                        break;
                    } else {
                        // println!("{}", line);
                    }
                }
            }
        };

    } //
} //

impl Engine {
    pub fn new(path: &str, name: &str) -> Self {
        let path = Path::new(path);

        if !path.exists() {
            panic!("Engine path does not exist");
        } else if !path.is_file() {
            panic!("Engine path is not a file");
        }
        if let Some(extension) = path.extension() {
            if extension != "exe" && extension != "" {
                panic!("Engine file is not an executable");
            }
        } else {
            panic!("Engine file has no extension");
        }

        let mut engine_process = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start engine process");

        let mut stdin = engine_process
            .stdin
            .take()
            .expect("Failed to take engine stdin");
        let mut stdout = BufReader::new(
            engine_process
                .stdout
                .take()
                .expect("Failed to take engine stdout"),
        );

        stdin
            .write_all("uci\n".as_bytes())
            .expect("Failed to write 'uci' to engine stdin");

        let is_uci_ok;
        loop {
            let mut line = String::new();
            let _ = stdout.read_line(&mut line);
            if line.starts_with("uciok") {
                is_uci_ok = true;
                break;
            }
        }
        if !is_uci_ok {
            panic!("Engine is not UCI compatible");
        }

        Engine {
            path: path.to_str().unwrap().to_string(),
            name: name.to_string(),
            child_process: engine_process,
            stdin,
            stdout,
        }
    } //

    pub fn send_command(&mut self, command: &str) {
        self.stdin
            .write_all(command.as_bytes())
            .expect("Failed to write command to engine stdin");
        self.stdin.flush().unwrap();
    } //

    pub fn read_line(&mut self) -> Option<String> {
        let mut line = String::new();
        self.stdout.read_line(&mut line).ok()?;
        if line.is_empty() { None } else { Some(line) }
    } //
} //

mod test {
    use super::*;
    use queenfish::board::bishop_magic::init_bishop_magics;
    use queenfish::board::rook_magic::init_rook_magics;
    use queenfish::board::{Board, Move, Turn};

    #[test]
    fn it_works() {
        init_bishop_magics();
        init_rook_magics();

        let engine = Engine::new(
            "C:\\Learn\\LearnRust\\chess\\target\\release\\uci.exe",
            "Queenfish 2",
        );

        let engine2 = Engine::new(
            "C:\\Program Files\\stockfish\\stockfish-windows-x86-64-avx2.exe",
            "Stockfish",
        );

        let mut game = Game::new(engine , engine2);
        dbg!(game.play());

    }
} //
