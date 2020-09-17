#[no_mangle]

pub extern "C" fn anagrams(dictionary: *mut u8) -> i32 {
    let list = unsafe {
        std::slice::from_raw_parts_mut(dictionary, 7)
    };

    for l in list {
        println!("{}", l);
    }

    0
}
