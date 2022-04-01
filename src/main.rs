use colorful::{Color, Colorful};
use std::env;
use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();
    let user = env::var("USER").unwrap().color(Color::DeepPink1a);
    let hostname = sys.host_name().unwrap().color(Color::Blue).bold();
    let mut charcount = env::var("USER").unwrap().chars().count();
    let mut line = "─".to_owned();
    let mut i = 0;
    charcount = charcount + sys.host_name().unwrap().chars().count();
    while i < charcount {
        line = line + "─";
        i += 1;
    }
    let os = sys.name().unwrap();
    let kernel = sys.kernel_version().unwrap();
    let shell = env::var("SHELL").unwrap();
    println!(
        "             {}@{}
   (\\_/)     {}
 __(. .)__   {} {}
 \\__|_|__/   {} {} 
    / \\      {} {} 

 Talking is {}, show me the {}",
        user,
        hostname,
        line,
        "os:".color(Color::Red).bold(),
        os,
        "kernel:".color(Color::Blue).bold(),
        kernel,
        "shell:".color(Color::Yellow).bold(),
        shell,
        "easy".color(Color::Green).bold(),
        "code".color(Color::Red).bold()
    );
}