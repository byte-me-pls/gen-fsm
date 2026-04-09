use crate::drone::{DroneContext, Direction};
use crate::world::World;

pub fn read_context(world: &World, pos: (usize, usize)) -> DroneContext {
    let mut blocked_near = 0; 
    let mut blocked_far = 0;  
    let mut blocked_directions = 0; 

    for dir in &Direction::ALL {
        let mut is_blocked = false;

        if let Some((r1, c1)) = dir.apply(pos.0, pos.1, world.height, world.width) {
            if !world.is_walkable(r1, c1) {
                blocked_near += 1;
                is_blocked = true;
            } else if let Some((r2, c2)) = dir.apply(r1, c1, world.height, world.width) {
                if !world.is_walkable(r2, c2) {
                    blocked_near += 1;
                }
            }
        } else {
            is_blocked = true;
        }

        if let Some((r1, c1)) = dir.apply(pos.0, pos.1, world.height, world.width) {
            if let Some((r2, c2)) = dir.apply(r1, c1, world.height, world.width) {
                if let Some((r3, c3)) = dir.apply(r2, c2, world.height, world.width) {
                    if !world.is_walkable(r3, c3) {
                        blocked_far += 1;
                    }
                }
            }
        }

        if is_blocked {
            blocked_directions += 1;
        }
    }

    if blocked_directions >= 3 {
        DroneContext::DeadEnd
    } else if blocked_near >= 2 {
        DroneContext::ObstacleNear
    } else if blocked_far >= 1 || blocked_near >= 1 {
        DroneContext::ObstacleFar
    } else {
        DroneContext::Clear
    }
}
