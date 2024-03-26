#[repr(C)]
#[derive(Clone)]
struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn up(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    pub fn down(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    pub fn left(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    pub fn right(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }
}

enum HighwayState {
    LeftLane,
    RightLane,
}

fn highway_state(pos: &Pos) -> Option<HighwayState> {
    use HighwayState::*;
    match pos.x {
        8 => Some(LeftLane),
        9 => Some(RightLane),
        _ => None,
    }
}

fn in_pod_row(grid_pos: &Pos) -> bool {
    (0..9).step_by(2).any(|r| grid_pos.y == r) && grid_pos.x < 6
}

fn is_below_pod_row(grid_pos: &Pos) -> bool {
    in_pod_row(&grid_pos.up())
}

fn available_moves(pos: &Pos, target_pos: &Pos, is_carrying: bool) -> Vec<Pos> {
    let is_in_pod_row = in_pod_row(&pos);
    let is_below_pod_row = is_below_pod_row(&pos);
    let is_in_correct_row = if is_carrying && in_pod_row(target_pos) {
        pos.y == target_pos.y + 1
    } else {
        pos.y == target_pos.y
    };

    use HighwayState::*;
    if is_carrying && is_in_pod_row {
        // When the robot just picked up a pod and is still in the pod location
        vec![pos.down()]
    } else if is_carrying && is_below_pod_row && pos.x == target_pos.x && pos.y == target_pos.y + 1 {
        // When returning a pod and below where pod should go
        vec![pos.up(), pos.right()]
    } else if let Some(highway_state) = highway_state(pos) {
        match highway_state {
            LeftLane if pos.y == target_pos.y && target_pos.x > pos.x => vec![pos.down(), pos.left()],
            LeftLane if is_in_correct_row => vec![pos.left(), pos.down()],
            LeftLane if pos.y == 9 => vec![pos.right()],
            LeftLane if pos.y > target_pos.y => vec![pos.right(), pos.down()],
            LeftLane => vec![pos.down()],
            RightLane if pos.y == 0 => vec![pos.left()],
            RightLane if pos.y < target_pos.y => vec![pos.left(), pos.up()],
            RightLane => vec![pos.up()],
        }
    } else if !is_in_correct_row {
        if is_carrying && is_below_pod_row {
            vec![pos.right()]
        } else {
            vec![pos.right(), pos.up(), pos.down()]
        }
    } else if pos.x > target_pos.x {
        if is_carrying {
            vec![pos.left(), pos.right()]
        } else {
            vec![pos.left(), pos.up(), pos.down()]
        }
    } else if pos.x < target_pos.x {
        vec![pos.right()]
    } else {
        vec![pos.clone()]
    }
}

#[no_mangle]
pub extern fn available_moves_u(pos: Pos, target_pos: Pos, is_carrying: bool, result: &mut [Pos; 3]) {
    let moves = available_moves(&pos, &target_pos, is_carrying);

    for i in 0..3 {
        result[i] = moves.get(i).map(Pos::clone).unwrap_or(Pos::new(-1, -1));
    }
}
