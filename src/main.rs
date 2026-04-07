#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use core::str::from_utf8;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn execute_command(cmd: &str) {
    match cmd.trim() {
        "help" => {
            println!("Available commands:");
            println!("  help  - show this message");
            println!("  clear - clear screen");
            println!("  hello - print greeting");
            println!("  exit  - halt the system");
        }
        "clear" => {
            // Простая очистка - выводим много новых строк
            for _ in 0..25 {
                println!();
            }
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
        "" => {} // Пустая команда - ничего не делаем
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
        print!("> ");
        let input = vga_buffer::read_line_with_echo();
        
        // Конвертируем массив в строку
        let cmd = match from_utf8(&input) {
            Ok(s) => s.trim_end_matches('\0').trim(),
            Err(_) => "",
        };
        
        execute_command(cmd);
    }
}