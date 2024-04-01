use rand::{Rng, thread_rng};

#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
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

#[no_mangle]
pub extern fn random_pod(robot_tasks: *const Pos, robot_count: i32, task_queue: *const Pos, task_queue_length: i32, task_queue_start: i32, task_queue_end: i32, result: &mut [Pos; 1]) {
    let robot_tasks = unsafe {
      std::slice::from_raw_parts(robot_tasks, robot_count as usize)
    };
    let task_queue = unsafe {
        std::slice::from_raw_parts(task_queue, task_queue_length as usize)
    };

    // Find all pods that have already been assigned to a robot
    let robot_tasks: Vec<_> = robot_tasks
        .iter()
        .filter(|p| p != &&Pos::new(-1, -1)) // A position of (-1, -1) means that no task has been assigned to that robot
        .collect();

    // Find all pods that have a task already in the queue
    let is_index_in_queue = |i: &i32| if task_queue_start < task_queue_end {
        (task_queue_start..task_queue_end).contains(i)
    } else {
        (0..task_queue_end).contains(i) || (task_queue_start..task_queue_length).contains(i)
    };
    let queue_tasks: Vec<_> = (0..task_queue_length)
        .filter(is_index_in_queue)
        .map(|i| &task_queue[i as usize])
        .collect();

    // Get all pods except for those already in the queue or assigned to a robot
    let available_pods: Vec<_> = all_pods()
        .into_iter()
        .filter(|p| !robot_tasks.contains(&p) && !queue_tasks.contains(&p))
        .collect();

    // Select a random pod from our filtered list
    let mut rng = thread_rng();
    let random_pod = &available_pods[rng.gen_range(0..available_pods.len())];

    result[0].x = random_pod.x;
    result[0].y = random_pod.y;
}

#[no_mangle]
pub extern fn available_moves_u(pos: Pos, target_pos: Pos, is_carrying: bool, result: &mut [Pos; 3]) {
    let moves = available_moves(&pos, &target_pos, is_carrying);

    for i in 0..3 {
        result[i] = moves.get(i).map(Pos::clone).unwrap_or(Pos::new(-1, -1));
    }
}

fn available_moves(pos: &Pos, target_pos: &Pos, is_carrying: bool) -> Vec<Pos> {
    let is_in_pod_row = in_pod_row(&pos);
    let is_below_pod_row = is_below_pod_row(&pos);
    let is_in_correct_row = if is_carrying && in_pod_row(target_pos) {
        pos.y == target_pos.y + 1
    } else {
        pos.y == target_pos.y
    };

    if is_carrying && is_in_pod_row {
        // When the robot just picked up a pod and is still in the pod location
        vec![pos.down()]
    } else if is_carrying && is_below_pod_row && pos.x == target_pos.x && pos.y == target_pos.y + 1 {
        // When returning a pod and below where pod should go
        vec![pos.up(), pos.right()]
    } else if let Some(highway_state) = highway_state(pos) {
        use HighwayState::*;
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

fn pod_row(r: i32) -> Vec<Pos> {
    (0..6).map(|c| Pos::new(c, r)).collect()
}

fn all_pods() -> Vec<Pos> {
    (0..9).step_by(2).flat_map(pod_row).collect()
}