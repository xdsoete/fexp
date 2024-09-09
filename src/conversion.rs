const PREFIXES: [&str;5] = ["", "k", "M", "G", "T"];

pub fn format_bytes(mut bytes: u64) -> String {
    let mut i = 0;
    while bytes > 1024 && i < PREFIXES.len()-1 {
        bytes = bytes/1024;
        i = i+1;
    }
    format!("{} {}B", bytes, PREFIXES[i])
}
