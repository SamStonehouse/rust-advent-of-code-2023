use crate::inputs;
use std::collections::HashMap;

const IGNORED: [char; 1] = ['.'];
const SYMBOLS: [char; 10] = ['&', '*', '/', '+', '-', '@', '=', '%', '#', '$'];
const VALUES: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const GEARS: [char; 1] = ['*'];

#[derive(Debug, Clone)]
struct RawSchematic {
    lines: Vec<String>,
    ignored: Vec<char>,
    symbols: Vec<char>,
    values: Vec<char>,
}

impl RawSchematic {
    fn new(lines: Vec<String>) -> RawSchematic {
        RawSchematic {
            lines,
            ignored: Vec::from(IGNORED),
            symbols: Vec::from(SYMBOLS),
            values: Vec::from(VALUES),
        }
    }

    fn rect_contains_chars(self: &Self, rect: &Rect, chars: &Vec<char>) -> bool {
        for col in rect.left..(rect.right + 1) {
            for row in rect.top..(rect.bottom + 1) {
                match self.is_char(col, row, chars) {
                    true => return true,
                    _ => (),
                }
            }
        }

        false
    }

    fn get_contains_chars_or_none(
        self: &Self,
        col: isize,
        row: isize,
        chars: &Vec<char>,
    ) -> Option<bool> {
        let symbol = self
            .lines
            .get(usize::try_from(row).ok()?)?
            .chars()
            .collect::<Vec<char>>()
            .get(usize::try_from(col).ok()?)?
            .clone();

        Some(chars.contains(&symbol))
    }

    fn is_char(self: &Self, col: isize, row: isize, chars: &Vec<char>) -> bool {
        if col < 0 {
            return false;
        }

        if row < 0 {
            return false;
        }

        match self.get_contains_chars_or_none(col, row, chars) {
            Some(val) => return val,
            None => return false,
        }
    }

    fn find_part_number_candidates(self: &Self) -> Vec<PartNumberCandidate> {
        let mut candidates: Vec<PartNumberCandidate> = Vec::new();
        let mut current_candidate: Option<PartNumberCandidate> = None;

        self.lines
            .iter()
            .enumerate()
            .for_each(|(line_index, line)| {
                line.chars().enumerate().for_each(|(index, char)| {
                    if SYMBOLS.contains(&char) | IGNORED.contains(&char) {
                        // If we are currently constructing a candidate, stop
                        if let Some(curr) = current_candidate {
                            candidates.push(curr);
                            current_candidate = None;
                        }
                    } else if VALUES.contains(&char) {
                        let value = char as u32 - 0x30;
                        match current_candidate {
                            // If we are currently constructing a candidate, append a value to it
                            Some(curr) => {
                                current_candidate = Some(curr.append_digit(value));
                            }
                            // If we are not, start constructing a new candidate
                            None => {
                                current_candidate =
                                    Some(PartNumberCandidate::new(line_index, index, 1, value));
                                println!("{:?}", current_candidate);
                            }
                        }
                    } else {
                        println!("Unmatched char {:}", char);
                    }
                });

                if let Some(curr) = current_candidate {
                    candidates.push(curr);
                    current_candidate = None;
                }
            });

        candidates
    }

    fn candidate_valid(self: &Self, candidate: &PartNumberCandidate) -> bool {
        let outline = candidate.get_outline().unwrap();
        self.rect_contains_chars(&outline, &self.symbols)
    }

    fn get_part_numbers(self: &Self) -> Vec<PartNumber> {
        self.find_part_number_candidates()
            .iter()
            .filter(|candidate| {
                println!(
                    "{:?} - {:?}",
                    candidate.value,
                    self.candidate_valid(candidate)
                );
                self.candidate_valid(candidate)
            })
            .map(|candidate| PartNumber::from_candidate(candidate))
            .collect()
    }

    fn get_part_proximity_map(self: &Self) {
        let mut proximity_map: HashMap<String, PartNumber> = HashMap::new();

        self.get_part_numbers().iter().for_each(|part_number| {
            println!("Add to hashmap");
            // for col in rect.left..(rect.right + 1) {
            //     for row in rect.top..(rect.bottom + 1) {
            //         match self.is_char(col, row, chars) {
            //             true => return true,
            //             _ => (),
            //         }
            //     }
            // }
        });
    }

