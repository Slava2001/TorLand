use botc::{code::{Command, Dir, Reg, RwReg}, compiler};


#[test]
fn compile_test() {
    let expect_code: Vec<Command> = vec![
        Command::Nop,
        Command::Mov(Dir::Front),
        Command::Rot(Dir::Right),
        Command::Jmp(0),
        Command::Cmp(Reg::Ax, Reg::Bx),
        Command::Jme(0),
        Command::Jne(0),
        Command::Jmg(0),
        Command::Jml(0),
        Command::Jle(0),
        Command::Jge(0),
        Command::Jmb(0),
        Command::Jnb(0),
        Command::Jmc(0),
        Command::Jnc(0),
        Command::Jmf(0),
        Command::Jnf(0),
        Command::Chk(Dir::Back),
        Command::Cmpv(Reg::Ax, 123),
        Command::Split(Dir::Front, 0),
        Command::Fork(Dir::Front, 0),
        Command::Bite(Dir::Right),
        Command::Eatsun,
        Command::Absorb,
        Command::Call(0),
        Command::Ret,
        Command::Ld(RwReg::Ax, Reg::En),
        Command::Ldv(RwReg::Cx, 321),
        Command::Ldr(3, Reg::Ax),
        Command::Ldm(RwReg::Bx, 4)
    ];
    let text_code: &str = r#"
        start:
        nop
        mov Front
        rot Right
        jmp start
        cmp Ax Bx
        jme start
        jne end
        jmg start
        jml end
        jle start
        jge end
        jmb start
        jnb end
        jmc start
        jnc end
        jmf start
        jnf end
        chk Back
        cmpv Ax 123
        split Front start
        fork Front end
        bite Right
        eatsun
        absorb
        call start
        ret
        ld Ax En
        ldv Cx 321
        ldr [3] Ax
        ldm Bx [4]
        end:
    "#;
    let comtiled = compiler::compile(text_code.into()).unwrap();
    assert_eq!(comtiled, expect_code);
}
