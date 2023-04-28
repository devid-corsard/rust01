fn main() {
    let mut v = Vec::with_capacity(4);
    for i in 0..3 {
        v.push(i);
    }
    let n = &v[2] as *const i32;
    v.push(4);
    println!("{}", unsafe { *n });
}
// fn main() {
//     let m = 0x012665usize;
//     let r = m as *const i32;
//     unsafe {
//         // println!("{:?}", *r);
//     }
//     let mut num = 5;
//
//     let r1 = &num as *const i32;
//     let r2 = &mut num as *mut i32;
//
//     num = 99999999;
//     unsafe {
//         println!("r1 is: {}", *r1);
//         println!("r2 is: {}", *r2);
//     }
//
//     use std::slice;
//
//     let address = 0x01234usize;
//     let r = address as *mut i32;
//
//     let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
//     // println!("{:?}", values);
//     unsafe {
//         println!("Absolute value of -3 according to C: {}", abs(-3));
//     }
// }
// extern "C" {
//     fn abs(input: i32) -> i32;
// }
