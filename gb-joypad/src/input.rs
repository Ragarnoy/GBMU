use serde::{Deserialize, Serialize};

// /// The list of input supported by the gameboy.
// pub enum InputType {
//     Up,
//     Down,
//     Left,
//     Right,
//     Start,
//     Select,
//     B,
//     A,
// }

// const INPUT_LIST: [InputType; 8] = [
//     InputType::Up,
//     InputType::Down,
//     InputType::Left,
//     InputType::Right,
//     InputType::Start,
//     InputType::Select,
//     InputType::B,
//     InputType::A,
// ];

macro_rules! make_enum {
    (
        $name:ident $array:ident {
            $( $variant:ident, )*
        }
    ) => {
        #[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
        pub enum $name {
            $( $variant, )*
        }
        pub const $array: &[$name] = &[
            $( $name::$variant, )*
            ];
        }
    }

make_enum!(InputType INPUT_LIST {
    Up, Down, Left, Right, Start, Select, A, B,
});
