use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;
use std::io;


fn main() {
    let string = read_username_from_file().unwrap();
    println!("{}", string);
    //let f = File::open("hello.txt").unwrap();
    //let f = File::open("hello.txt").expect("cant find file!!");
    let f = File::open("hello.txt");
    let f = match f {
        Ok(f) => f,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => {
                    println!("Created file!");
                    fc
                },
                Err(e) => panic!("Cant create file. {:?}", e)
            }
        },
        Err(error) => panic!("something went wrong {:?}", error)
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut buf = String::new();
    File::open("hello.txt")?.read_to_string(&mut buf)?;
    Ok(buf)
}
