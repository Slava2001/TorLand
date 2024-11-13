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
    pub sun: usize,
    pub mineral: usize,
    pub bot: Option<BotRef>,
}

pub struct Rules {
    pub max_commands_per_cycle: usize,
    pub energy_for_split: isize,
    pub energy_per_sun: isize,
    pub energy_per_mineral: isize,
    pub energy_per_step: isize,
    pub age_per_energy_penalty: isize,
    pub start_energy: isize,
    pub max_energy: isize,
    pub on_bite_energy_delimiter: isize,
    pub max_randov_value: isize,
    pub mutation_ver: f64
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            max_commands_per_cycle: 10,
            energy_for_split: 1000,
            energy_per_sun: 3,
            energy_per_mineral: 3,
            energy_per_step: 10,
            age_per_energy_penalty: 1000,
            start_energy: 100,
            on_bite_energy_delimiter: 10,
            max_energy: 10_000,
            max_randov_value: 1000,
            mutation_ver: 0.1
        }
    }
}

pub struct WorldConfig<T: FnMut(usize, usize)-> usize, U: FnMut(usize, usize)-> usize> {
    pub h: usize,
    pub w: usize,
    pub rules: Rules,
    pub sun: T,
    pub mineral: U
}

pub struct World {
    size: Vec2u,
    map: Vec<Vec<Cell>>,
    bots: Vec<(Vec2u, BotRef)>,
    colony_cnt: usize,
    rules: Rules
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
    pub fn new<T: FnMut(usize, usize)->usize, U: FnMut(usize, usize)->usize>(mut cfg: WorldConfig<T, U>) -> Self {
        let mut map = Vec::new();
        for y in 0..cfg.h {
            let mut row = Vec::new();
            for x in 0..cfg.w {
                row.push(Cell {
                    sun: (cfg.sun)(x, y),
                    mineral: (cfg.mineral)(x, y),
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
            rules: cfg.rules
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
            RefCell::borrow_mut(Rc::borrow(b)).update(&mut wa, &self.rules)?;
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
