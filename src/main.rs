mod colors;
use rand::Rng;
use std::env;
use std::process::Command;
use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    const RESET: &str = colors::reset;
    let osd = format!("{}os:{RESET}", colors::bold::red);
    let kerneld = format!("{}kernel:{RESET}", colors::bold::blue);
    let uptd = format!("{}uptime:{RESET}", colors::bold::purple);
    let shd = format!("{}shell:{RESET}", colors::bold::yellow);
    let wmd = format!("{}wm:{RESET}", colors::bold::green);
    let easy = format!("{}easy{RESET}", colors::bold::green);
    let code = format!("{}code{RESET}", colors::bold::red);
    let user = format!("{}{}{RESET}", colors::purple, env::var("USER").unwrap());
    let hostname = format!("{}{}{RESET}", colors::bold::blue, sys.host_name().unwrap());
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
            "             {user}@{hostname}
   (\\_/)     {line}
 __(. .)__   {osd} {os}
 \\__|_|__/   {kerneld} {kernel}
    / \\      {uptd} {uptime}
             {shd} {shell}
             {wmd} {}
 Talking is {easy}, show me the {code}",
            String::from_utf8_lossy(&wm.stdout)
        );
    } else {
        println!(
            "             {user}@{hostname}
  __   __    {line}
 (  \\,/  )   {osd} {os}
  \\_ | _/    {kerneld} {kernel}
  (_/ \\_)    {uptd} {uptime}
             {shd} {shell}
             {wmd} {}
 Talking is {easy}, show me the {code}",
            String::from_utf8_lossy(&wm.stdout)
        );
    }
}
