
use toml::Value;

use std::fs;
pub fn get_generated_entities(level: u32) -> [crate::world::Entity] {
    match level {
	0 => {
	    let mut entities = [];
	}
	1 => {
	    
	}
    }
}
pub fn get_generated_level(level: u32) -> crate::world::Level {
    
    match level {
	
	0 => {
	    let mut map_content = [
	  [1,1,1,1, 1,1,1,1, 1,1,1,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [1,2,2,2, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [1,2,2,2, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [1,2,2,2, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],

	  [1,2,2,2, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [1,2,2,2, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [1,2,2,2, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [1,2,2,2, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],

	  [1,2,2,2, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [1,2,2,2, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [1,2,2,2, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [1,1,1,1, 1,1,1,1, 1,1,1,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],

	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],

	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],

	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],

	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],

	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0]];
	    crate::world::Level {
	    
	    map_size: 32,
	    content: map_content
	    }},
	1 => {
	    
	    let mut map_content = [
	  [1,1,1,1, 1,1,1,1, 1,1,1,1, 1,1,1,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [1,2,2,2, 2,2,2,2, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [1,2,2,2, 2,2,2,2, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [1,1,1,1, 1,1,1,1, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],

	  [0,0,0,0, 0,0,0,1, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,1, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,1, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,1, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],

	  [0,0,0,0, 0,0,0,1, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,1, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,1, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,1, 2,2,2,2, 2,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],

	  [0,0,0,0, 0,0,0,1, 2,2,2,0, 0,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,1, 2,2,2,0, 0,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,1, 2,2,2,0, 0,2,2,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,1, 1,1,1,1, 1,1,1,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],

	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],

	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],

	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],

	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
	  [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0]];
	    crate::world::Level {
	    
	    map_size: 32,
	    content: map_content
	}},
	_ => {

	    let map_content = [[1;32];32];
	    crate::world::Level {
	    
	    map_size: 32,
	    content: map_content
	}}
    }}

pub fn get_station_map() -> crate::world::Level {

    let mut level = crate::world::Level {
	map_size: 32,
	content: [[1;32];32]
    };
    return level;
}
