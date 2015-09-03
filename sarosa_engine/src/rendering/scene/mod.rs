
use cgmath::Vector2;
use glium::VertexBuffer;
use glium::index::IndexBuffer;
use glium::texture::Texture2dArray;

use rendering::camera::Camera;

pub mod builder;

// Module related to the `render` calls.
mod render;
// Module related to the `update` calls triggered by the viewport changes
mod update;
// Module related to geometry computations
mod geometry;

/// The game unit is 8 * PixelUnit
/// it is used by the position.
pub type GameUnit = f32;

/// The tile unit is 16 * PixelUnit
/// it is also twice more as the GameUnit.
pub type TileUnit = u32;

/// The PixelUnit is the unit used before
/// applying the camera transformation.
pub type PixelUnit = u32;

/// The world scene is the root of the rendering state.
/// It contains the camera and the map.
pub struct WorldScene {
    camera: Camera,
    map: Map,
}

/// The `Map` object in scene contains all
/// the states about the `Map` to do its
/// rendering.
pub struct Map {

    // The width of the map in tile unit.
    // This value should never change once the Map has been created.
    width: TileUnit,

    // The height of the map in tile unit.
    // This value should never change once the Map has been created.
    height: TileUnit,

    // The viewport of the map storing all the attributes needed
    // to update the VertexBuffer when the camera moves.
    viewport: MapViewport,

    // Object layer. This layer is sorted using the -y axis.
    // It has also special properties.
    objects: ObjectLayer,

    // Basic tiles layers. Usually, the size should be one, that is
    // the ground layer. If there's more they're sorted by depth.
    layers: SmallVec<[TileLayerWithDepth; 2]>,

    // Vertices for the chunk contained inside the viewport
    vertices: VertexBuffer<Vertex>,

    // Indices for the chunk contained inside the viewport
    indices: IndexBuffer<u32>,

    // Chipset used by this map (combined into an array):
    // This imply that all chipset used by the map must be of the
    // same size. I think this is currently the case.
    chipsets_texture: Texture2dArray,

    // The width of each chipset.
    chipset_width: TileUnit,

    // The height of each chipset.
    chipset_height: TileUnit,
}

/// The map viewport represent the part of the map
/// that is rendered on screen. It stores information to detect
/// when the buffers should be updated because of a camera change.
struct MapViewport {
    // The size of a tile. As of today, it should always be 16.
    // This value should never change once the Map has been created.
    tile_size: PixelUnit,

    // The position of the viewport
    x: TileUnit,
    y: TileUnit,

    // Size of the viewport
    width: TileUnit,
    height: TileUnit,
}

const MAP_VIEWPORT_UPDATE_RANGE: u32 = 5;


/// The TileLayer and the depth value associated to it.
/// XXX:
///  - Currently, the depth value is used for depth testing.
///    should we use it at some point or does it bring too much confusion ?
///  - If it turns out we always have only one layer, then we should
///    remove the overhead of having this structure that would then solve nothing.
pub struct TileLayerWithDepth(TileLayer, f32);

/// TileLayer store all the TileId for it and its size is
/// given by the owner. The owner currently can either be of
/// type `Object` or of type `Map`.
/// The geomtry used to render a `TileLayer` is the aggregation
/// of the map's `vertices`, `indices` and this texture buffer.
pub struct TileLayer {
    tex_ids: Vec<TileId>,
    current_tex_buffer: VertexBuffer<TexId>,
}

/// The Object layer contains a list of Object not sorted.
/// Primtive sorting is done using depth testing.
pub struct ObjectLayer {
    objects: Vec<Object>,
}

/// `Object` are conceptually composed of only one TileLayer, and may also have children.
/// In the following struct, the children have been merged into one single `VertexBuffer`.
/// To see the original representation of an Object see `loader::ObjectTree`.
///
/// # Notes:
///
///   * This representation is very generic but has lost the original concept.
///     It's perfectly fine because this representation is designed for the
///     rendering and to support the only changes that can occurs (frame update).
///     In that respect, the data layout is really easy to use.
///
///   * Generating an `Object` directly is hard. The reason lies in the previous comment.
///     If you need to generate one, have a look at the `loader::ObjectTree` class.
///     If an additional feature need to be provided, it is more likely than the change
///     needs to be done on the `loader::ObjectTree` itself.
///
pub struct Object {
    // This information is unique per instance:
    // and the array is sorted by -y axis (no longer applicable)
    inst_attr: VertexBuffer<ObjInstAttr>,
    // Each instance of the object use those vertices:
    vertices: VertexBuffer<VertexObj>,
    // and those indices:
    indices: IndexBuffer<u32>,
    // Store the frames of this objet:
    frames_texture: Texture2dArray,
}

/// Id for a tile. Zero is a reserved value to tell
/// to the shader to discard this tile.
/// The tile id include the id of the chipset and must
/// thus be divided by the chipset size to find the corresponding
pub struct TileId(u32);

const EMPTY_TILE: TileId = TileId(0);

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

#[derive(Copy, Clone)]
struct TexId {
    // Here a value of zero means that we should discard the fragment.
    tex_id: u32,
}

#[derive(Copy, Clone)]
struct VertexObj {
    position: [f32; 2],
    tex_coords: [f32; 2],
    tex_id: u32,
}

#[derive(Copy, Clone)]
struct ObjInstAttr {
    // The depth is computed by the shader using the -y coordinate
    obj_pos: [f32; 2],
    frame_index: u32,
}

implement_vertex!(TexId, tex_id);
implement_vertex!(Vertex, position);
implement_vertex!(VertexObj, position, tex_coords, tex_id);
implement_vertex!(ObjInstAttr, obj_pos, frame_index);
