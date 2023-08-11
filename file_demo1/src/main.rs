use std::{fs::File, io::Read};

fn main(){


    let filename = std::env::args().nth(1).expect("usge:file");

    let mut f = File::open(filename).expect("open file failed");

    let mut buf = [0u8;1024];
    let mut pos = 0;

    loop {
        let n = f.read(&mut buf).expect("read failed");
        if n == 0 {
            break;
        }
        print!("[0x{:02x}] ",pos);
        for bytes in buf{
            match bytes{
                0 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ",bytes)
            }
        }
        pos += n as u32;
        println!();
    }

}