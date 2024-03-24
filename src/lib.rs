use std::collections::VecDeque;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[repr(C)]
struct Pos {
    pub x: i32,
    pub y: i32
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos {
            x,
            y
        }
    }
}

#[repr(C)]
struct Task {
    pub pod_position: Pos,
}

impl Task {
    pub fn new(pod_position: Pos) -> Self {
        Self {
            pod_position
        }
    }
}

lazy_static! {
    static ref TASK_QUEUE: Mutex<VecDeque<Task>> = Mutex::new(VecDeque::new());
}

#[no_mangle]
pub extern fn add_to_queue() {
    let mut queue = TASK_QUEUE.lock().unwrap();

    queue.push_back(Task::new(Pos::new(0, 0)));
}

#[no_mangle]
pub extern fn is_queue_empty() -> bool {
    let queue = TASK_QUEUE.lock().unwrap();

    queue.is_empty()
}

#[no_mangle]
pub extern fn pop_queue() -> Task {
    let mut queue = TASK_QUEUE.lock().unwrap();

    queue.pop_front().unwrap()
}

#[no_mangle]
pub extern fn add(a: i32, b: i32) -> i32 {
    a + b
}

