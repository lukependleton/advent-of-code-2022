// use std::ops::Index;

use colored::Colorize;

use crate::coordinate::Coord;

/// A more general purpose world map that uses world-space coordinates to address its elements
/// This map is presented as a 2d map, but implemented in one vector so that its elements will be in a contiguous place in memory
pub struct WorldMap {
    map: Vec<char>,
    coordinate_offset: Coord,
    height: usize,
    width: usize,
}

impl WorldMap {
    pub fn new(height: usize, width: usize, fill_char: char, coordinate_offset: Coord) -> Self {
        // Create the backing array that will be used for the 2d map
        let map = vec![fill_char; width * height];

        // Create the WorldMap
        WorldMap {
            map,
            coordinate_offset,
            height,
            width,
        }
    }

    pub fn render(&self, color: bool) -> String {
        if color {
            self.map
                .chunks(self.width)
                .enumerate()
                .map(|(j, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(i, c)| {
                            if (i + j) % 2 == 0 {
                                c.to_string().bold().red().to_string()
                            } else {
                                c.to_string().bold().green().to_string()
                            }
                        })
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            self.map
                .chunks(self.width)
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}

/// Basically the Index & IndexMut traits but combining them and returning a Result of the reference instead of the reference itself
// ? Possible future, it might be better to just look into overriding the traits for get/get_mut...😅
pub trait IndexResult<Idx> {
    type OkType;
    type ErrType;
    fn index(&self, index: Idx) -> Result<&Self::OkType, Self::ErrType>;
    fn index_mut(&mut self, index: Idx) -> Result<&mut Self::OkType, Self::ErrType>;
}

impl IndexResult<Coord> for WorldMap {
    type OkType = char;
    // There's only going to be one way that this will error so unit is fine for the error type
    type ErrType = ();

    /// Return a reference to the right element of the map considering the coordinate offset given a world-space coordinate
    fn index(&self, index: Coord) -> Result<&Self::OkType, Self::ErrType> {
        // Get the local coodinate inside the map given the world space (global) coordinate index
        let local_coord = index - self.coordinate_offset;

        // Get the usize equivalents of the index, returning an Error if negative (would be out of bounds)
        let local_x = TryInto::<usize>::try_into(local_coord.x).map_err(|_| ())?;
        let local_y = TryInto::<usize>::try_into(local_coord.y).map_err(|_| ())?;

        // Confirm that the local x and y are in the currect width and height bounds of the 2d map respectively
        let local_x = (0..self.width)
            .contains(&local_x)
            .then_some(local_x)
            .ok_or(())?;
        let local_y = (0..self.height)
            .contains(&local_y)
            .then_some(local_y)
            .ok_or(())?;

        // Return a reference to the appropriate element of the map
        self.map.get(local_y * self.width + local_x).ok_or(())
    }

    /// Return a mutable reference to the right element of the map considering the coordinate offset given a world-space coordinate
    fn index_mut(&mut self, index: Coord) -> Result<&mut Self::OkType, Self::ErrType> {
        // Get the local coodinate inside the map given the world space (global) coordinate index
        let local_coord = index - self.coordinate_offset;

        // Get the usize equivalents of the index, returning an Error if negative (would be out of bounds)
        let local_x = TryInto::<usize>::try_into(local_coord.x).map_err(|_| ())?;
        let local_y = TryInto::<usize>::try_into(local_coord.y).map_err(|_| ())?;

        // Confirm that the local x and y are in the currect width and height bounds of the 2d map respectively
        let local_x = (0..self.width)
            .contains(&local_x)
            .then_some(local_x)
            .ok_or(())?;
        let local_y = (0..self.height)
            .contains(&local_y)
            .then_some(local_y)
            .ok_or(())?;

        // Return a mutable reference to the appropriate element of the map
        self.map.get_mut(local_y * self.width + local_x).ok_or(())
    }
}

impl WorldMap {
    pub fn get_row(&self, y: i32) -> Result<&[char], ()> {
        // Get the local y coodinate inside the map given the world space (global) y index
        let local_y = TryInto::<usize>::try_into(y - self.coordinate_offset.y).map_err(|_| ())?;

        // Confirm that the local y is in the currect height bounds of the 2d map
        let local_y = (0..self.height)
            .contains(&local_y)
            .then_some(local_y)
            .ok_or(())?;

        // Return a reference to the appropriate row of the map
        self.map
            .get((local_y * self.width)..((local_y + 1) * self.width))
            // .map(|row_slice| &row_slice.to_vec())
            .ok_or(())
    }

    /*
    pub fn get_row_mut(&mut self, y: i32) -> Result<&mut [char], ()> {
        // Get the local y coodinate inside the map given the world space (global) y index
        let local_y = TryInto::<usize>::try_into(y - self.coordinate_offset.y).map_err(|_| ())?;

        // Confirm that the local y is in the currect height bounds of the 2d map
        let local_y = (0..self.height).contains(&local_y).then_some(local_y).ok_or(())?;

        // Return a mutable reference to the appropriate row of the map
        self.map
            .get_mut((local_y * self.width)..((local_y + 1) * self.width))
            // .map(|row_slice| &mut row_slice.to_vec())
            .ok_or(())
    }
    */
}
