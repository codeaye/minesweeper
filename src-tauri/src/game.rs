use serde::Serialize;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

pub fn random_range(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
}

pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    mine_count: usize,
    lost: bool,
    won: bool,
    first: bool,
}

#[derive(Serialize, Debug)]
pub struct MinesweeperView {
    code: u8,
    x: usize,
    y: usize,
    count: u8,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: HashSet::new(),
            flagged_fields: HashSet::new(),
            lost: false,
            won: false,
            mine_count,
            first: true,
        }
    }

    pub fn initiate_fields(&mut self, start: Position) -> Option<OpenResult> {
        self.mines = {
            let mut mines = HashSet::new();
            let mut i: usize = 0;

            while i < self.mine_count {
                let pos: Position = (random_range(0, self.width), random_range(0, self.height));

                if pos != start && !mines.contains(&pos) {
                    mines.insert(pos);
                    i += 1;
                }
            }

            mines
        };
        self.first = false;
        self.open(start)
    }

    pub fn view(&self) -> Vec<MinesweeperView> {
        let mut result = Vec::<MinesweeperView>::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);

                if !self.open_fields.contains(&pos) {
                    if self.lost && self.mines.contains(&pos) {
                        result.push(MinesweeperView {
                            code: 2,
                            x,
                            y,
                            count: 0,
                        })
                    } else if self.flagged_fields.contains(&pos) {
                        result.push(MinesweeperView {
                            code: 3,
                            x,
                            y,
                            count: 0,
                        })
                    } else {
                        result.push(MinesweeperView {
                            code: 0,
                            x,
                            y,
                            count: 0,
                        });
                    }
                } else if self.mines.contains(&pos) {
                    result.push(MinesweeperView {
                        code: 2,
                        x,
                        y,
                        count: 0,
                    });
                } else {
                    let mine_count = self.neighboring_mines(pos);

                    if mine_count > 0 {
                        result.push(MinesweeperView {
                            code: 4,
                            x,
                            y,
                            count: mine_count,
                        });
                    } else {
                        result.push(MinesweeperView {
                            code: 1,
                            x,
                            y,
                            count: 0,
                        });
                    }
                }
            }
        }
        result
    }

    pub fn flag_count(&self) -> usize {
        self.flagged_fields.len()
    }

    pub fn reset(&mut self) {
        self.open_fields.clear();
        self.flagged_fields.clear();
        self.mines.clear();
        self.lost = false;
        self.won = false;
        self.first = true;
    }

    pub fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    pub fn neighboring_mines(&self, pos: Position) -> u8 {
        self.iter_neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn open(&mut self, pos: Position) -> Option<OpenResult> {
        if self.first == false {
            if self.open_fields.contains(&pos) {
                let mine_count = self.neighboring_mines(pos);
                let flag_count = self
                    .iter_neighbors(pos)
                    .filter(|neighbor| self.flagged_fields.contains(neighbor))
                    .count() as u8;

                if mine_count == flag_count {
                    for neighbor in self.iter_neighbors(pos) {
                        if !self.flagged_fields.contains(&neighbor)
                            && !self.open_fields.contains(&neighbor)
                        {
                            self.open(neighbor);
                        }
                    }
                }

                return None;
            }

            if self.lost || self.flagged_fields.contains(&pos) {
                return None;
            }

            self.open_fields.insert(pos);

            let is_mine = self.mines.contains(&pos);

            if is_mine {
                self.lost = true;
                Some(OpenResult::Mine)
            } else {
                let mine_count = self.neighboring_mines(pos);

                if mine_count == 0 {
                    for neighbor in self.iter_neighbors(pos) {
                        if !self.open_fields.contains(&neighbor) {
                            self.open(neighbor);
                        }
                    }
                }

                Some(OpenResult::NoMine(mine_count))
            }
        } else {
            self.initiate_fields(pos)
        }
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.lost || self.open_fields.contains(&pos) {
            return;
        }
        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }

        let mut won_i = true;

        for mine in self.mines.iter() {
            if self.flagged_fields.contains(mine) == false {
                won_i = false;
            }
        }

        self.won = won_i;
    }

    pub fn won(&self) -> bool {
        self.won
    }

    pub fn lost(&self) -> bool {
        self.lost
    }
}
