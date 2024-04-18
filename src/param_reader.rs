pub fn read_properties(args: Vec<String>) -> String {
    if !args.is_empty() && args.len() >= 3 {
        for i in 1..args.len() {
            if args[i - 1].contains("-properties") {
                return args[i].to_owned();
            }
        }
    }
    "".to_owned()
}

pub fn read_rom_path(args: Vec<String>) -> String {
    if !args.is_empty() && args.len() >= 3 {
        for i in 1..args.len() {
            if args[i - 1].contains("-rom") {
                return args[i].to_owned();
            }
        }
    }
    "".to_owned()
}

