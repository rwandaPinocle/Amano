extern crate args;
extern crate getopts;

use chess::{ Board, MoveGen, Color, BoardStatus, ChessMove };
use std::env;
use std::io::{self, Read};
use std::str::FromStr;
use args::{Args, ArgsError};
use args::validations::{Order, OrderValidation};
use getopts::Occur;
mod piece_values;

//const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const STARTING_FEN: &str = "rnbqkbnr/ppppp1pp/8/5p2/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 2";
const TEST_FEN: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";
const DEFAULT_DEPTH: i64 = 4;

const PROGRAM_DESC: &'static str = "A good old fashioned Rust chess engine";
const PROGRAM_NAME: &'static str = "Amano";


fn calc_piece_value(pc_idx: usize, sq_idx: usize, color: Option<Color>) -> i64{
    match color {
        Some(Color::White) => {
            let sq_value = piece_values::PIECE_SQUARES[pc_idx][sq_idx];
            return -(piece_values::PIECE_VALS[pc_idx] + sq_value);
        },
        Some(Color::Black) => {
            let sq_value = piece_values::PIECE_SQUARES[pc_idx][63 - sq_idx];
            return piece_values::PIECE_VALS[pc_idx] + sq_value;
        },
        None => {
            return 0;
        },
    }
}

fn calc_pieces_value(board: &Board) -> i64{
    let mut result = 0;
    for pc_idx in 0..6 {
        let pc_type = piece_values::PIECES[pc_idx];
        let bboard = *board.pieces(pc_type);
        for square in bboard {
            let sq_idx = square.to_index();
            result += calc_piece_value(pc_idx, sq_idx, board.color_on(square));
        }
    }
    result
}


fn calc_board_value(board: &Board) -> i64 {
    let w_move = board.side_to_move() == Color::White;
    let result = match board.status() {
        BoardStatus::Checkmate => if w_move { 20000 } else { -20000 },
        BoardStatus::Stalemate => 0,
        BoardStatus::Ongoing => calc_pieces_value(board)
    };
    result
}


fn alpha_beta(
        board: &Board, depth: i8,
        is_max: bool, alpha: i64,
        beta: i64,
        total: &mut i64
        ) -> i64 {
    if (depth == 0) || (board.status() != BoardStatus::Ongoing) {
        *total += 1;
        let val = calc_board_value(board);
        return val;
    }
    let mut alpha = alpha;
    let mut beta = beta;
    if is_max {
        let mut best_val = i64::MIN;
        let moves = MoveGen::new_legal(&board);
        let mut result_board = chess::Board::default(); 
        for mv in moves {
            board.make_move(mv, &mut result_board);
            let value = alpha_beta(
                &result_board,
                depth-1,
                false,
                alpha,
                beta,
                total
            );
            best_val = std::cmp::max(value, best_val);
            alpha = std::cmp::max(alpha, best_val);
            if beta <= alpha {
                break;
            }
        }
        return best_val;
    } else {
        let mut best_val = i64::MAX;
        let moves = MoveGen::new_legal(&board);
        let mut result_board = chess::Board::default(); 
        for mv in moves {
            board.make_move(mv, &mut result_board);
            let value = alpha_beta(
                &result_board,
                depth-1,
                true,
                alpha,
                beta,
                total
            );
            best_val = std::cmp::min(value, best_val);
            beta = std::cmp::min(beta, best_val);
            if beta <= alpha {
                break;
            }
        }
        return best_val;
    }
}


fn find_best_move(board: &Board, depth: i8) -> Option<ChessMove> {
    let black_move = board.side_to_move() == Color::Black;
    let moves = MoveGen::new_legal(board);
    let mut best_move = MoveGen::new_legal(board).nth(0);
    let mut best_val;
    let is_better = {
        if black_move {
            best_val = i64::MIN;
            |x: i64, y: i64| -> bool { x > y }
        } else {
            best_val = i64::MAX;
            |x: i64, y: i64| -> bool { x < y }
        }
    };
    let mut total = 0;
    for mv in moves {
        let mut new_board = Board::default();       
        println!("{:?}", mv);
        board.make_move(mv, &mut new_board);
        let val = alpha_beta(
            &new_board,
            depth,
            black_move,
            i64::MIN,
            i64::MAX,
            &mut total
        );
        //println!("move: {}, val: {}", mv, val);
        if is_better(val, best_val) {
            best_val = val;
            best_move = Some(mv);
        }
    }
    println!("Positions evaluated: {}", total);
    best_move
}


fn parse(
    input: &Vec<&str>,
    is_help: &mut bool,
    is_interactive: &mut bool,
    fen_str: &mut String,
    depth_str: &mut String,
) -> Result<(), ArgsError>
{
    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
    args.flag("h", "help", "Print the usage menu");
    args.flag("i", "interactive", "Run the engine in interactive mode");
    args.option("d", "depth", "The depth of the tree search. Default = 4",
        "DEPTH", Occur::Req, Some("4".to_string())
    );
    args.option("f", "fen", "The state of the game as FEN",
        "FEN", Occur::Optional, Some(STARTING_FEN.to_string())
    );
    args.parse(input)?;

    *is_help = args.value_of("help")?;
    *is_interactive = args.value_of("interactive")?;
    *fen_str = args.value_of("fen")?;
    *depth_str = args.value_of("depth")?;
    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    let ply_count: i8 = if args.len() >= 2 {
        args[args.len()-1].parse().unwrap()
    } else {
        println!("Specify ply count");
        return;
    };

    let is_interactive = args.iter().any(|i| i == "-i");
    let fen_str = if args.len() >= 4 { &args[3] } else { STARTING_FEN };

    if !is_interactive {
        match Board::from_str(fen_str) {
            Err(_) => { println!("ERROR: Bad fen") }
            Ok(mut board) => {
                match find_best_move(&board, ply_count) {
                    Some(n) => { println!("Best move: {}", n) },
                    None => { println!("ERROR: No move found") },
                }
            }
        }
    } else {
        if let Ok(mut board) = Board::from_str(fen_str) {
            let mut ai_turn = true;
            loop {
                if ai_turn {
                    println!("Finding best move");
                    match find_best_move(&mut board, ply_count) {
                        Some(n) => { board = board.make_move_new(n) }
                        None => { println!("ERROR: No move found") }
                    }
                    println!("------------------");
                    println!("{}", board);
                    println!("Your move");
                    ai_turn = false;
                } else {
                    if let Some(n) = find_best_move(&mut board, ply_count) {
                        loop {
                            let mut buffer = String::new();
                            io::stdin().read_to_string(&mut buffer);
                            let mv_result = ChessMove::from_san(&board, &buffer);
                            if let Ok(mv) = mv_result {
                                board = board.make_move_new(mv);
                                break;
                            } else {
                                println!("Invalid move");
                            }
                        }
                    } else {
                        println!("ERROR: No move found")
                    }
                    println!("------------------");
                    println!("{}", board);
                    ai_turn = true;
                }
            }
        } else {
            println!("ERROR: Bad fen");
        }
    }

    /*
    match Board::from_str(STARTING_FEN) {
        Ok(board) => {
            match find_best_move(&board) {
                Some(n) => { println!("Best move: {}", n) },
                None => { println!("ERROR: NO MOVE FOUND") }
            }
            //println!("board value: {}", calc_board_value(&board));
        }
        _ => { println!("ERROR: Invalid board") }
    };
    */
}
