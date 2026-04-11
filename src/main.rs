#![no_std]
#![no_main]

mod fs;
mod vga_buffer;

use core::panic::PanicInfo;
use core::str::from_utf8;
use x86_64::instructions::port::Port;
use core::sync::atomic::{AtomicBool, Ordering};



use vga_buffer::Color;
static DOWNLOAD_ANIMATION: AtomicBool = AtomicBool::new(false);


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\nPANIC: {}", info);
    loop { x86_64::instructions::hlt(); }
}
fn simulate_download(package: &str) {
    println!("Downloading {}...", package);
    
    let frames = ["|", "/", "-", "\\"];
    let mut frame_idx = 0;
    
    DOWNLOAD_ANIMATION.store(true, Ordering::Relaxed);
    
    for i in 0..=20 {
        let progress = i * 5;
        let bar_width = 20;
        let filled = (i * bar_width) / 20;
        let empty = bar_width - filled;
        
        print!("\r[");
        for _ in 0..filled {
            print!("=");
        }
        if i < 20 {
            print!(">");
            for _ in 0..empty-1 {
                print!(" ");
            }
        } else {
            for _ in 0..empty {
                print!("=");
            }
        }
        print!("] {}% ", progress);
        print!("{} ", frames[frame_idx]);
        
        frame_idx = (frame_idx + 1) % frames.len();
        
        for _ in 0..3000000 {
            x86_64::instructions::nop();
        }
    }
    
    DOWNLOAD_ANIMATION.store(false, Ordering::Relaxed);
    println!("\r[====================] 100% Complete!");
    println!("Package {} installed successfully!", package);
}

fn simulate_download_simple(package: &str) {
    println!("Fetching {} from repository...", package);
    println!("Connecting to repo.reeos.org..."); // это пока что будет просто заглушка
    
    for _ in 0..5 {
        print!(".");
        for _ in 0..5000000 {
            x86_64::instructions::nop();
        }
    }
    println!(" Connected!");
    
    println!("Resolving dependencies...");
    for _ in 0..3 {
        print!(".");
        for _ in 0..3000000 {
            x86_64::instructions::nop();
        }
    }
    println!(" Done!");
    
    println!("");
    println!("Downloading: [                    ] 0%");
    
    for i in 1..=10 {
        let filled = i * 2;
        print!("\x1B[1A"); 
        print!("\x1B[2K"); 
        print!("Downloading: [");
        for _ in 0..filled {
            print!("=");
        }
        print!(">");
        for _ in 0..(20 - filled) {
            print!(" ");
        }
        println!("] {}%", i * 10);
        
        for _ in 0..4000000 {
            x86_64::instructions::nop();
        }
    }
    
    println!("");
    println!("Extracting files...");
    for _ in 0..3 {
        print!(".");
        for _ in 0..3000000 {
            x86_64::instructions::nop();
        }
    }
    println!(" Done!");
    
    println!("Configuring {}...", package);
    for _ in 0..2 {
        print!(".");
        for _ in 0..2000000 {
            x86_64::instructions::nop();
        }
    }
    println!(" Done!");
    
    println!("");
    println!(" {} v1.0.0 installed successfully!", package);
}
fn package_manager() {
    println!("ReeOS Package Manager (RPM)");
    println!("Available packages:");
    println!("  editor  - Simple text editor");
    println!("  game    - Snake game");
    println!("  calc    - Scientific calculator");
    println!("Type 'install <name>' to install");
    println!("Type 'exit' to return\n");
    
    loop {
        print!("rpm> ");
        let input = vga_buffer::read_line_with_echo();
        
        let cmd = match from_utf8(&input) {
            Ok(s) => s.trim_end_matches('\0').trim(),
            Err(_) => "",
        };
        
        if cmd == "exit" {
            println!("Exiting package manager...");
            break;
        }
        
        if cmd == "list" {
            println!("Available packages:");
            println!("  editor  - Text editor (1.2MB)");
            println!("  game    - Snake game (512KB)");
            println!("  calc    - Calculator (256KB)");
            println!("  fetch   - System fetch (128KB)");
            continue;
        }
        
        if cmd.starts_with("install ") {
            let package = &cmd[8..];
            match package {
                "editor" => {
                    vga_buffer::set_color(Color::Cyan, Color::Black);
                    simulate_download("editor");
                    vga_buffer::reset_color();
                    println!("Use 'edit' command to launch editor");
                }
                "game" => {
                    vga_buffer::set_color(Color::Green, Color::Black);
                    simulate_download_simple("snake");
                    vga_buffer::reset_color();
                    println!("Use 'snake' command to play");
                }
                "calc" => {
                    vga_buffer::set_color(Color::Yellow, Color::Black);
                    simulate_download("calculator");
                    vga_buffer::reset_color();
                }
                "fetch" => {
                    vga_buffer::set_color(Color::Magenta, Color::Black);
                    simulate_download_simple("neofetch");
                    vga_buffer::reset_color();
                }
                _ => {
                    vga_buffer::set_color(Color::Red, Color::Black);
                    println!("Package '{}' not found", package);
                    vga_buffer::reset_color();
                }
            }
        } else if !cmd.is_empty() {
            println!("Unknown command: '{}'. Use 'list' or 'install <name>'", cmd);
        }
    }
}



