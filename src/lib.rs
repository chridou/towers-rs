use std::cell::Cell;
use std::convert::{Into, From};

mod iterative_solver;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Disk(usize);

impl Disk {
    pub fn size(&self) -> usize {
        self.0
    }
}

pub struct Peg {
    disks: Vec<Disk>,
}

impl Peg {
    pub fn new() -> Peg {
        Peg { disks: Vec::new() }
    }

    pub fn with_disks(num_disks: usize) -> Peg {
        let mut peg = Peg { disks: Vec::new() };
        for h in 0..num_disks {
            let disk = Disk(num_disks - h);
            peg.put(disk).unwrap();
        }
        peg
    }

    pub fn put(&mut self, disk: Disk) -> Result<(), String> {
        match self.disks.pop() {
            Some(top_disk) => {
                if disk < top_disk {
                    self.disks.push(top_disk);
                    self.disks.push(disk);
                    Ok(())
                } else {
                    self.disks.push(top_disk);
                    Err(format!("Disk is too big: {:?}", disk))
                }
            }
            None => {
                self.disks.push(disk);
                Ok(())
            }
        }
    }

    pub fn take(&mut self) -> Option<Disk> {
        self.disks.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.disks.is_empty()
    }

    pub fn disks(&self) -> usize {
        self.disks.len()
    }

    pub fn contains_smallest(&self) -> bool {
        self.disks.iter().any(|d| d.size() == 1)
    }

    pub fn peek(&self) -> Option<&Disk> {
        self.disks.last()
    }
}

pub struct Board {
    pegs: [Peg; 3],
}

impl Board {
    pub fn with_initial_disks(disks: usize) -> Board {
        Board { pegs: [Peg::with_disks(disks), Peg::new(), Peg::new()] }
    }

    pub fn put(&mut self, disk: Disk, peg_index: usize) -> Result<(), String> {
        if peg_index > 2 {
            return Err("peg_index out of bounds.".to_string());
        }
        let peg = &mut self.pegs[peg_index];
        peg.put(disk)
    }

    pub fn take(&mut self, peg_index: usize) -> Result<Option<Disk>, String> {
        if peg_index > 2 {
            return Err("peg_index out of bounds.".to_string());
        }
        let peg = &mut self.pegs[peg_index];
        Ok(peg.take())
    }

    pub fn pegs(&self) -> &[Peg; 3] {
        &self.pegs
    }

    pub fn is_finished(&self) -> bool {
        self.pegs[0].is_empty() && self.pegs[1].is_empty()
    }

    pub fn num_disks(&self) -> usize {
        self.pegs.iter().map(|p| p.disks()).sum()
    }
}

#[derive(Debug)]
pub enum PlayerAction {
    Moved { from: usize, to: usize },
    Finished,
}

pub trait PlaysTowers {
    fn name(&self) -> &str;
    fn next_turn(&self, board: &mut Board) -> Result<PlayerAction, String>;
}

pub struct StupidPlayer {
    name: String,
    next_move_number: Cell<u64>,
}

impl StupidPlayer {
    pub fn new<T: Into<String>>(name: T) -> Self {
        StupidPlayer {
            name: name.into(),
            next_move_number: Cell::new(1),
        }
    }
}

impl PlaysTowers for StupidPlayer {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn next_turn(&self, board: &mut Board) -> Result<PlayerAction, String> {
        let current_move_number = self.next_move_number.get();
        match iterative_solver::next_move(current_move_number, board) {
            Some((from, to)) => {
                self.next_move_number.set(current_move_number + 1);
                // There are multiple solutions. Can you rewrite it?
                // What are the advantages and disadvantages of the different solutions?
                board.take(from)
                    .and_then(|maybe_a_disk| maybe_a_disk.ok_or("no disc!".to_string()))
                    .and_then(|disk| board.put(disk, to))
                    .map(|_| {
                        PlayerAction::Moved {
                            from: from,
                            to: to,
                        }
                    })
                    .map_err(|err| format!("Cannot move from {} to {}: {}", from, to, err))
            }
            None => Ok(PlayerAction::Finished),
        }
    }
}

#[derive(Debug)]
pub enum PegName {
    Source,
    Middle,
    Destination,
}

#[derive(Debug)]
pub enum SessionEvent {
    PlayerMovedDisk { from: PegName, to: PegName },
    PlayerWins,
    PlayerGaveUp,
    PlayerCheated(String),
}

pub struct Session<P: PlaysTowers> {
    player: P,
    board: Board,
}

impl<P: PlaysTowers> Session<P> {
    pub fn new(player: P, board: Board) -> Session<P> {
        Session {
            player: player,
            board: board,
        }
    }

    pub fn with_initial_disks(player: P, num_disks: usize) -> Session<P> {
        Session {
            player: player,
            board: Board::with_initial_disks(num_disks),
        }
    }

    pub fn next_turn(&mut self) -> SessionEvent {
        match self.player.next_turn(&mut self.board) {
            Ok(PlayerAction::Moved { from, to }) => {
                SessionEvent::PlayerMovedDisk {
                    from: from.into(),
                    to: From::from(to),
                }
            }
            Ok(PlayerAction::Finished) => {
                if self.board.is_finished() {
                    SessionEvent::PlayerWins
                } else {
                    SessionEvent::PlayerGaveUp
                }
            }
            Err(err) => SessionEvent::PlayerCheated(format!("{}", err)),
        }
    }

    pub fn iter<'a>(&'a mut self) -> SessionIterator<P> {
        SessionIterator {
            is_finished: false,
            session: self,
        }
    }
}

pub struct SessionIterator<'a, P: 'a + PlaysTowers> {
    is_finished: bool,
    session: &'a mut Session<P>,
}

impl<'a, P: PlaysTowers> Iterator for SessionIterator<'a, P> {
    type Item = SessionEvent;

    fn next(&mut self) -> Option<SessionEvent> {
        if self.is_finished {
            None
        } else {
            match self.session.next_turn() {
                event @ SessionEvent::PlayerMovedDisk { .. } => Some(event),
                event => {
                    self.is_finished = true;
                    Some(event)
                }
            }
        }
    }
}

// Try From is not stable! Implement it yourself
impl From<usize> for PegName {
    fn from(what: usize) -> PegName {
        match what {
            0 => PegName::Source,
            1 => PegName::Middle,
            2 => PegName::Destination,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test;