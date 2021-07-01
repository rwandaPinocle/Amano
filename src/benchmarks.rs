use chess::{ Board };

pub const depths: &[i8] = &[1, 2, 3, 4, 5, 6, 7];

pub const cases: &[(&str, &str)] = &[
    ("Opening: Ruy Lopez", "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 1"),
    ("Opening: Bishop's Opening", "rnbqkbnr/pppp1ppp/8/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 1"),
    ("Opening: King's Gambit", "rnbqkbnr/pppp1ppp/8/4p3/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 2"),
    ("Endgame: Mate Queen and Rook", "8/8/8/3k4/8/8/8/5RQK w - - 0 1"),
    ("Endgame: Queen vs Bishop", "8/8/3B4/6K1/8/8/2k5/q7 b - - 0 1"),
    ("Lichess Puzzle: i3PiQ", "1n4nr/2N1kppp/4Bq2/7Q/3P4/8/1P1K1P1P/6Nb w - - 0 1"),
    ("Lichess Puzzle: LM6pR", "3r1rk1/pp3ppp/q1p1pn2/7R/2NP2PP/PP3P2/2P5/1K1R4 w - - 0 1"),
    ("Lichess Puzzle: OtLqK", "2k5/2r2R2/pN1p2p1/1p1Pp1P1/5n2/P7/1PP5/2K5 w - - 0 1"),
];
