use super::{Rules, WorldAccessor};
use botc::code::{Command, Dir, Label, Mem, Reg, Val};
use rand::{thread_rng, Rng};
use std::{borrow::Borrow, isize, rc::Rc};

#[derive(Debug)]
pub struct Info {
    pub colony_id: usize,
    pub genom_id: usize,
    pub genom: Rc<Vec<Command>>,
    pub reg_ax: Val,
    pub reg_bx: Val,
    pub reg_cx: Val,
    pub reg_dx: Val,
    pub reg_en: Val,
    pub reg_ag: Val,
    pub reg_sd: Val,
    pub reg_md: Val,
    pub flag_fs: bool,
    pub flag_fz: bool,
    pub flag_fo: bool,
    pub flag_ef: bool,
    pub flag_eb: bool,
    pub flag_ec: bool,
    pub pc: usize,
    pub sp: usize,
    pub dir: Dir,
    pub ram: Vec<Val>,
    pub stack: Vec<Label>,
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Bot Info:")?;
        writeln!(
            f,
            "genom: {}",
            botc::code_packer::to_b32(self.genom.borrow()).unwrap()
        )?;
        writeln!(f, "colony_id: {}", self.colony_id)?;
        writeln!(f, "genom_id: {}", self.genom_id)?;
        writeln!(f, "Regs:")?;
        writeln!(f, "Ax: {}", self.reg_ax)?;
        writeln!(f, "Bx: {}", self.reg_bx)?;
        writeln!(f, "Cx: {}", self.reg_cx)?;
        writeln!(f, "Dx: {}", self.reg_dx)?;
        writeln!(f, "En: {}", self.reg_en)?;
        writeln!(f, "Ag: {}", self.reg_ag)?;
        writeln!(f, "Sd: {}", self.reg_sd)?;
        writeln!(f, "Md: {}", self.reg_md)?;
        writeln!(f, "Flags:")?;
        writeln!(f, "fs: {}", self.flag_fs)?;
        writeln!(f, "fz: {}", self.flag_fz)?;
        writeln!(f, "fo: {}", self.flag_fo)?;
        writeln!(f, "ef: {}", self.flag_ef)?;
        writeln!(f, "eb: {}", self.flag_eb)?;
        writeln!(f, "ec: {}", self.flag_ec)?;
        writeln!(f, "Other:")?;
        writeln!(f, "pc: {}", self.pc)?;
        writeln!(f, "sp: {}", self.sp)?;
        writeln!(f, "dir: {}", self.dir)?;
        writeln!(f, "ram: {:?}", &self.ram[..])?;
        writeln!(f, "stack: {:?}", &self.stack[..(self.sp)])
    }
}

const REG_CNT: usize = 8;
#[derive(Debug, Clone)]
pub struct State {
    // regs
    regs: [Val; REG_CNT],
    // flags
    fs: bool,
    fz: bool,
    fo: bool,
    ef: bool,
    eb: bool,
    ec: bool,
    // other
    pc: usize,
    sp: usize,
    dir: Dir,
    ram: Vec<Val>,
    stack: Vec<Label>,
}

impl State {
    fn new(ram_size: usize, stack_size: usize, energy: Val) -> Self {
        let mut state = Self {
            regs: [0; REG_CNT],
            fs: false,
            fz: false,
            fo: false,
            ef: false,
            eb: false,
            ec: false,
            pc: 0,
            sp: 0,
            dir: Dir::Front,
            ram: vec![0; ram_size],
            stack: vec![0; stack_size],
        };
        state.set_reg(Reg::En, energy);
        state
    }

    fn set_reg(&mut self, reg: Reg, val: Val) {
        self.regs[reg as usize] = val;
    }

    fn get_reg(&self, reg: Reg) -> Val {
        self.regs[reg as usize]
    }

    fn get_ram(&self, addr: Mem) -> Val {
        self.ram[addr as usize]
    }

    fn set_ram(&mut self, addr: Mem, val: Val) {
        self.ram[addr as usize] = val;
    }

