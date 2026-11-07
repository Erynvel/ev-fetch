// EV//Fetch — minimal system fetch. Zero dependencies, Linux only.
use std::{env, fs};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";

struct Logo {
    color: &'static str,
    art: &'static [&'static str],
}

fn logo_for(ids: &[String]) -> Logo {
    for id in ids {
        let l = match id.as_str() {
            "arch" | "archarm" | "endeavouros" | "cachyos" | "artix" | "garuda" | "manjaro" => Logo {
                color: "\x1b[36m",
                art: &[
                    r"      /\      ",
                    r"     /  \     ",
                    r"    /    \    ",
                    r"   /  ,,  \   ",
                    r"  /  |  |  \  ",
                    r" /_-''  ''-_\ ",
                ],
            },
            "debian" => Logo {
                color: "\x1b[31m",
                art: &[
                    r"   _____     ",
                    r"  /  __ \    ",
                    r" |  /    |   ",
                    r" |  \___-    ",
                    r" -_          ",
                    r"   --_       ",
                ],
            },
            "ubuntu" | "linuxmint" | "pop" | "zorin" | "elementary" => Logo {
                color: "\x1b[33m",
                art: &[
                    r"        _    ",
                    r"    ---(_)   ",
                    r"  _/  ---  \ ",
                    r" (_) |   |   ",
                    r"   \  --- _/ ",
                    r"    ---(_)   ",
                ],
            },
            "fedora" | "rhel" | "centos" | "rocky" | "almalinux" => Logo {
                color: "\x1b[34m",
                art: &[
                    r"     _____   ",
                    r"    /   __)\ ",
                    r"    |  /  \ \",
                    r" ___|  |__/ /",
                    r"/ (_    _)_/ ",
                    r"\__(_)(___)  ",
                ],
            },
            "nixos" => Logo {
                color: "\x1b[94m",
                art: &[
                    r"  \\  \\ //   ",
                    r" ==\\__\\/ // ",
                    r"   //   \\//  ",
                    r"==//     //== ",
                    r" //\\___//    ",
                    r"// /\\  \\==  ",
                ],
            },
            "opensuse" | "opensuse-leap" | "opensuse-tumbleweed" | "suse" => Logo {
                color: "\x1b[32m",
                art: &[
                    r"   _______   ",
                    r"__|   __ \   ",
                    r"     / .\ \  ",
                    r"     \__/ |  ",
                    r"   _______|  ",
                    r"   \_______  ",
                ],
            },
            _ => continue,
        };
        return l;
    }
    Logo {
        color: "\x1b[90m",
        art: &[
            r"    .--.     ",
            r"   |o_o |    ",
            r"   |:_/ |    ",
            r"  //   \ \   ",
            r" (|     | )  ",
            r"/'\_   _/`\  ",
            r"\___)=(___/  ",
        ],
    }
}

fn read(p: &str) -> String {
    fs::read_to_string(p).unwrap_or_default()
}

fn os_release(key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    read("/etc/os-release")
        .lines()
        .find_map(|l| l.strip_prefix(&prefix).map(|v| v.trim_matches('"').to_string()))
}

fn uptime() -> String {
    let secs = read("/proc/uptime")
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0) as u64;
    let (d, h, m) = (secs / 86400, secs % 86400 / 3600, secs % 3600 / 60);
    match (d, h) {
        (0, 0) => format!("{m}m"),
        (0, _) => format!("{h}h {m}m"),
        _ => format!("{d}d {h}h {m}m"),
    }
}

fn cpu() -> String {
    read("/proc/cpuinfo")
        .lines()
        .find(|l| l.starts_with("model name"))
        .and_then(|l| l.split(':').nth(1))
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .unwrap_or_else(|| "unknown".into())
}

fn memory() -> String {
    let info = read("/proc/meminfo");
    let get = |k: &str| -> f64 {
        info.lines()
            .find(|l| l.starts_with(k))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0)
            / 1048576.0
    };
    let total = get("MemTotal:");
    format!("{:.1} / {:.1} GiB", total - get("MemAvailable:"), total)
}

fn main() {
    let name = os_release("PRETTY_NAME").unwrap_or_else(|| "Linux".into());
    let mut ids = vec![os_release("ID").unwrap_or_default()];
    ids.extend(
        os_release("ID_LIKE")
            .unwrap_or_default()
            .split_whitespace()
            .map(String::from),
    );

    let logo = logo_for(&ids);
    let c = logo.color;

    let user = env::var("USER").unwrap_or_else(|_| "user".into());
    let host = read("/proc/sys/kernel/hostname").trim().to_string();
    let shell = env::var("SHELL")
        .ok()
        .and_then(|s| s.rsplit('/').next().map(String::from))
        .unwrap_or_default();

    let row = |k: &str, v: String| format!("{c}{BOLD}{k:<7}{RESET}{v}");
    let info = vec![
        format!("{c}{BOLD}EV//Fetch{RESET}"),
        format!("{DIM}{user}@{host}{RESET}"),
        String::new(),
        row("os", name),
        row("kernel", read("/proc/sys/kernel/osrelease").trim().to_string()),
        row("uptime", uptime()),
        row("shell", shell),
        row("cpu", cpu()),
        row("mem", memory()),
    ];

    let width = logo.art.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    println!();
    for i in 0..logo.art.len().max(info.len()) {
        let art = logo.art.get(i).copied().unwrap_or("");
        let pad = width - art.chars().count();
        println!(
            "  {c}{art}{RESET}{:pad$}   {}",
            "",
            info.get(i).map(String::as_str).unwrap_or("")
        );
    }
    println!();
}
