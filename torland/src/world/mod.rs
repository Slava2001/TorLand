use botc::code::Dir;
use serde::Deserialize;
use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    mem::swap,
    rc::Rc,
    usize,
};

pub mod bot;

use crate::vec2::Vec2u;
use bot::Bot;

type BotRef = Rc<RefCell<Bot>>;

#[derive(Debug)]
pub struct Cell {
    pub sun: usize,
    pub mineral: usize,
    pub bot: Option<BotRef>,
}

#[derive(Deserialize)]
pub struct Rules {
    pub max_commands_per_cycle: usize,
    pub energy_for_split: isize,
    pub energy_per_mineral: isize,
    pub energy_per_step: isize,
    pub age_per_energy_penalty: isize,
    pub start_energy: isize,
    pub max_energy: isize,
    pub on_bite_energy_delimiter: isize,
    pub max_random_value: isize,
    pub mutation_ver: f64,
    pub energy_per_sun: isize,
    pub energy_per_sun_free_boost: isize,
    pub energy_per_sun_bro_boost: isize,
    pub energy_per_sun_oth_boost: isize,
}

#[derive(Clone, Copy)]
pub struct Info {
    pub h: usize,
    pub w: usize,
    pub max_sun: usize,
    pub max_mineral: usize,
    pub max_age: usize,
    pub min_age: usize,
    pub max_energy: usize,
    pub min_energy: usize,
}

pub struct WorldConfig<T: FnMut(usize, usize) -> usize, U: FnMut(usize, usize) -> usize> {
    pub h: usize,
    pub w: usize,
    pub rules: Rules,
    pub sun: T,
    pub mineral: U,
}

pub struct World {
    size: Vec2u,
    map: Vec<Vec<Cell>>,
    bots: Vec<(Vec2u, BotRef)>,
    colony_cnt: usize,
    genom_cnt: usize,
    rules: Rules,
    info: Info,
}

struct WorldAccessor<'a> {
    pos: &'a mut Vec2u,
    map_size: Vec2u,
    map: &'a mut Vec<Vec<Cell>>,
    newborn: &'a mut Vec<(Vec2u, BotRef)>,
    colony_cnt: &'a mut usize,
    genom_cnt: &'a mut usize,
}

impl<'a> WorldAccessor<'_> {
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

    pub fn get_new_genom_id(&mut self) -> usize {
        let rc = *self.genom_cnt;
        *self.genom_cnt = *self.genom_cnt + 1;
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

    fn energy_diffusion(&self, colony: usize, mut energy: isize) -> isize {
        let mut bro = Vec::new();
        for d in [Dir::Front, Dir::Right, Dir::Back, Dir::Left] {
            let pos = self.pos.mod_add(d.into(), self.map_size);
            if let Some(b) = &self.map[pos.y][pos.x].bot {
                if RefCell::borrow(Rc::borrow(b)).get_colony() == colony {
                    bro.push(b);
                    energy += RefCell::borrow(Rc::borrow(b)).get_energy();
                }
            }
        }
        energy = energy / (bro.len() + 1) as isize;
        for b in bro.into_iter() {
            RefCell::borrow_mut(Rc::borrow(b)).set_energy(energy);
        }
        energy
    }
}

impl World {
    pub fn new<T: FnMut(usize, usize) -> usize, U: FnMut(usize, usize) -> usize>(
        mut cfg: WorldConfig<T, U>,
    ) -> Self {
        let mut max_sun: usize = 0;
        let mut max_mineral: usize = 0;
        let mut map = Vec::new();
        for y in 0..cfg.h {
            let mut row = Vec::new();
            for x in 0..cfg.w {
                let c = Cell {
                    sun: (cfg.sun)(x, y),
                    mineral: (cfg.mineral)(x, y),
                    bot: None,
                };
                max_sun = max_sun.max(c.sun);
                max_mineral = max_mineral.max(c.mineral);
                row.push(c);
            }
            map.push(row);
        }
        Self {
            size: (cfg.w, cfg.h).into(),
            map,
            bots: Vec::new(),
            colony_cnt: 0,
            genom_cnt: 1,
            info: Info {
                h: cfg.h,
                w: cfg.w,
                max_age: usize::MAX,
                min_age: 0,
                max_sun,
                max_mineral,
                max_energy: usize::MAX,
                min_energy: 0,
            },
            rules: cfg.rules,
        }
    }

    pub fn update(&mut self) -> Result<(), ()> {
        self.info.max_age = 0;
        self.info.max_energy = 0;
        self.info.min_age = usize::MAX;
        self.info.min_energy = usize::MAX;

        let mut newborn: Vec<(Vec2u, BotRef)> = Vec::new();
        for (pos, b) in self.bots.iter_mut() {
            let mut wa = WorldAccessor {
                newborn: &mut newborn,
                pos: pos,
                map_size: self.size,
                map: &mut self.map,
                colony_cnt: &mut self.colony_cnt,
                genom_cnt: &mut self.genom_cnt,
            };
            RefCell::borrow_mut(Rc::borrow(b)).update(&mut wa, &self.rules)?;
            let info = RefCell::borrow(Rc::borrow(b)).get_info();
            self.info.max_age = self.info.max_age.max(info.age);
            self.info.max_energy = self.info.max_age.max(info.energy);
            self.info.min_age = self.info.min_age.min(info.age);
            self.info.min_energy = self.info.min_energy.min(info.energy);
        }

        for (_, b) in newborn.iter_mut() {
            let info = RefCell::borrow(Rc::borrow(b)).get_info();
            self.info.max_age = self.info.max_age.max(info.age);
            self.info.max_energy = self.info.max_age.max(info.energy);
            self.info.min_age = self.info.min_age.min(info.age);
            self.info.min_energy = self.info.min_energy.min(info.energy);
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

    pub fn spawn(&mut self, pos: Vec2u, gen_b32: &str) -> Result<(), ()> {
        let genom = botc::code_packer::from_b32(gen_b32).map_err(|_| ())?;
        let cell = self
            .map
            .get_mut(pos.y)
            .ok_or(())?
            .get_mut(pos.x)
            .ok_or(())?;
        if let None = cell.bot {
            let mut b = Bot::new(self.colony_cnt, 0, Rc::new(genom));
            b.set_energy(self.rules.start_energy);
            let b = Rc::new(RefCell::new(b));
            self.colony_cnt = self.colony_cnt + 1;
            self.map[pos.y][pos.x].bot = Some(b.clone());
            self.bots.push((pos, b));
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_info(&self) -> Info {
        self.info
    }

    pub fn get_bot_info(&self, pos: Vec2u) ->  Result<bot::Info, ()> {
        let cell = self
        .map
        .get(pos.y)
        .ok_or(())?
        .get(pos.x)
        .ok_or(())?;
        let Some(b) = &cell.bot else {
            return Err(());
        };
        Ok(RefCell::borrow(Rc::borrow(b)).get_info())
    }
}
