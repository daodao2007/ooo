use std::process::Command;
use std::env;
fn openWindows(args:Vec<String>)
{
    if(args.len()>1)
        {
            let path =args[1].clone();
            Command::new("explorer.exe")
                .args(&[path])
                .output()
                .expect("failed to execute process");
        }
        else
        {
            Command::new("explorer.exe")
                .args(&["."])
                .output()
                .expect("failed to execute process");
        }

}
fn main() {
    let args: Vec<String> = env::args().collect();
    //let mut absolute_path = try!(std::env::current_dir());
    println!("{:?}", args);
    if cfg!(target_os = "windows") {
        openWindows(args);
    //println!("Hello, world!");
    }
}
