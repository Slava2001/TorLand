use std::{borrow::Borrow, isize, rc::Rc};

use super::{Rules, WorldAccessor};
use botc::code::{Command, Dir, Reg, Val};
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Info {
    pub age: usize,
    pub colony_id: usize,
    pub genom_id: usize,
    pub energy: usize,
    pub genom: Rc<Vec<Command>>,
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "age: {}", self.age)?;
        writeln!(f, "colony_id: {}", self.colony_id)?;
        writeln!(f, "genom_id: {}", self.genom_id)?;
        writeln!(f, "energy: {}", self.energy)?;
        write!(
            f,
            "gemon: {}",
            botc::code_packer::to_b32(self.genom.borrow()).unwrap()
        )
    }
}

const REG_CNT: usize = 8;
#[derive(Debug, Clone)]
struct State {
    // regs
    regs: [Val; REG_CNT],
    // flags
    fs: bool,
    fz: bool,
    ef: bool,
    eb: bool,
    ec: bool,
    // other
    pc: usize,
    dir: Dir,
}

impl Default for State {
    fn default() -> Self {
        Self {
            regs: [0; REG_CNT],
            fs: false,
            fz: false,
            ef: false,
            eb: false,
            ec: false,
            pc: 0,
            dir: Dir::Front,
        }
    }
}

impl State {
    fn get_reg_mut(&mut self, reg: Reg) -> &mut Val {
        &mut self.regs[reg as usize]
    }
    fn get_reg(&self, reg: Reg) -> &Val {
        &self.regs[reg as usize]
    }
}

#[derive(Debug)]
pub struct Bot {
    colony_id: usize,
    genom_id: usize,
    is_live: bool,
    state: State,
    genom: Rc<Vec<Command>>,
}

impl Bot {
    pub(super) fn new(colony_id: usize, genom_id: usize, genom: Rc<Vec<Command>>) -> Self {
        Self {
            colony_id,
            genom_id,
            state: State::default(),
            genom,
            is_live: true,
        }
    }

