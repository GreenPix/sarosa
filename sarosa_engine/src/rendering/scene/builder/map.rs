
// pub struct MapBuilder(Map);
//
// impl MapBuilder {
//
//     pub fn new(width: u32, height: u32) -> MapBuilder {
//         MapBuilder(Map {
//             tile_size: 0,
//             width: width,
//             height: height,
//             objects: unsafe { mem::unintialized() },
//             tiles: SmallVec::new(),
//         })
//     }
//
//     pub fn add_tile()
//
//     pub fn build(self) -> Result<Map, MapCreationError> {
//         let map = self.0;
//         if map.tiles.len() == 0 {
//             return Err(MapCreationError::NoTilesGiven);
//         }
//         for tile in map.tiles.iter() {
//             if map.width * map.height != tile.len() {
//                 return Err(MapCreationError::TileSizeDoNotMatchMapSize)
//             }
//         }
//         Ok(map)
//     }
// }
//
// pub enum MapCreationError {
//     NoTilesGiven,
//     TileSizeDoNotMatchMapSize,
// }
