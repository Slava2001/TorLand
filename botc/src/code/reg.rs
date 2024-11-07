use crate::decl_tokens_enum;

macro_rules! decl_reg {
    (RW: $(($rw_reg_name:literal, $rw_reg:ident)),*, RO: $(($ro_reg_name:literal, $ro_reg:ident)),*) => {
        decl_tokens_enum!{
            Reg,
            $(($rw_reg_name, $rw_reg)),*,
            $(($ro_reg_name, $ro_reg)),*
        }

        decl_tokens_enum!{
            RwReg,
            $(($rw_reg_name, $rw_reg)),*
        }

        impl From<RwReg> for Reg {
            fn from(value: RwReg) -> Self {
                match value {
                    $(RwReg::$rw_reg => Reg::$rw_reg),*
                }
            }
        }
    };
}

decl_reg! {
    RW:
    ("ax", Ax),
    ("bx", Bx),
    ("cx", Cx),
    ("dx", Dx),
    RO:
    ("en", En),
    ("ag", Ag),
    ("sd", Sd),
    ("md", Md)
}
