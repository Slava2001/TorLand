use super::WordAccessor;
use botc::code::{Command, Dir, Reg, Val};

const MAX_COMMANDS_PER_CYCLE: usize = 10;

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
    is_live: bool,
    state: State,
    genom: Vec<Command>,
}

impl Bot {
    pub(super) fn new(genom: Vec<Command>) -> Self {
        Self {
            state: State {
                ..Default::default()
            },
            genom,
            is_live: true,
        }
    }

    pub(super) fn update(&mut self, wa: &mut WordAccessor) {
        if !self.is_live {
            return;
        }

        for _ in 0..MAX_COMMANDS_PER_CYCLE {
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
                Command::Chk(_dir) => {
                    todo!();
                    // break;
                },
                Command::Cmp(reg1, reg2) => {
                    let reg1 = *self.state.get_reg(reg1);
                    let reg2 = *self.state.get_reg(reg2);
                    self.state.fz = reg1 == reg2;
                    self.state.fs = reg1 >= reg2;
                },
                Command::Cmpv(reg, val) => {
                    let reg = *self.state.get_reg(reg);
                    self.state.fz = reg == *val;
                    self.state.fs = reg >= *val;
                },
                Command::Split(dir, lable) => {
                    let mut new = Bot::new(self.genom.clone());
                    new.state = self.state.clone();
                    new.state.pc = *lable;
                    wa.spawn(dir+self.state.dir, new).ok();
                    break;
                },
                Command::Forc(_dir, _) => {
                    todo!();
                    // break;
                },
                Command::Bite(_dir) => {
                    todo!();
                    // break;
                },
                Command::Eatsun => {
                    todo!();
                    // break;
                },
                Command::Absorb => {
                    todo!();
                    // break;
                },
                Command::Call(_) => {
                    todo!();
                },
                Command::Ret => {
                    todo!();
                },
                Command::Load(rw_reg, reg) => {
                    let val = *self.state.get_reg(reg);
                    *self.state.get_reg(&(*rw_reg).into()) = val;
                },
                Command::Loadv(rw_reg, val) =>  {
                    *self.state.get_reg(&(*rw_reg).into()) = *val;
                },
            };
        }
    }

    pub fn is_live(&self) -> bool {
        self.is_live
    }
}
