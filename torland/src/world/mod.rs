use botc::code::Dir;
use std::{
    borrow::Borrow, cell::{Ref, RefCell}, mem::swap, rc::Rc, usize
};

mod bot;
pub use bot::Bot;

use crate::vec2::Vec2u;

type BotRef = Rc<RefCell<Bot>>;

#[derive(Debug)]
pub enum Cell {
    None,
    Bot(BotRef),
}

pub struct WorldConfig {
    pub h: usize,
    pub w: usize,
}

pub struct World {
    size: Vec2u,
    map: Vec<Vec<Cell>>,
    bots: Vec<(Vec2u, BotRef)>,
}

struct WordAccessor<'a> {
    pos: &'a mut Vec2u,
    map_size: Vec2u,
    map: &'a mut Vec<Vec<Cell>>,
    newborn: &'a mut Vec<(Vec2u, BotRef)>,
}

impl<'a> WordAccessor<'_> {
    fn mov(&mut self, dir: Dir) -> Result<(), ()> {
        let pos = self.pos.mod_add(dir.into(), self.map_size);
        // let new_cell = &mut self.map[pos.y][pos.x];
        match self.map[pos.y][pos.x] {
            Cell::None => {
                let mut old_cell = Cell::None;
                swap(&mut self.map[self.pos.y][self.pos.x], &mut old_cell);
                self.map[pos.y][pos.x] = old_cell;
                *self.pos = pos;
                Ok(())
            },
            Cell::Bot(_) => Err(()),
        }
    }

    fn spawn(&mut self, dir: Dir, b: Bot) -> Result<(), ()> {
        let pos = self.pos.mod_add(dir.into(), self.map_size);
        // let new_cell = &mut self.map[pos.y][pos.x];
        match self.map[pos.y][pos.x] {
            Cell::None => {
                let b = Rc::new(RefCell::new(b));
                self.map[pos.y][pos.x] = Cell::Bot(b.clone());
                self.newborn.push((pos, b));
                Ok(())
            },
            Cell::Bot(_) => Err(()),
        }
    }
}

impl World {
    pub fn new(cfg: WorldConfig) -> Self {
        let mut map = Vec::new();
        for _ in 0..cfg.h {
            let mut row = Vec::new();
            for _ in 0..cfg.w {
                row.push(Cell::None);
            }
            map.push(row);
        }
        Self {
            size: (cfg.w, cfg.h).into(),
            map,
            bots: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        let mut newborn: Vec<(Vec2u, BotRef)> = Vec::new();
        for (pos, b) in self.bots.iter_mut() {
            let mut wa = WordAccessor {
                newborn: &mut newborn,
                pos: pos,
                map_size: self.size,
                map: &mut self.map,
            };
            RefCell::borrow_mut(Rc::borrow(b)).update(&mut wa);
        }
        self.bots.append(&mut newborn);
        self.bots.retain(|(pos, bot)| {
            if RefCell::borrow_mut(Rc::borrow(bot)).is_live() {
                true
            } else {
                self.map[pos.y][pos.x] = Cell::None;
                false
            }
        });
    }

    pub fn foreach_cell<F>(&self, mut func: F)
    where
        F: FnMut(usize, usize, &Cell),
    {
        self.map.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, c)| {
                func(x, y, c);
            });
        });
    }

    pub fn foreach_bot<F>(&self, mut func: F)
    where
        F: FnMut(usize, usize, Ref<Bot>),
    {
        self.bots.iter().for_each(|(pos, c)| {
            func(pos.x, pos.y, RefCell::borrow(Rc::borrow(c)));
        });
    }

    pub fn spawn(&mut self, pos: Vec2u, gen_b32: String) -> Result<(), ()> {
        let genom = botc::code_packer::from_b32(&gen_b32).map_err(|_|())?;
        let cell = self.map.get_mut(pos.y).ok_or(())?.get_mut(pos.x).ok_or(())?;
        match cell {
            Cell::None => {
                let b = Rc::new(RefCell::new(Bot::new(genom)));
                self.map[pos.y][pos.x] = Cell::Bot(b.clone());
                self.bots.push((pos, b));
                Ok(())
            },
            Cell::Bot(_) => Err(()),
        }
    }
}