fn clear_screen() {
    for _ in 0..25 { println!(); }
}

fn calculator() {
    println!("ReeOS Calculator");
    println!("Enter expression (e.g., '2=2', '10-5')");
    println!("Type 'exit' to return\n");
    
    loop {
        print!("calc> ");
        let input = vga_buffer::read_line_with_echo();
        
        let expr = match from_utf8(&input) {
            Ok(s) => s.trim_end_matches('\0').trim(),
            Err(_) => "",
        };
        
        if expr == "exit" {
            println!("Returning to shell...");
            break;
        }
        
        if expr.is_empty() { continue; }
        
        if let Some(pos) = expr.find('=') {
            let a = expr[..pos].trim().parse::<i32>().unwrap_or(0);
            let b = expr[pos+1..].trim().parse::<i32>().unwrap_or(0);
            println!("= {}", a + b);
        } else if let Some(pos) = expr.find('-') {
            let a = expr[..pos].trim().parse::<i32>().unwrap_or(0);
            let b = expr[pos+1..].trim().parse::<i32>().unwrap_or(0);
            println!("= {}", a - b);
        } else {
            println!("Use + or -");
        }
    }
}

fn execute_command(cmd: &str) {
    let cmd = cmd.trim();
    
    
    if cmd.starts_with("echo ") {
        let rest = &cmd[5..];
        if let Some(pos) = rest.find('>') {
            let text = rest[..pos].trim().trim_matches('"');
            let path = rest[pos+1..].trim();
            if fs::create_file(path.as_bytes(), text.as_bytes()) {
                println!("OK");
            } else {
                println!("Error: disk full");
            }
        } else {
            println!("{}", rest);
        }
        return;
    }
    
    match cmd {
        "rpm" => {
        package_manager();
    }
        
        "red" => {
        vga_buffer::set_color(Color::Red, Color::Black);
        println!("text color changed to red");
    }
    "green" => {
        vga_buffer::set_color(Color::Green, Color::Black);
        println!("text color changed to green");
    }
    "blue" => {
        vga_buffer::set_color(Color::Blue, Color::Black);
        println!("text color changed to blue");
    }
    "yellow" => {
        vga_buffer::set_color(Color::Yellow, Color::Black);
        println!("Text color changed to yellow");
    }
    "cyan" => {
        vga_buffer::set_color(Color::Cyan, Color::Black);
        println!("Text color changed to cyan");
    }
    "magenta" => {
        vga_buffer::set_color(Color::Magenta, Color::Black);
        println!("Text color changed to magenta");
    }
    "white" => {
        vga_buffer::set_color(Color::White, Color::Black);
        println!("Text color changed to white");
    }
    "color" => {
        vga_buffer::set_color(Color::Yellow, Color::Black);
        println!("This is yellow text!");
        vga_buffer::set_color(Color::Red, Color::Black);
        println!("This is red text!");
        vga_buffer::set_color(Color::Green, Color::Black);
        println!("This is green text!");
        vga_buffer::set_color(Color::Cyan, Color::Black);
        println!("This is cyan text!");
        vga_buffer::reset_color();
        println!("Back to white text!");
    }
        "help" => {
            println!("Commands:");
            println!("________________________");
            println!("|  help  - this message|");
            println!("| clear - clear screen |");
            println!("| hello - greeting     |");
            println!("| calc  - calculator   |");
            println!("| ls    - list files   |");
            println!("| -cat <file>-show file|");
            println!("| -echo \"text\"<file>-write file");
            println!("| -rm <file> - delete file");
            println!("| -exit  - shutdown    |");
            println!("|doc- documentation    |");
            println!("________________________");
        }
        "clear" => clear_screen(),
        "mem" => {
            println!("VGA Buffer: 0xB8000");
            println!("RAM Disk: {} files, max {}", fs::count_files(), 16);
        }
        "reboot" => {
        println!("Rebooting...");
        unsafe { x86_64::instructions::port::PortWriteOnly::new(0x64).write(0xFEu8); }
        loop { x86_64::instructions::hlt(); }
        }
        "doc" => {
    vga_buffer::set_color(Color::Blue, Color::Black);
    println!("|                    ReeOS Doc                                 |");
    println!("|                     Version 1.0.4                            |");
  
            
    
    
    vga_buffer::set_color(Color::White, Color::Black);
    println!("TECHNICAL SPECS");

    println!("  Resolution: 80x25 chars");
    println!("  Colors: 16 VGA colors");
    println!("  Filesystem: RAM (16 files max, 512B each)");
    println!("  Keyboard: US QWERTY (lowercase)");
    println!("");
    println!("COMMANDS");
    println!("  help       - Show all commands");
    println!("  clear      - Clear screen");
    println!("  ls         - List files");
    println!("  cat <f>    - View file");
    println!("  echo > <f> - Write file");
    println!("  rm <f>     - Delete file");
    println!("  calc       - Calculator");
    println!("  time/date  - Show time/date");
    println!("  rpm        - Package manager");
    println!("  exit       - Shutdown");
    println!("");
    println!("COLORS");
    println!("  red, green, blue, yellow, cyan, magenta, white");
    println!("");
    
}

        "calc" => calculator(),
        "hello" => println!("Hello from ReeOS!"),
        "ls" => {
            let mut buffer = [0u8; 512];
            let len = fs::list_files(&mut buffer);
            if len == 0 {
                println!("No files");
            } else {
                print!("{}", from_utf8(&buffer[..len]).unwrap_or(""));
            }
        }
        
    
    
    
    cmd if cmd.starts_with("touch ") => {
        let path = cmd[6..].trim();
        if fs::create_file(path.as_bytes(), b"") {
            println!("Created {}", path);
        } else {
            println!("Error: cannot create {}", path);
        }
    }
    
    
    cmd if cmd.starts_with("wrt ") => {
        let rest = &cmd[6..];
        if let Some(space_pos) = rest.find(' ') {
            let path = &rest[..space_pos];
            let content = rest[space_pos+1..].trim();
            if fs::create_file(path.as_bytes(), content.as_bytes()) {
                println!("Written to {}", path);
            } else {
                println!("Error writing to {}", path);
            }
        } else {
            println!("Usage: write <file> <content>");
        }
    }
    


        cmd if cmd.starts_with("cat ") => {
            let path = cmd[4..].trim();
            match fs::read_file(path.as_bytes()) {
                Some(data) => {
                    if let Some(len) = fs::read_file_len(path.as_bytes()) {
                        for i in 0..len {
                            vga_buffer::WRITER.lock().write_byte(data[i]);
                        }
                        println!();
                    }
                }
                None => println!("File not found: {}", path),
            }
        }
        cmd if cmd.starts_with("rm ") => {
            let path = cmd[3..].trim();
            if fs::delete_file(path.as_bytes()) {
                println!("Deleted {}", path);
            } else {
                println!("File not found: {}", path);
            }
        }
        "exit" => {
            println!("Shutting down...");
            loop { x86_64::instructions::hlt(); }
        }
        "" => {}
        _ => println!("Unknown command: '{}'", cmd),
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("<ReeOS>");
    println!("Type 'help' for commands\n");
    
    vga_buffer::clear_keyboard_buffer();
    
    loop {
        print!("> ");
        let input = vga_buffer::read_line_with_echo();
        
        let cmd = match from_utf8(&input) {
            Ok(s) => s.trim_end_matches('\0').trim(),
            Err(_) => "",
        };
        
        if !cmd.is_empty() {
            execute_command(cmd);
        }
        
        vga_buffer::clear_keyboard_buffer();
    }
}