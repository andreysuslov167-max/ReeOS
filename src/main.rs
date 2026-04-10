#![no_std]
#![no_main]

mod fs;
mod vga_buffer;

use core::panic::PanicInfo;
use core::str::from_utf8;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\nPANIC: {}", info);
    loop { x86_64::instructions::hlt(); }
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
        "help" => {
            println!("Commands:");
            println!("  help  - this message");
            println!("  clear - clear screen");
            println!("  hello - greeting");
            println!("  calc  - calculator");
            println!("  ls    - list files");
            println!("  cat <file> - show file");
            println!("  echo \"text\" > <file> - write file");
            println!("  rm <file> - delete file");
            println!("  exit  - shutdown");
        }
        "clear" => clear_screen(),
        "mem" => {
            println!("VGA Buffer: 0xB8000");
            println!("Stack: 1MB");
            println!("RAM Disk: {} files, max {}", fs::count_files(), 16);
        }
        "reboot" => {
        println!("Rebooting...");
        unsafe { x86_64::instructions::port::PortWriteOnly::new(0x64).write(0xFEu8); }
        loop { x86_64::instructions::hlt(); }
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