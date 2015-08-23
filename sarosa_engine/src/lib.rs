#![feature(result_expect)]
#![feature(iter_arith)]


#[macro_use] extern crate log;
#[macro_use] extern crate glium;
extern crate sarosa_net;
extern crate glutin;
extern crate cgmath;
extern crate image;
extern crate num;

pub use self::models::settings::Settings;
pub use self::rendering::Window;
pub use self::net::Server;
pub use self::core::GameLoop;
pub use self::core::GameInstance;

pub mod models;

// TODO(Nemikolh):
//  When warning are off, swicth to a public loader.
//pub mod loader;
mod loader;

mod core;
mod net;
mod events;
mod rendering;
mod animation;
mod unit;
mod profiler;
