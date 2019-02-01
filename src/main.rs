use std::io;

fn main() {
    loop {
        println!("Convert temp (e.g 79 F):");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        // trim trailing new line
        temp = String::from(temp.trim());
        let fc = temp.pop();
        let t = temp.trim();

        let temp: f64 = match t.parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match fc {
            Some('F') => {
                let c = (temp - 32.0) / 1.8;
                println!("{} C", c);
            }
            Some('C') => {
                let f = (temp * 1.8) + 32.0;
                println!("{} F", f);
            }
            _ => println!("what?")
        };
    }
}