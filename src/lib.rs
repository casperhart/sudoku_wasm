mod utils;

use rand::seq::SliceRandom;

use sudoku::strategy::{Strategy, StrategySolver};
use sudoku::{Sudoku, Symmetry};
use wasm_bindgen::prelude::*;

enum Difficulty {
    Easy,
    Medium,
    Hard,
    Any,
}

#[wasm_bindgen(js_namespace = console)]
extern "C" {
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn printme() {
    log("Hello, sudoku_wasm!");
}

const EASY_RULES: &[Strategy] = &[Strategy::NakedSingles, Strategy::HiddenSingles];
const MEDIUM_RULES: &[Strategy] = &[
    Strategy::NakedSingles,
    Strategy::HiddenSingles,
    Strategy::LockedCandidates,
    Strategy::NakedPairs,
    Strategy::NakedTriples,
    Strategy::NakedQuads,
    Strategy::HiddenPairs,
    Strategy::HiddenTriples,
    Strategy::HiddenQuads,
    Strategy::XWing,
    Strategy::Swordfish,
    Strategy::Jellyfish,
    Strategy::XyWing,
    Strategy::XyzWing,
    Strategy::MutantSwordfish,
    Strategy::MutantJellyfish,
];

fn try_generate_sudoku(s: &Symmetry, difficulty: &Difficulty) -> Option<Sudoku> {
    let sudoku = Sudoku::generate_with_symmetry(*s);
    let easy_solver = StrategySolver::from_sudoku(sudoku);
    let medium_solver = StrategySolver::from_sudoku(sudoku);

    let easy_solution = easy_solver.solve(EASY_RULES);
    let medium_solution = medium_solver.solve(MEDIUM_RULES);

    match (difficulty, easy_solution, medium_solution) {
        (Difficulty::Any, _, _) => Some(sudoku),
        (Difficulty::Easy, Ok(_), _) => Some(sudoku),
        (Difficulty::Medium, _, Ok(_)) => Some(sudoku),
        (Difficulty::Hard, Err(_), Err(_)) => Some(sudoku),
        _ => None,
    }
}

#[wasm_bindgen]
pub fn generate_sudoku(difficulty: &str) -> String {
    let d = match difficulty {
        "easy" => &Difficulty::Easy,
        "medium" => &Difficulty::Medium,
        "hard" => &Difficulty::Hard,
        _ => &Difficulty::Any,
    };
    let symmetries = [
        sudoku::Symmetry::AntidiagonalMirror,
        sudoku::Symmetry::BidiagonalMirror,
        sudoku::Symmetry::DiagonalMirror,
        sudoku::Symmetry::Dihedral,
        sudoku::Symmetry::HalfRotation,
        sudoku::Symmetry::HorizontalMirror,
        sudoku::Symmetry::QuarterRotation,
        sudoku::Symmetry::VerticalAndHorizontalMirror,
        sudoku::Symmetry::VerticalMirror,
        sudoku::Symmetry::None,
    ];

    let mut rng = rand::thread_rng();

    let s = symmetries.choose(&mut rng).unwrap();

    let sudoku = loop {
        match try_generate_sudoku(s, d) {
            Some(sudoku) => break sudoku,
            None => {
                log("Generated sudoku doesn't have the right difficulty. Retrying");
                continue;
            }
        }
    };

    sudoku.to_string()
}
