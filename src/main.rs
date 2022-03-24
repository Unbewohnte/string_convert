/*
This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
*/

//! A trivial program to convert a string to different numbers because I`m tired to do it in the browser

use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("string_convert [hex|dec|oct|bin] [string]...");
        exit(0);
    }

    let conv_type = args[1].to_lowercase();

    for arg in &args[2..] {
        for (count, char) in arg.chars().enumerate() {
            match &conv_type[..] {
                "hex" => {
                    println!("{}: {} --> {:x}", count, char, char as u32);
                }

                "dec" => {
                    println!("{}: {} --> {}", count, char, char as u32);
                }

                "oct" => {
                    println!("{}: {} --> {:o}", count, char, char as u32);
                }

                "bin" => {
                    println!("{}: {} --> {:b}", count, char, char as u32);
                }

                _ => {
                    eprintln!("[ERROR] Invalid conversion type. Run \"string_convert\" without arguments to get help");
                    exit(1);
                }
            }
        }

        println!();
    }
}
