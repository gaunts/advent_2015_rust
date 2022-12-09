use md5;

pub fn run_part(_input: &String, _part: u8) {    
    let mut i:u64 = 0;

    loop {
        let str = _input.to_owned() + &i.to_string();
        let md5 = md5::compute(&str);

        if md5.starts_with(&[0, 0, 0]) {
            println!("{} - {} - {:x?}", str, i, md5.0);
            break;
        }
        i = i + 1;
    }
}

pub fn run(_input: &String) {
    run_part(_input, 1);
    // run_part(_input, 2);
}
