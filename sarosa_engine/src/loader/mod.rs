
use cgmath::Vector2;
use std::rc::Rc;

use rendering::scene::TileUnit;
use rendering::scene::GameUnit;


// Re-export for doc.
pub use core::loops::deferred::DeferredLoader;

/// Trait to define a loader that could be loaded
/// using the `DeferredLoader`.
///
/// TODO(Nemikolh): Add also an interface to do
/// incremental loading with reports after each step.
/// (Can be usefull to display a percentage or what is being
/// currently made).
///
/// Probably Something like:
/// ```ignore
/// struct NextResourceInfo {
///     name: String,
///     progress: f32,
/// }
/// // To add to the trait Loader.
/// fn next_resource_kind() -> NextResourceInfo;
/// fn load_next_resource();
/// ```
pub trait Loader: Send {

    type Resources: Send;

    /// This function is supposed to do
    /// all the loading for R. Here, you should
    /// be doing long operation like reading a file,
    /// and so on.
    fn load_resources(&mut self) -> Self::Resources;
}

/// This is the closest representation of an Object
/// as intended originally in the design.
/// After converting an `ObjectTree` to an `Object`,
/// we should generate:
///     - A texture array containing the different frame of the object
///     - A vertex buffer containing 4 vertices
///     - A set of position for each of the instance of `ObjectTree`
///       that share the same `ObjectNode`.
///
/// Notes:
///
///  * `self.data.position` should always be (0, 0)
pub struct ObjectTree {
    data: Rc<ObjectNode>,
    obj_pos: Vector2<GameUnit>,
}

pub struct ObjectNode {
    width: TileUnit,
    height: TileUnit,
    position: Vector2<GameUnit>,
    tex_coords: [Vector2<TileUnit>; 4],
    children: Vec<ObjectNode>,
}
