use crate::Cmd;
pub fn echo(cmd: Cmd){
    let mut i = 0;
    let len =cmd.args.len()-1;
    for arg in cmd.args{
        print!("{arg}");
        if i <len{
print!(" ")
        }
        i+=1;
    }
    println!();
}