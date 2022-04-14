use colorful::{Color, Colorful};
use rand::Rng;
use std::env;
use std::process::Command;
use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let os = sys.name().unwrap();
    let kernel = sys.kernel_version().unwrap();
    let shell = env::var("SHELL").unwrap();
    let random: u32 = rand::thread_rng().gen_range(1..=2);
    let user = env::var("USER").unwrap().color(Color::DeepPink1a);
    let hostname = sys.host_name().unwrap().color(Color::Blue).bold();
    let mut charcount = env::var("USER").unwrap().chars().count();
    charcount += sys.host_name().unwrap().chars().count();
    let mut line = "─".to_owned();
    let wm = Command::new("sh")
        .arg("-c")
        .arg("wmctrl -m | awk ' /Name/ {print $2}'")
        .output()
        .expect("wmctrl not installed.");
    let uptime = Command::new("sh")
        .arg("-c")
        .arg("uptime -p | sed -n '1p' | sed 's/up //'")
        .output()
        .expect("lol");
    let mut i: usize = 0;

    while i < charcount {
        line += "─";
        i += 1;
    } // increases the line size until it equals the number of characters of user + hostname

    if random < 2 {
        println!(
            "             {}@{}
   (\\_/)     {}
 __(. .)__   {} {}
 \\__|_|__/   {} {}
    / \\      {} {}             {} {}
             {} {}
 Talking is {}, show me the {}", // Sorry for this workaround, idk a better way to do that for now.
            user,
            hostname,
            line,
            "os:".color(Color::Red).bold(),
            os,
            "kernel:".color(Color::Blue).bold(),
            kernel,
            "uptime:".color(Color::DeepPink1a),
            String::from_utf8_lossy(&uptime.stdout),
            "shell:".color(Color::Yellow).bold(),
            shell,
            "wm:".color(Color::Green).bold(),
            String::from_utf8_lossy(&wm.stdout),
            "easy".color(Color::Green).bold(),
            "code".color(Color::Red).bold()
        );
    } else {
        println!(
            "             {}@{}
  __   __    {}
 (  \\,/  )   {} {}
  \\_ | _/    {} {}
  (_/ \\_)    {} {}             {} {}
             {} {}
 Talking is {}, show me the {}",
            user,
            hostname,
            line,
            "os:".color(Color::Red).bold(),
            os,
            "kernel:".color(Color::Blue).bold(),
            kernel,
            "uptime:".color(Color::DeepPink1a),
            String::from_utf8_lossy(&uptime.stdout),
            "shell:".color(Color::Yellow).bold(),
            shell,
            "wm:".color(Color::Green).bold(),
            String::from_utf8_lossy(&wm.stdout),
            "easy".color(Color::Green).bold(),
            "code".color(Color::Red).bold()
        );
    }
}
