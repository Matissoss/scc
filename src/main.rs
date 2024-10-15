fn main() {
    let args : Vec<String> = std::env::args().collect();

    if args.len() < 3{
        return;
    }
    let encrypt_bits : i32 = match &args[2].trim().parse(){
        Ok(val) => {
            *val
        }
        Err (_) =>{
            println!("Invalid Encryption Argument, expected u32 -> 32-bit unsigned integer; couldn't parse to u32");
            return;
        }
    };
    let file = &args[1];
    if file.len() > 4{
        if file[file.len()-4..file.len()] == *".txt"{
            println!("text file detected");
        }
        else{
            println!("Encoded Input is: \n {:?}", encrypt(&file.to_string(), &encrypt_bits));
        }
    }
    else if file.len() == 1{
        println!("Not Enough Args!");
        return;
    }
    else{
        let bits: Vec<i32> = encrypt(&file.to_string(), &encrypt_bits);
        println!("Encoded Input is: \n {bits:?}");
    }
    println!("Encryption key is: {}", -encrypt_bits);
}

fn encrypt(text: &String, bits: &i32) -> Vec<i32>{
    let mut to_return: Vec<i32> = vec![0; text.len()];
    let mut index : usize = 0;
    for c in text.chars(){
        to_return[index] = c as i32 + bits;
        index += 1;
    }
    to_return
}
