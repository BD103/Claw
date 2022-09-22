use crate::Type::{self, *};
use paste::paste;

/// If given a value, returns [`Some(value)`](`Some`). Else, returns [`None`].
///
/// This exists so that return arguments are optional in [`create_opcodes`]. There's no way to create if-else statements with `macro_rules`, so this is a workaround.
macro_rules! create_optional {
    ($value:expr) => {
        Some($value)
    };
    () => {
        None
    };
}

macro_rules! create_opcodes {
    (
        $(mod $clmod:ident => $emod:ident => $opmod:ident {
            $($clname:ident => $ename:ident => ($($oparg:expr),*)$(: $opreturn:expr)? => $opname:ident),*
        }),*
    ) => {
        paste! {
            pub enum OpCode {
                // Double recursion so $ename can be unwound as well as $emod
                $(
                    $(
                        [<$emod $ename>]
                    ),*
                ),*
            }

            impl OpCode {
                pub fn code(&self) -> &'static str {
                    match self {
                        $(
                            $(
                                Self::[<$emod $ename>] => stringify!([<$opmod $opname>])
                            ),*
                        ),*
                    }
                }

                pub fn args(&self) -> &'static [Type] {
                    match self {
                        $(
                            $(
                                Self::[<$emod $ename>] => &[$($oparg),*]
                            ),*
                        ),*
                    }
                }

                pub fn returns(&self) -> Option<Type> {
                    match self {
                        $(
                            $(
                                Self::[<$emod $ename>] => create_optional!($($opreturn)?)
                            ),*
                        ),*
                    }
                }
            }
        }
    };
}

create_opcodes!(
    mod motion => Motion => motion_ {
        move_steps => MoveSteps => (Number) => movesteps,
        goto_xy => GotoXY => (Number, Number) => gotoxy,
        turn_right => TurnRight => (Number) => turnright,
        turn_left => TurnLeft => (Number) => turnleft,
        get_x => GetX => (): Number => xposition,
        get_y => GetY => (): Number => yposition
    },
    mod looks => Looks => looks_ {
        say => Say => (Text) => say,
        say_for => SayFor => (Text, Number) => sayforsecs
    }
);
