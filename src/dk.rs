use crate::prelude::*;
#[cfg(feature = "threaded")]
use rayon::prelude::*;
#[allow(unused_imports)]
use std::collections::VecDeque;
use std::convert::TryInto;
use std::f32::MAX;

pub fn build_dwino(dm: &mut DijkstraMap, starts: &[usize], map: &dyn BaseMap) {
    // let threaded = DijkstraMap::build_helper(dm, starts, map);
    //     if threaded == RunThreaded::True {
    //         return;
    //     }
    let mapsize: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
    let mut open_list: VecDeque<(usize, f32)> = VecDeque::with_capacity(mapsize);

    let mut ix = 0.0;
    for start in starts {
        open_list.push_back((*start, ix));
        ix -= 1000.0;
    }

    while let Some((tile_idx, depth)) = open_list.pop_front() {
        let exits = map.get_available_exits(tile_idx);
        for (new_idx, add_depth) in exits {
            let new_depth = depth + add_depth;
            let prev_depth = dm.map[new_idx];
            if new_depth >= prev_depth {
                continue;
            }
            if new_depth >= 1024.0 {
                continue;
            }
            dm.map[new_idx] = new_depth;
            open_list.push_back((new_idx, new_depth));
        }
    }
}