    // fn get_gear_candidates(self: &Self) -> Vec<GearCandidate> {
    //     // self.
    // }
}

#[derive(Debug, Clone)]
struct Schematic {
    raw: RawSchematic,
    parts: Vec<PartNumber>,
    gears: Vec<Gear>,
}

impl Schematic {
    fn new(raw: RawSchematic, parts: Vec<PartNumber>, gears: Vec<Gear>) -> Schematic {
        Schematic { raw, parts, gears }
    }
}

#[derive(Debug, Copy, Clone)]
struct PartNumberCandidate {
    start_row: usize,
    start_column: usize,
    length: usize,
    value: u32,
}

impl PartNumberCandidate {
    fn new(
        start_row: usize,
        start_column: usize,
        length: usize,
        value: u32,
    ) -> PartNumberCandidate {
        PartNumberCandidate {
            start_row,
            start_column,
            length,
            value,
        }
    }

    fn append_digit(self: &Self, value: u32) -> PartNumberCandidate {
        PartNumberCandidate::new(
            self.start_row,
            self.start_column,
            self.length + 1,
            self.value * 10 + value,
        )
    }

    fn get_outline(self: &Self) -> Option<Rect> {
        let i_start_row = isize::try_from(self.start_row).ok()?;
        let i_start_col = isize::try_from(self.start_column).ok()?;
        let i_length = isize::try_from(self.length).ok()?;

        Some(Rect {
            top: i_start_row - 1,
            left: i_start_col - 1,
            bottom: i_start_row + 1,
            right: i_start_col + i_length,
        })
    }
}

#[derive(Debug, Clone)]
struct PartNumber {
    start_row: usize,
    start_column: usize,
    length: usize,
    value: u32,
}

impl PartNumber {
    fn new(start_row: usize, start_column: usize, length: usize, value: u32) -> PartNumber {
        PartNumber {
            start_row,
            start_column,
            length,
            value,
        }
    }

    fn from_candidate(candidate: &PartNumberCandidate) -> PartNumber {
        PartNumber {
            start_row: candidate.start_row,
            start_column: candidate.start_column,
            length: candidate.length,
            value: candidate.value,
        }
    }
}

#[derive(Debug, Clone)]
struct GearCandidate {
    row: usize,
    column: usize,
    value: u32,
}

impl GearCandidate {
    fn new(row: usize, column: usize, value: u32) -> GearCandidate {
        GearCandidate { row, column, value }
    }

    // fn get_outline(self: &Self) -> Option<Outline> {
    //     let i_start_row = isize::try_from(self.row).ok()?;
    //     let i_start_col = isize::try_from(self.column).ok()?;

    //     Some(Outline {
    //         top: i_start_row - 1,
    //         left: i_start_col - 1,
    //         bottom: i_start_row + 1,
    //         right: i_start_col + 1,
    //     })
    // }
}

#[derive(Debug, Clone)]
struct Gear {
    row: usize,
    column: usize,
    value: u32,
}

impl Gear {
    fn new(row: usize, column: usize, value: u32) -> Gear {
        Gear { row, column, value }
    }
}

#[derive(Debug, Copy, Clone)]
struct Rect {
    top: isize,
    left: isize,
    bottom: isize,
    right: isize,
}

pub(crate) fn part_one() {
    println!("Day Three, Part One");
    let lines = inputs::read_inputs_from_file("./inputs/day_three.txt").unwrap();
    let schematic = RawSchematic::new(lines);
    let valid_schematics: Vec<PartNumber> = schematic.get_part_numbers();

    let valid_schematic_values: Vec<u32> = valid_schematics
        .iter()
        .map(|candidate| {
            return candidate.value;
        })
        .collect();

    println!("{:?}", valid_schematic_values.iter().sum::<u32>());
}

pub(crate) fn part_two() {
    println!("Day Three, Part Two");
    let lines = inputs::read_inputs_from_file("./inputs/day_three.txt").unwrap();
}
