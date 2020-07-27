//#![no_std]
use blisp;
use std::{env, fs};
use blisp::LispErr;
//use my_heap;

fn read_line() -> String{
        let mut line = String::new();
	std::io::stdin().read_line(&mut line).ok();
	line
}

fn send(c : u32){
   let mut s = String::from("");
   let cs : char = (c  as u8) as char;
   s.push(cs);
   print!("{}", &s);    
}

pub fn my_puts(s : &str) {
    for c in s.bytes() {
        send(c as u32);
        if c == '\n' as u8 {
            send('\r' as u32);
        }
   }
}

pub fn print_decimal(mut h: u64) {
    let mut num = [0; 32];

    if h == 0 {
        send('0' as u32);
        return;
    }

    let mut i = 0;
    while h > 0 {
         let n = h % 10;
         h /= 10;
         num[i] = n + 0x30;
        i += 1;
    }
    while i > 0 {
        send(num[i - 1] as u32);
        i -= 1;
    }
}

fn print_err (e: LispErr) {
   my_puts("error:");
   print_decimal((e.pos.line + 1) as u64);
   my_puts(":");
   print_decimal((e.pos.column + 1) as u64);
   my_puts(":'");
   let msg = e.msg.to_string();
   my_puts(&msg);
   my_puts("'\n");
}

fn run_lisp(code: &String) {
    // initialize
    match blisp::init(&code) {
        Ok(exprs) => {
            // typing
            match blisp::typing(&exprs) {
                Ok(ctx) => {
                    my_puts("init code");	
                    my_puts(&code);
                    my_puts("\n");
                    run_repl(code, &ctx);
                }
                Err(e) => {
		    print_err(e);
                }
            }
        }
        Err(e) => {
           print_err(e);
        }
    }
}

fn run_repl(code: &String, ctx: &blisp::semantics::Context) {
    my_puts("CTRL-D to exit\n");
    loop {
        my_puts(">>");
        let mut line = read_line();
        my_puts("'");
        my_puts(&line);
        my_puts("'\n");
        let result = blisp::eval(&line, ctx);
        match result {
              Ok(rs) => {
                   for r in &rs {
		        my_puts("input:\n");
	                my_puts(&line);
	                my_puts("\nresult:\n");
                        my_puts(&r);
                        my_puts("\n");
                   }
              }
              Err(e) => {
	          print_err(e);
              }
        }
    }
}

fn main() {
//   my_heap::init_head(1000 as usize, 2200 as usize);
   let init = String::from("");
   run_lisp(&init);
}
