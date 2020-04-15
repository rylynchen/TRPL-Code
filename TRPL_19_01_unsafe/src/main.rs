fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
        dangerous();
    }

    add_to_counter(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

static mut COUNTER: i32 = 0;

fn add_to_counter(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe fn dangerous() {}