    pub(super) fn update(&mut self, wa: &mut WorldAccessor, rules: &Rules) -> Result<(), ()> {
        if !self.is_live {
            return Ok(());
        }

        for _ in 0..rules.max_commands_per_cycle {

            let cmd = &self.genom[self.state.pc];
            self.state.pc = (self.state.pc + 1).rem_euclid(self.genom.len());
            match cmd {
                Command::Nop => break,
                Command::Mov(dir) => {
                    wa.mov(dir + self.state.dir).ok();
                    break;
                }
                Command::Rot(dir) => {
                    self.state.dir = self.state.dir + dir;
                    break;
                }
                Command::Jmf(lable) => {
                    if self.state.ef {
                        self.state.pc = *lable
                    }
                }
                Command::Jnf(lable) => {
                    if !self.state.ef {
                        self.state.pc = *lable
                    }
                }
                Command::Jmb(lable) => {
                    if self.state.eb {
                        self.state.pc = *lable
                    }
                }
                Command::Jnb(lable) => {
                    if !self.state.eb {
                        self.state.pc = *lable
                    }
                }
                Command::Jmc(lable) => {
                    if self.state.ec {
                        self.state.pc = *lable
                    }
                }
                Command::Jnc(lable) => {
                    if !self.state.ec {
                        self.state.pc = *lable
                    }
                }
                Command::Chk(dir) => {
                    let dir = *dir + self.state.dir;
                    *self.state.get_reg_mut(Reg::Sd) = wa.get_sun_diff(dir);
                    *self.state.get_reg_mut(Reg::Md) = wa.get_mineral_diff(dir);
                    let (ef, ec) = match wa.is_some_colony(dir, self.colony_id) {
                        Some(is_some) => (false, is_some),
                        None => (true, false),
                    };
                    self.state.ef = ef;
                    self.state.eb = !ef;
                    self.state.ec = ec;
                    break;
                }
                Command::Cmp(reg1, reg2) => {
                    let reg1 = *self.state.get_reg(*reg1);
                    let reg2 = *self.state.get_reg(*reg2);
                    self.state.fz = reg1 == reg2;
                    self.state.fs = reg1 >= reg2;
                }
                Command::Cmpv(reg, val) => {
                    let reg = *self.state.get_reg(*reg);
                    self.state.fz = reg == *val;
                    self.state.fs = reg >= *val;
                }
                Command::Split(dir, lable) => {
                    let energy = *self.state.get_reg(Reg::En);
                    *self.state.get_reg_mut(Reg::En) = energy - rules.energy_for_split;
                    if energy > rules.energy_for_split {
                        let mut new = Bot::new(self.colony_id, self.genom_id, self.genom.clone());
                        new.state = self.state.clone();
                        new.state.pc = *lable;
                        *new.state.get_reg_mut(Reg::Ag) = 0;
                        *new.state.get_reg_mut(Reg::En) = rules.energy_for_split;
                        if let Err(_) = wa.spawn(dir + self.state.dir, new) {
                            self.kill();
                        }
                    }
                    break;
                }
                Command::Fork(dir, lable) => {
                    let energy = *self.state.get_reg(Reg::En);
                    *self.state.get_reg_mut(Reg::En) = energy - rules.energy_for_split;
                    if energy > rules.energy_for_split {
                        let mut new = Bot::new(self.colony_id, self.genom_id, self.genom.clone());
                        new.state = self.state.clone();
                        new.state.pc = *lable;
                        *new.state.get_reg_mut(Reg::Ag) = 0;
                        *new.state.get_reg_mut(Reg::En) = rules.energy_for_split;
                        new.colony_id = wa.get_new_colony_id();
                        if thread_rng().gen_bool(rules.mutation_ver) {
                            let mut genom = Vec::clone(self.genom.borrow());
                            let index = thread_rng().gen_range(0..new.genom.len());
                            let command = Command::rand(
                                &mut thread_rng(),
                                new.genom.len(),
                                rules.max_random_value,
                            );
                            genom[index] = command;
                            new.genom = Rc::new(genom);
                            new.genom_id = wa.get_new_genom_id();
                        }
                        if let Err(_) = wa.spawn(dir + self.state.dir, new) {
                            self.kill();
                        }
                    }
                    break;
                }
                Command::Bite(dir) => {
                    let energy = *self.state.get_reg(Reg::En);
                    *self.state.get_reg_mut(Reg::En) = energy
                        + wa.kill(dir + self.state.dir).unwrap_or(0)
                            / rules.on_bite_energy_delimiter;
                    break;
                }
                Command::Eatsun => {
                    const DIRS: [Dir; 8] = [
                        Dir::Front,
                        Dir::FrontRight,
                        Dir::Right,
                        Dir::BackRight,
                        Dir::Back,
                        Dir::BackLeft,
                        Dir::Left,
                        Dir::FrontLeft,
                    ];
                    let mut free_cnt = 0;
                    let mut bro_cnt = 0;

                    for d in DIRS {
                        match wa.is_some_colony(d, self.colony_id) {
                            Some(is_some) => {
                                if is_some {
                                    bro_cnt += 1;
                                }
                            }
                            None => free_cnt += 1,
                        };
                    }

                    let energy = wa.get_sun() * rules.energy_per_sun
                        + rules.energy_per_sun_bro_boost * bro_cnt
                        + rules.energy_per_sun_free_boost * free_cnt
                        + rules.energy_per_sun_oth_boost * (DIRS.len() as isize - free_cnt);

                    *self.state.get_reg_mut(Reg::En) = energy + *self.state.get_reg(Reg::En);
                    break;
                }
                Command::Absorb => {
                    let energy = *self.state.get_reg(Reg::En);
                    *self.state.get_reg_mut(Reg::En) =
                        energy + wa.get_mineral() * rules.energy_per_mineral;
                    break;
                }
                Command::Call(_) => {
                    // todo!();
                }
                Command::Ret => {
                    // todo!();
                }
                Command::Ld(rw_reg, reg) => {
                    let val = *self.state.get_reg(*reg);
                    *self.state.get_reg_mut((*rw_reg).into()) = val;
                }
                Command::Ldv(rw_reg, val) => {
                    *self.state.get_reg_mut((*rw_reg).into()) = *val;
                }
                Command::Jmp(lable) => {
                    self.state.pc = *lable;
                }
                Command::Jme(lable) => {
                    if self.state.fz {
                        self.state.pc = *lable;
                    }
                }
                Command::Jne(lable) => {
                    if !self.state.fz {
                        self.state.pc = *lable;
                    }
                }
                Command::Jmg(lable) => {
                    if self.state.fs && !self.state.fz {
                        self.state.pc = *lable;
                    }
                }
                Command::Jml(lable) => {
                    if !self.state.fs && !self.state.fz {
                        self.state.pc = *lable;
                    }
                }
                Command::Jle(lable) => {
                    if !self.state.fs || self.state.fz {
                        self.state.pc = *lable;
                    }
                }
                Command::Jge(lable) => {
                    if self.state.fs || self.state.fz {
                        self.state.pc = *lable;
                    }
                }
                Command::Ldr(_, _reg) => {},
                Command::Ldm(_rw_reg, _) => {},
            };
        }

        let age = *self.state.get_reg(Reg::Ag) + 1;
        *self.state.get_reg_mut(Reg::Ag) = age;

        let energy = (*self.state.get_reg(Reg::En)
            - rules.energy_per_step
            - age / rules.age_per_energy_penalty)
            .min(rules.max_energy);
        *self.state.get_reg_mut(Reg::En) = energy;

        self.is_live = self.is_live && energy > 0;

        if self.is_live {
            self.set_energy(wa.energy_diffusion(self.colony_id, self.get_energy()));
        }

        Ok(())
    }

    pub(super) fn is_live(&self) -> bool {
        self.is_live
    }

    pub(super) fn kill(&mut self) -> isize {
        self.is_live = false;
        *self.state.get_reg(Reg::En)
    }

    pub(super) fn get_colony(&self) -> usize {
        self.colony_id
    }

    pub(super) fn get_energy(&self) -> isize {
        *self.state.get_reg(Reg::En)
    }

    pub(super) fn set_energy(&mut self, en: isize) {
        *self.state.get_reg_mut(Reg::En) = en
    }

    pub fn get_info(&self) -> Info {
        Info {
            age: *self.state.get_reg(Reg::Ag) as usize,
            colony_id: self.colony_id,
            genom_id: self.genom_id,
            energy: *self.state.get_reg(Reg::En) as usize,
            genom: self.genom.clone(),
        }
    }
}
