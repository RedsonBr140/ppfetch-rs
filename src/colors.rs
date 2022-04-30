// See some ANSI escapes info here: https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
pub const _NONE: &str = "\x1b[0m";

pub mod normal {
    pub const _GREEN: &str = "\x1b[32m";
    pub const _YELLOW: &str = "\x1b[33m";
    pub const _RED: &str = "\x1b[31m";
    pub const _BLUE: &str = "\x1b[34m";
    pub const _PURPLE: &str = "\x1b[35m";
}

pub mod bold {
    pub const _GREEN: &str = "\x1b[1;32m";
    pub const _YELLOW: &str = "\x1b[1;33m";
    pub const _RED: &str = "\x1b[1;31m";
    pub const _BLUE: &str = "\x1b[1;34m";
    pub const _PURPLE: &str = "\x1b[1;35m";

    pub use _BLUE as blue;
    pub use _GREEN as green;
    pub use _PURPLE as purple;
    pub use _RED as red;
    pub use _YELLOW as yellow;
}

pub use self::normal::_BLUE as blue;
pub use self::normal::_GREEN as green;
pub use self::normal::_PURPLE as purple;
pub use self::normal::_RED as red;
pub use self::normal::_YELLOW as yellow;
pub use self::_NONE as reset;
