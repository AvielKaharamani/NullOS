
pub fn strlen(buff: &[char; 80]) -> usize {
    let mut len = 0;

    for i in 0..80 {
        if buff[i] == '\0' {
            break;
        }
        len += 1;
    }
    
    len
}

pub fn strcmp(first: &[char; 80], second: &[char; 80]) -> i32 {
    use core::cmp::min;
    let mut len = min(strlen(first), strlen(second));

    for i in 0..len {
        if first[i] > second[i] {
            return 1;
        } else if second[i] > first[i] {
            return -1;
        }
    }

    0
}

pub fn print_string(buff: &[char; 80]) {
    let len = strlen(buff);

    for i in 0..len {
        print!("{}", buff[i]);
    }
}

pub fn print_string_new_line(buff: &[char; 80]) {
    print_string(buff);
    println!("");
}