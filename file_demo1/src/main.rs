use std::{fs::File, io::Read};


const SIZE:u32 = 10;
fn main(){


    let filename = std::env::args().nth(1).expect("usge:file");

    let mut f = File::open(filename).expect("open file failed");

    let mut buf = [0u8;10];
    let mut pos = 0;
    while let Ok(_) = f.read_exact(&mut buf){
        print!("[0x{:02x}] ",pos);
        for bytes in buf{
            match bytes{
                0 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ",bytes)
            }
        }
        pos += SIZE;
        println!()
    }

}