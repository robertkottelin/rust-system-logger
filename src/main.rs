use std::{fs::File, io::Read};

fn main() {
    let mut x = 1;
    loop {
        // read system logs on ubuntu
        let mut file = File::open("/var/log/syslog").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            if line.contains("error") {
                println!("Error found: {}", line);
            } else if line.contains("warning") {
                //println!("Warning found: {}", line);
            }
        }

        // sleep for 1 second
        std::thread::sleep(std::time::Duration::from_secs(1));
        // increment x
        x += 1;

        if x == 2 {
            break;
        }
    }
}
