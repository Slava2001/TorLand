use crate::decl_tokens_enum;

decl_tokens_enum! {
    Dir,
    // Order is important here
    ("front",      Front      ),
    ("frontright", FrontRight ),
    ("right",      Right      ),
    ("backright",  BackRight  ),
    ("back",       Back       ),
    ("backleft",   BackLeft   ),
    ("left",       Left       ),
    ("frontleft",  FrontLeft  )
}
