pub fn clear(a:Vec<String>){
if a.len() != 0{
    eprintln!("clear doesnt work with options");
    return;
}
 print!("\x1B[2J\x1B[3J\x1B[H")
}
