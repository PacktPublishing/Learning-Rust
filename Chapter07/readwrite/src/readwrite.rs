pub struct RWData {
    pub X: i32,
    Y: i32,
}

static mut rwdata: RWData = RWData { X: 0, Y: 0 };

pub fn store_y(val: i32) {
    unsafe {
        rwdata.Y = val;
    }
}

pub fn new_y() -> i32 {
    unsafe {
        rwdata.Y * 6
    }
}
