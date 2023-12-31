use std::{collections::HashMap, io::Write};

fn main() {
    let mut want = String::new();
    print!("Input: ");
    let mut stdout = std::io::stdout();
    stdout.flush().unwrap();
    std::io::stdin().read_line(&mut want).unwrap();

    let want = want.trim().chars().map(|c| c as u8).collect::<Vec<_>>();
    
    let start = u8::MIN;
    let end = u8::MAX;

    let mut lock = stdout.lock();
    for i in start..=end {
        let mut msg = HashMap::new();
        for j in start..=end {
            let res = i ^ j;
            if want.contains(&res) {
                msg.insert(res as char, format!(r"{:?} ^ {:?}", i as char, j as char));
            }
        }
        for c in want.iter() {
            let msg = if msg.contains_key(&(*c as char)) {
                msg[&(*c as char)].as_str()
            } else {
                "Not found"
            };

            writeln!(lock, "\t{} :   {}", *c as char, msg).unwrap();
        }
        writeln!(lock, "-------------------------------------------------").unwrap();
    }
}
