use super::Type::{self, *};

macro_rules! create_opcodes {
    {
        $($opname:ident => ($($oparg:expr),*): $opreturn:expr => $opcode:ident),*
    } => {
        // #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum OpCode {
            $($opname),*
        }

        impl OpCode {
            pub fn code(&self) -> &'static str {
                match self {
                    $(Self::$opname => stringify!($opcode)),*
                }
            }

            // TODO: maybe make lifetime static?
            pub fn args(&self) -> &[Type] {
                match self {
                    $(Self::$opname => &[$($oparg),*]),*
                }
            }

            pub fn returns(&self) -> Type {
                match self {
                    $(Self::$opname => $opreturn),*
                }
            }
        }
    };
}

create_opcodes! {
    // Motion
    MotionMoveSteps => (Number): Nothing => motion_movesteps,
    MotionGotoXY => (Number, Number): Nothing => motion_gotoxy,
    MotionTurnRight => (Number): Nothing => motion_turnright,
    MotionTurnLeft => (Number): Nothing => motion_turnleft,
    MotionSetX => (Number): Nothing => motion_setx,
    MotionSetY => (Number): Nothing => motion_sety,
    MotionChangeX => (Number): Nothing => motion_changexby,
    MotionChangeY => (Number): Nothing => motion_changeyby,

    MotionGetX => (): Number => motion_xposition,
    MotionGetY => (): Number => motion_yposition,

    // Looks
    LooksSay => (Text): Nothing => looks_say
}
