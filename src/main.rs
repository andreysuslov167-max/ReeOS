#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use core::str::from_utf8;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

fn clear_screen() {
   
    for _ in 0..25 {
        println!();
    }
}

fn calculator() {
    println!("ReeOS Calculator");
    println!("Enter expression (e.g., '2=2', '10-5', '3*4', '8/2')");
    println!("= works like + This will be fixed in the next update");
    println!("Type 'exit' to return to shell\n");
    
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
        
        if expr.is_empty() {
            continue;
        }
        
       
        let result = evaluate_expression(expr);
        match result {
            Ok(value) => println!("= {}", value),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn evaluate_expression(expr: &str) -> Result<i32, &'static str> {
    
    let operators = ['=', '-', '*', '/'];
    
    for &op in &operators {
        if let Some(pos) = expr.find(op) {
            let left_str = expr[..pos].trim();
            let right_str = expr[pos+1..].trim();
            
            let left = left_str.parse::<i32>()
                .map_err(|_| "Invalid left operand")?;
            let right = right_str.parse::<i32>()
                .map_err(|_| "Invalid right operand")?;
            
            return match op {
                '=' => Ok(left + right),
                '-' => Ok(left - right),
                '*' => Ok(left * right),
                '/' => {
                    if right == 0 {
                        Err("Division by zero")
                    } else {
                        Ok(left / right)
                    }
                },
                _ => Err("Unknown operator"),
            };
        }
    }
    
    Err("No operator found (use =, -, *, /)")
}

fn execute_command(cmd: &str) {
    let cmd = cmd.trim();
    
   
    if cmd.starts_with("echo ") {
        let message = &cmd[5..];
        println!("{}", message);
        return;
    }
    
    match cmd {
        "help" => {
            println!("Available commands:");
            println!("  help  - show this message");
            println!("  clear - clear screen");
            println!("  hello - print greeting");
            println!("  exit  - halt the system");
            println!("  calc  - calculator");
            println!("  doc   - documentation");
        }
        "clear" => {
            clear_screen();
        }
        "calc" => {
            calculator();
        }
        "doc" => {
            println!("=== ReeOS Documentation ===");
            println!();
            println!("SYSTEM OVERVIEW");
            println!("  Minimal x86_64 kernel with VGA text mode");
            println!("  Resolution: 80x25 characters");
            println!("  Default colors: Yellow on Black");
            println!();
            println!("COMMANDS");
            println!("  help  - Show this help");
            println!("  clear - Clear entire screen");
            println!("  hello - Display greeting");
            println!("  echo <text> - Print input text");
            println!("  calc  - Interactive calculator");
            println!("  exit  - Halt the system");
            println!();
            println!("KEYBOARD");
            println!("  Supported: A-Z, 0-9, Enter, Backspace, Space");
            println!("  Layout: US QWERTY");
            println!();
            println!("CALCULATOR");
            println!("  Operators: + - * /");
            println!("  Example: 2+2, 10-5, 3*4, 8/2");
            println!();
            println!("MEMORY");
            println!("  VGA Buffer: 0xB8000");
            println!("  Stack: 1MB");
            println!();
            println!("TIPS");
            println!("  Type 'clear' to start fresh");
            println!("  Commands are case-sensitive (lowercase only)");
        }
        "hello" => {
            println!("Hello from ReeOS!");
        }
        "exit" => {
            println!("Shutting down...");
            loop {
                x86_64::instructions::hlt();
            }
        }
        "" => {} 
        _ => {
            println!("Unknown command: '{}'. Type 'help' for commands.", cmd);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("<ReeOS>");
    println!("Type 'help' for commands\n");
    
    loop {
        print!("kernel> ");
        let input = vga_buffer::read_line_with_echo();
        vga_buffer::clear_keyboard_buffer();
        
        
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