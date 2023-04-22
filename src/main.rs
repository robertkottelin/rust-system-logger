use std::{fs::File, io::Read};

fn main() {
    loop {
        // read system logs on ubuntu
        let mut file = File::open("/var/log/syslog").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("With text:\n{}", contents);

        // sleep for 1 second
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

}