    fn push(&mut self, val: Label) -> Result<(), ()> {
        if self.sp >= self.stack.len() {
            return Err(());
        }
        self.stack[self.sp] = val;
        self.sp += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<Label, ()> {
        if self.sp <= 0 {
            return Err(());
        }
        self.sp -= 1;
        Ok(self.stack[self.sp])
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

pub struct BotCfg {
    pub colony_id: usize,
    pub genom_id: usize,
    pub genom: Rc<Vec<Command>>,
    pub ram_size: usize,
    pub stack_size: usize,
    pub energy: Val,
}

impl Bot {
    pub(super) fn new(cfg: BotCfg) -> Self {
        Self {
            colony_id: cfg.colony_id,
            genom_id: cfg.genom_id,
            state: State::new(cfg.ram_size, cfg.stack_size, cfg.energy),
            genom: cfg.genom,
            is_live: true,
        }
    }

    pub(super) fn update(&mut self, wa: &mut WorldAccessor, rules: &Rules) -> Result<(), ()> {
        if !self.is_live {
            return Ok(());
        }

        for _ in 0..rules.max_commands_per_cycle {
            if self.do_command(wa, rules) {
                break;
            }
        }

        let age = self.state.get_reg(Reg::Ag) + 1;
        self.state.set_reg(Reg::Ag, age);

        let energy = (self.state.get_reg(Reg::En)
            - rules.energy_per_step
            - age / rules.age_per_energy_penalty)
            .min(rules.max_energy);
        self.state.set_reg(Reg::En, energy);

        self.is_live = self.is_live && energy > 0;

        if self.is_live {
            self.set_energy(wa.energy_diffusion(self.colony_id, self.get_energy()));
        }

        Ok(())
    }

    fn do_command(&mut self, wa: &mut WorldAccessor, rules: &Rules) -> bool {
        let cmd = &self.genom[self.state.pc];
        self.state.pc = (self.state.pc + 1).rem_euclid(self.genom.len());
        match cmd {
            Command::Nop => return true,
            Command::Mov(dir) => {
                wa.mov(dir + self.state.dir).ok();
                return true;
            }
            Command::Rot(dir) => {
                self.state.dir = self.state.dir + dir;
                return true;
            }
            Command::Jmf(label) => {
                if self.state.ef {
                    self.state.pc = *label
                }
            }
            Command::Jnf(label) => {
                if !self.state.ef {
                    self.state.pc = *label
                }
            }
            Command::Jmb(label) => {
                if self.state.eb {
                    self.state.pc = *label
                }
            }
            Command::Jnb(label) => {
                if !self.state.eb {
                    self.state.pc = *label
                }
            }
            Command::Jmc(label) => {
                if self.state.ec {
                    self.state.pc = *label
                }
            }
            Command::Jnc(label) => {
                if !self.state.ec {
                    self.state.pc = *label
                }
            }
            Command::Chk(dir) => {
                let dir = *dir + self.state.dir;
                self.state.set_reg(Reg::Sd, wa.get_sun_diff(dir));
                self.state.set_reg(Reg::Md, wa.get_mineral_diff(dir));
                let (ef, ec) = match wa.is_some_colony(dir, self.colony_id) {
                    Some(is_some) => (false, is_some),
                    None => (true, false),
                };
                self.state.ef = ef;
                self.state.eb = !ef;
                self.state.ec = ec;
                return true;
            }
            Command::Cmp(reg1, reg2) => {
                let reg1 = self.state.get_reg(*reg1);
                let reg2 = self.state.get_reg(*reg2);
                self.state.fz = reg1 == reg2;
                self.state.fs = reg1 >= reg2;
            }
            Command::Cmpv(reg, val) => {
                let reg = self.state.get_reg(*reg);
                self.state.fz = reg == *val;
                self.state.fs = reg >= *val;
            }
            Command::Split(dir, label) => {
                let energy = self.state.get_reg(Reg::En);
                self.state.set_reg(Reg::En, energy - rules.energy_for_split);
                if energy > rules.energy_for_split {
                    let mut new = Bot {
                        colony_id: self.colony_id,
                        genom_id: self.genom_id,
                        is_live: true,
                        state: self.state.clone(),
                        genom: self.genom.clone(),
                    };
                    new.state.pc = *label;
                    new.state.set_reg(Reg::Ag, 0);
                    new.state.set_reg(Reg::En, rules.energy_for_split);
                    if let Err(_) = wa.spawn(dir + self.state.dir, new) {
                        self.kill();
                    }
                }
                return true;
            }
            Command::Fork(dir, label) => {
                let energy = self.state.get_reg(Reg::En);
                self.state.set_reg(Reg::En, energy - rules.energy_for_split);
                if energy > rules.energy_for_split {
                    let mut new = Bot {
                        colony_id: self.colony_id,
                        genom_id: self.genom_id,
                        is_live: true,
                        state: self.state.clone(),
                        genom: self.genom.clone(),
                    };
                    new.state.pc = *label;
                    new.state.set_reg(Reg::Ag, 0);
                    new.state.set_reg(Reg::En, rules.energy_for_split);
                    new.colony_id = wa.get_new_colony_id();
                    if thread_rng().gen_bool(rules.mutation_ver) {
                        let mut genom = Vec::clone(self.genom.borrow());
                        let index = thread_rng().gen_range(0..new.genom.len());
                        let command = Command::rand(
                            &mut thread_rng(),
                            new.genom.len(),
                            rules.max_random_value,
                            rules.ram_size as isize,
                        );
                        genom[index] = command;
                        new.genom = Rc::new(genom);
                        new.genom_id = wa.get_new_genom_id();
                    }
                    if let Err(_) = wa.spawn(dir + self.state.dir, new) {
                        self.kill();
                    }
                }
                return true;
            }
            Command::Bite(dir) => {
                let energy = self.state.get_reg(Reg::En);
                self.state.set_reg(
                    Reg::En,
                    energy
                        + wa.kill(dir + self.state.dir).unwrap_or(0)
                            / rules.on_bite_energy_delimiter,
                );
                return true;
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

                self.state
                    .set_reg(Reg::En, energy + self.state.get_reg(Reg::En));
                return true;
            }
            Command::Absorb => {
                let energy = self.state.get_reg(Reg::En);
                self.state.set_reg(
                    Reg::En,
                    energy + wa.get_mineral() * rules.energy_per_mineral,
                );
                return true;
            }
            Command::Ld(rw_reg, reg) => {
                let val = self.state.get_reg(*reg);
                self.state.set_reg((*rw_reg).into(), val);
            }
            Command::Ldv(rw_reg, val) => {
                self.state.set_reg((*rw_reg).into(), *val);
            }
            Command::Jmp(label) => {
                self.state.pc = *label;
            }
            Command::Jme(label) => {
                if self.state.fz {
                    self.state.pc = *label;
                }
            }
            Command::Jne(label) => {
                if !self.state.fz {
                    self.state.pc = *label;
                }
            }
            Command::Jmg(label) => {
                if self.state.fs && !self.state.fz {
                    self.state.pc = *label;
                }
            }
            Command::Jml(label) => {
                if !self.state.fs && !self.state.fz {
                    self.state.pc = *label;
                }
            }
            Command::Jle(label) => {
                if !self.state.fs || self.state.fz {
                    self.state.pc = *label;
                }
            }
            Command::Jge(label) => {
                if self.state.fs || self.state.fz {
                    self.state.pc = *label;
                }
            }
            Command::Jmo(label) => {
                if self.state.fo {
                    self.state.pc = *label;
                }
            }
            Command::Jno(label) => {
                if !self.state.fo {
                    self.state.pc = *label;
                }
            }
            Command::Ldr(mem, reg) => {
                self.state.set_ram(*mem, self.state.get_reg(*reg));
            }
            Command::Ldm(rw_reg, mem) => {
                self.state
                    .set_reg((*rw_reg).into(), self.state.get_ram(*mem));
            }
            Command::Neg(rw_reg) => {
                let reg = (*rw_reg).into();
                self.state.set_reg(reg, -self.state.get_reg(reg));
            }
            Command::Add(rw_reg, reg) => {
                let res = self
                    .state
                    .get_reg((*rw_reg).into())
                    .checked_add(self.state.get_reg(*reg));
                self.state.fo = res.is_none();
                self.state
                    .set_reg((*rw_reg).into(), res.unwrap_or(rules.max_random_value));
            }
            Command::Addv(rw_reg, val) => {
                let res = self.state.get_reg((*rw_reg).into()).checked_add(*val);
                self.state.fo = res.is_none();
                self.state
                    .set_reg((*rw_reg).into(), res.unwrap_or(rules.max_random_value));
            }
            Command::Sub(rw_reg, reg) => {
                let res = self
                    .state
                    .get_reg((*rw_reg).into())
                    .checked_sub(self.state.get_reg(*reg));
                self.state.fo = res.is_none();
                self.state
                    .set_reg((*rw_reg).into(), res.unwrap_or(rules.max_random_value));
            }
            Command::Subv(rw_reg, val) => {
                let res = self.state.get_reg((*rw_reg).into()).checked_sub(*val);
                self.state.fo = res.is_none();
                self.state
                    .set_reg((*rw_reg).into(), res.unwrap_or(rules.max_random_value));
            }
            Command::Mul(rw_reg, reg) => {
                let res = self
                    .state
                    .get_reg((*rw_reg).into())
                    .checked_mul(self.state.get_reg(*reg));
                self.state.fo = res.is_none();
                self.state
                    .set_reg((*rw_reg).into(), res.unwrap_or(rules.max_random_value));
            }
            Command::Mulv(rw_reg, val) => {
                let res = self.state.get_reg((*rw_reg).into()).checked_mul(*val);
                self.state.fo = res.is_none();
                self.state
                    .set_reg((*rw_reg).into(), res.unwrap_or(rules.max_random_value));
            }
            Command::Div(rw_reg, reg) => {
                let res = self
                    .state
                    .get_reg((*rw_reg).into())
                    .checked_div(self.state.get_reg(*reg));
                self.state.fo = res.is_none();
                self.state
                    .set_reg((*rw_reg).into(), res.unwrap_or(rules.max_random_value));
            }
            Command::Divv(rw_reg, val) => {
                let res = self.state.get_reg((*rw_reg).into()).checked_div(*val);
                self.state.fo = res.is_none();
                self.state
                    .set_reg((*rw_reg).into(), res.unwrap_or(rules.max_random_value));
            }
            Command::Mod(rw_reg, reg) => {
                let res = self
                    .state
                    .get_reg((*rw_reg).into())
                    .checked_rem(self.state.get_reg(*reg));
                self.state.fo = res.is_none();
                self.state
                    .set_reg((*rw_reg).into(), res.unwrap_or(rules.max_random_value));
            }
            Command::Modv(rw_reg, val) => {
                let res = self.state.get_reg((*rw_reg).into()).checked_rem(*val);
                self.state.fo = res.is_none();
                self.state
                    .set_reg((*rw_reg).into(), res.unwrap_or(rules.max_random_value));
            }
            Command::Pow(rw_reg, reg) => {
                let res = self
                    .state
                    .get_reg((*rw_reg).into())
                    .checked_pow(self.state.get_reg(*reg) as u32);
                self.state.fo = res.is_none();
                self.state
                    .set_reg((*rw_reg).into(), res.unwrap_or(rules.max_random_value));
            }
            Command::Powv(rw_reg, val) => {
                let res = self
                    .state
                    .get_reg((*rw_reg).into())
                    .checked_pow(*val as u32);
                self.state.fo = res.is_none();
                self.state
                    .set_reg((*rw_reg).into(), res.unwrap_or(rules.max_random_value));
            }
            Command::Call(label) => {
                if self.state.push(self.state.pc).is_ok() {
                    self.state.pc = *label;
                } else {
                    self.state.fo = true;
                }
            }
            Command::Ret => {
                if let Ok(label) = self.state.pop() {
                    self.state.pc = label;
                } else {
                    self.state.fo = true;
                }
            }
        }
        return false;
    }

    pub(super) fn is_live(&self) -> bool {
        self.is_live
    }

    pub(super) fn kill(&mut self) -> isize {
        self.is_live = false;
        self.state.get_reg(Reg::En)
    }

    pub(super) fn get_colony(&self) -> usize {
        self.colony_id
    }

    pub(super) fn get_energy(&self) -> isize {
        self.state.get_reg(Reg::En)
    }

    pub(super) fn set_energy(&mut self, en: isize) {
        self.state.set_reg(Reg::En, en);
    }

    pub fn get_info(&self) -> Info {
        Info {
            colony_id: self.colony_id,
            genom_id: self.genom_id,
            genom: self.genom.clone(),
            reg_ax: self.state.get_reg(Reg::Ax),
            reg_bx: self.state.get_reg(Reg::Bx),
            reg_cx: self.state.get_reg(Reg::Cx),
            reg_dx: self.state.get_reg(Reg::Dx),
            reg_en: self.state.get_reg(Reg::En),
            reg_ag: self.state.get_reg(Reg::Ag),
            reg_sd: self.state.get_reg(Reg::Sd),
            reg_md: self.state.get_reg(Reg::Md),
            flag_fs: self.state.fs,
            flag_fz: self.state.fz,
            flag_fo: self.state.fo,
            flag_ef: self.state.ef,
            flag_eb: self.state.eb,
            flag_ec: self.state.ec,
            pc: self.state.pc,
            sp: self.state.sp,
            dir: self.state.dir,
            ram: self.state.ram.clone(),
            stack: self.state.stack.clone(),
        }
    }
}
