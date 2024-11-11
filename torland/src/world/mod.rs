use botc::code::Dir;
use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    mem::swap,
    rc::Rc,
    usize,
};

mod bot;
pub use bot::Bot;

use crate::vec2::Vec2u;

type BotRef = Rc<RefCell<Bot>>;

#[derive(Debug)]
pub struct Cell {
    sun: usize,
    mineral: usize,
    bot: Option<BotRef>,
}

pub struct WorldConfig {
    pub h: usize,
    pub w: usize,
}

pub struct World {
    size: Vec2u,
    map: Vec<Vec<Cell>>,
    bots: Vec<(Vec2u, BotRef)>,
    colony_cnt: usize,
}

struct WordAccessor<'a> {
    pos: &'a mut Vec2u,
    map_size: Vec2u,
    map: &'a mut Vec<Vec<Cell>>,
    newborn: &'a mut Vec<(Vec2u, BotRef)>,
    colony_cnt: &'a mut usize,
}

impl<'a> WordAccessor<'_> {
    fn mov(&mut self, dir: Dir) -> Result<(), ()> {
        let pos = self.pos.mod_add(dir.into(), self.map_size);
        if let None = self.map[pos.y][pos.x].bot {
            let mut bot = None;
            swap(&mut self.map[self.pos.y][self.pos.x].bot, &mut bot);
            self.map[pos.y][pos.x].bot = bot;
            *self.pos = pos;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_new_colony_id(&mut self) -> usize {
        let rc = *self.colony_cnt;
        *self.colony_cnt = *self.colony_cnt + 1;
        rc
    }

    fn spawn(&mut self, dir: Dir, b: Bot) -> Result<(), ()> {
        let pos = self.pos.mod_add(dir.into(), self.map_size);
        if let None = self.map[pos.y][pos.x].bot {
            let b = Rc::new(RefCell::new(b));
            self.map[pos.y][pos.x].bot = Some(b.clone());
            self.newborn.push((pos, b));
            Ok(())
        } else {
            Err(())
        }
    }

    fn kill(&mut self, dir: Dir) -> Option<isize> {
        let pos = self.pos.mod_add(dir.into(), self.map_size);
        self.map[pos.y][pos.x]
            .bot
            .clone()
            .map(|b| RefCell::borrow_mut(Rc::borrow(&b)).kill())
    }

    fn get_sun(&self) -> isize {
        self.map[self.pos.y][self.pos.x].sun as isize
    }

    fn get_sun_diff(&self, dir: Dir) -> isize {
        let pos = self.pos.mod_add(dir.into(), self.map_size);
        self.map[self.pos.y][self.pos.x].sun as isize - self.map[pos.y][pos.x].sun as isize
    }

    fn get_mineral(&self) -> isize {
        self.map[self.pos.y][self.pos.x].mineral as isize
    }

    fn get_mineral_diff(&self, dir: Dir) -> isize {
        let pos = self.pos.mod_add(dir.into(), self.map_size);
        self.map[self.pos.y][self.pos.x].mineral as isize - self.map[pos.y][pos.x].mineral as isize
    }

    fn is_some_colony(&self, dir: Dir, colony: usize) -> Option<bool> {
        let pos = self.pos.mod_add(dir.into(), self.map_size);
        let Some(bot) = &self.map[pos.y][pos.x].bot else {
            return None;
        };
        Some(RefCell::borrow(Rc::borrow(bot)).get_colony() == colony)
    }
}

impl World {
    pub fn new(cfg: WorldConfig) -> Self {
        let mut map = Vec::new();
        for _ in 0..cfg.h {
            let mut row = Vec::new();
            for _ in 0..cfg.w {
                row.push(Cell {
                    sun: 5,
                    mineral: 0,
                    bot: None,
                });
            }
            map.push(row);
        }
        Self {
            size: (cfg.w, cfg.h).into(),
            map,
            bots: Vec::new(),
            colony_cnt: 0,
        }
    }

    pub fn update(&mut self) -> Result<(), ()> {
        let mut newborn: Vec<(Vec2u, BotRef)> = Vec::new();
        for (pos, b) in self.bots.iter_mut() {
            let mut wa = WordAccessor {
                newborn: &mut newborn,
                pos: pos,
                map_size: self.size,
                map: &mut self.map,
                colony_cnt: &mut self.colony_cnt,
            };
            RefCell::borrow_mut(Rc::borrow(b)).update(&mut wa)?;
        }
        self.bots.append(&mut newborn);
        self.bots.retain(|(pos, bot)| {
            if RefCell::borrow_mut(Rc::borrow(bot)).is_live() {
                true
            } else {
                self.map[pos.y][pos.x].bot = None;
                false
            }
        });
        Ok(())
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
        let genom = botc::code_packer::from_b32(&gen_b32).map_err(|_| ())?;
        let cell = self
            .map
            .get_mut(pos.y)
            .ok_or(())?
            .get_mut(pos.x)
            .ok_or(())?;
        if let None = cell.bot {
            let b = Rc::new(RefCell::new(Bot::new(self.colony_cnt, genom)));
            self.colony_cnt = self.colony_cnt + 1;
            self.map[pos.y][pos.x].bot = Some(b.clone());
            self.bots.push((pos, b));
            Ok(())
        } else {
            Err(())
        }
    }
}
