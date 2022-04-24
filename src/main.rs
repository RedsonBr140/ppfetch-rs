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

    let shell = match env::var("SHELL") {
        Ok(var) => {
            let split: Vec<_> = var.split('/').collect();
            match split.last() {
                Some(sh) => sh.to_string(),
                _ => "unknown".to_string(),
            }
        }
        _ => "unknown".to_string(),
    };
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
    let uptime = format!("{}s", sys.uptime());
    let mut i: usize = 0;

    while i < charcount {
        line += "─";
        i += 1;
    } // increases the line size until it equals the number of characters of user + hostname

    if random < 2 {
        println!(
"             {}@{}
   (\\_/)     {}
 __(. .)__   {osd} {}
 \\__|_|__/   {kerneld} {}
    / \\      {uptd} {}
             {shd} {}
             {wmd} {}
 Talking is {easy}, show me the {code}",
            user,
            hostname,
            line,
            os,
            kernel,
            uptime,
            shell,
            String::from_utf8_lossy(&wm.stdout),
            osd ="os:".color(Color::Red).bold(),
            kerneld = "kernel:".color(Color::Blue).bold(),
            uptd = "uptime:".color(Color::DeepPink1a),
            shd = "shell:".color(Color::Yellow).bold(),
            wmd = "wm:".color(Color::Green).bold(),
            easy = "easy".color(Color::Green).bold(),
            code = "code".color(Color::Red).bold()
        );
    } else {
        println!(
"             {}@{}
  __   __    {}
 (  \\,/  )   {osd} {}
  \\_ | _/    {kerneld} {}
  (_/ \\_)    {uptd} {}
             {shd} {}
             {wmd} {}
 Talking is {easy}, show me the {code}",
            user,
            hostname,
            line,
            os,
            kernel,
            uptime,
            shell,
            String::from_utf8_lossy(&wm.stdout),
            osd ="os:".color(Color::Red).bold(),
            kerneld = "kernel:".color(Color::Blue).bold(),
            uptd = "uptime:".color(Color::DeepPink1a),
            shd = "shell:".color(Color::Yellow).bold(),
            wmd = "wm:".color(Color::Green).bold(),
            easy = "easy".color(Color::Green).bold(),
            code = "code".color(Color::Red).bold()
        );
    }
}
