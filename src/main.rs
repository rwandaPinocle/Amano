use chess::{ Board, MoveGen, Color, BoardStatus, ChessMove };
use std::env;
use std::io::{self, Read};
use std::str::FromStr;
mod piece_values;

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const TEST_FEN: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";


fn calc_piece_value(pc_idx: usize, sq_idx: usize, color: Option<Color>) -> i64{
    match color {
        Some(Color::White) => {
            let sq_value = piece_values::PIECE_SQUARES[pc_idx][sq_idx];
            //println!("square value: {}", sq_value);
            return - (piece_values::PIECE_VALS[pc_idx] + sq_value);
        },
        Some(Color::Black) => {
            let sq_value = piece_values::PIECE_SQUARES[pc_idx][63 - sq_idx];
            //println!("square value: {}", sq_value);
            return piece_values::PIECE_VALS[pc_idx] + sq_value;
        },
        None => {
            //println!("ERROR: INVALID SQUARE");
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
            /*
            println!(
                "rank: {}, file: {}, square index: {}, piece type: {}",
                square.get_rank().to_index(),
                square.get_file().to_index(),
                sq_idx,
                pc_type,
            );
            */
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
        _ => calc_pieces_value(board)
    };
    result
}


fn alpha_beta(
        board: &Board, depth: i8,
        is_max: bool, alpha: i64,
        beta: i64,
        total: &mut i64
        ) -> i64 {
    if depth == 0 {
        *total += 1;
        return calc_board_value(board);
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


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let ply_count: i8 = if args.len() >= 2 {
        args[1].parse().unwrap()
    } else {
        println!("Specify ply count");
        return;
    };

    let is_interactive = if args.len() >= 3 { &args[2] == "-i" } else { false };
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
        match Board::from_str(fen_str) {
            Err(_) => { println!("ERROR: Bad fen") }
            Ok(mut board) => {
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
                        match find_best_move(&mut board, ply_count) {
                            Some(n) => {
                                loop {
                                    let mut buffer = String::new();
                                    io::stdin().read_to_string(&mut buffer);
                                    let mv_result = ChessMove::from_san(&board, &buffer);
                                    match mv_result {
                                        Ok(mv) => {
                                            board = board.make_move_new(mv);
                                            break;
                                        }
                                        Err(_) => { println!("Invalid move") }
                                    }
                                }
                            }
                            None => { println!("ERROR: No move found") }
                        }
                        println!("------------------");
                        println!("{}", board);
                        ai_turn = true;
                    }
                }
            }
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
