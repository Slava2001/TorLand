use super::{Rules, WordAccessor};
use botc::code::{Command, Dir, Reg, Val};
use rand::{thread_rng, Rng};

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
    fn get_reg(&mut self, reg: &Reg) -> &mut Val {
        &mut self.regs[*reg as usize]
    }
}

#[derive(Debug)]
pub struct Bot {
    colony: usize,
    is_live: bool,
    state: State,
    genom: Vec<Command>,
}

impl Bot {
    pub(super) fn new(colony: usize, genom: Vec<Command>) -> Self {
        Self {
            colony,
            state: State {
                ..Default::default()
            },
            genom,
            is_live: true,
        }
    }

    pub(super) fn update(&mut self, wa: &mut WordAccessor, rules: &Rules) -> Result<(), ()> {
        if !self.is_live {
            return Ok(());
        }
        if *self.state.get_reg(&Reg::En) == 0 {
            *self.state.get_reg(&Reg::En) = rules.start_energy;
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
                Command::Jmp(lable) => self.state.pc = *lable,
                Command::Jmg(lable) => {
                    if self.state.fs {
                        self.state.pc = *lable
                    }
                }
                Command::Jnl(lable) => {
                    if !self.state.fs {
                        self.state.pc = *lable
                    }
                }
                Command::Jme(lable) => {
                    if self.state.fz {
                        self.state.pc = *lable
                    }
                }
                Command::Jne(lable) => {
                    if !self.state.fz {
                        self.state.pc = *lable
                    }
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
                Command::Jge(lable) => {
                    if self.state.fz || self.state.fs {
                        self.state.pc = *lable
                    }
                }
                Command::Jle(lable) => {
                    if self.state.fz || !self.state.fs {
                        self.state.pc = *lable
                    }
                }
                Command::Chk(dir) => {
                    let dir = *dir + self.state.dir;
                    *self.state.get_reg(&Reg::Sd) = wa.get_sun_diff(dir);
                    *self.state.get_reg(&Reg::Md) = wa.get_mineral_diff(dir);
                    let (ef, ec) = match wa.is_some_colony(dir, self.colony) {
                        Some(is_some) => (false, is_some),
                        None => (true, false),
                    };
                    self.state.ef = ef;
                    self.state.eb = !ef;
                    self.state.ec = ec;
                    break;
                }
                Command::Cmp(reg1, reg2) => {
                    let reg1 = *self.state.get_reg(reg1);
                    let reg2 = *self.state.get_reg(reg2);
                    self.state.fz = reg1 == reg2;
                    self.state.fs = reg1 >= reg2;
                }
                Command::Cmpv(reg, val) => {
                    let reg = *self.state.get_reg(reg);
                    self.state.fz = reg == *val;
                    self.state.fs = reg >= *val;
                }
                Command::Split(dir, lable) => {
                    let energy = *self.state.get_reg(&Reg::En);
                    *self.state.get_reg(&Reg::En) = energy - rules.energy_for_split;
                    if energy > rules.energy_for_split {
                        let mut new = Bot::new(self.colony, self.genom.clone());
                        new.state = self.state.clone();
                        new.state.pc = *lable;
                        *new.state.get_reg(&Reg::Ag) = 0;
                        wa.spawn(dir + self.state.dir, new).ok();
                    }
                    break;
                }
                Command::Forc(dir, lable) => {
                    let energy = *self.state.get_reg(&Reg::En);
                    *self.state.get_reg(&Reg::En) = energy - rules.energy_for_split;
                    if energy > rules.energy_for_split {
                        let mut new = Bot::new(self.colony, self.genom.clone());
                        new.state = self.state.clone();
                        new.state.pc = *lable;
                        *new.state.get_reg(&Reg::Ag) = 0;
                        new.colony = wa.get_new_colony_id();
                        if thread_rng().gen_bool(rules.mutation_ver) {
                            let index = new.genom.len();
                            new.genom[thread_rng().gen_range(0..index)] = Command::rand(&mut thread_rng(), new.genom.len(), rules.max_randov_value);
                        }
                        wa.spawn(dir + self.state.dir, new).ok();
                    }
                    break;
                }
                Command::Bite(dir) => {
                    let energy = *self.state.get_reg(&Reg::En);
                    *self.state.get_reg(&Reg::En) =
                        energy + wa.kill(*dir).unwrap_or(0) / rules.on_bite_energy_delimiter;
                    break;
                }
                Command::Eatsun => {
                    let energy = *self.state.get_reg(&Reg::En);
                    *self.state.get_reg(&Reg::En) = energy + wa.get_sun() * rules.energy_per_sun;
                    break;
                }
                Command::Absorb => {
                    let energy = *self.state.get_reg(&Reg::En);
                    *self.state.get_reg(&Reg::En) =
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
                    let val = *self.state.get_reg(reg);
                    *self.state.get_reg(&(*rw_reg).into()) = val;
                }
                Command::Ldv(rw_reg, val) => {
                    *self.state.get_reg(&(*rw_reg).into()) = *val;
                }
            };
        }

        let age = *self.state.get_reg(&Reg::Ag) + 1;
        *self.state.get_reg(&Reg::Ag) = age;

        let energy = (*self.state.get_reg(&Reg::En)
            - rules.energy_per_step
            - age / rules.age_per_energy_penalty)
            % rules.max_energy;
        *self.state.get_reg(&Reg::En) = energy;

        self.is_live = energy > 0;

        Ok(())
    }

    pub(super) fn is_live(&self) -> bool {
        self.is_live
    }

    pub(super) fn kill(&mut self) -> isize {
        self.is_live = false;
        *self.state.get_reg(&Reg::En)
    }

    pub fn get_colony(&self) -> usize {
        self.colony
    }
}
