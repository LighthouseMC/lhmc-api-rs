pub mod system;

pub mod plot;

pub mod player;

pub mod util;


mod data;
pub use data::*;


mod alloc;


pub use lhmc_api_rs_macro as macros;


pub mod prelude {
    pub use crate::system::sleep;
    pub use crate::plot::Plot;
    pub use crate::player::Player;
    pub use crate::util::sanitise_text;
    pub use crate::data::SoundCat;
    pub use lhmc_api_rs_macro::*;
}
