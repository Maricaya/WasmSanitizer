extern crate clap;
use clap::{App, Arg};
use ansi_term::Colour::{Red, Green, Yellow, Blue, Purple, Cyan};
//use std::ffi::OsStr;
//use std::process;
macro_rules! log_error {
    ($($arg:tt)*) => {{
        println!("{} {}",Red.paint("[ERROR]") ,format!($($arg)*));
    }};
}
fn main() {
    let matches = App::new("Test")
        .version("1.0")
        .about("testtestestsetset")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true), //.index(0),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output file to use")
                .required(true), //.index(1),
        )
        .arg(
            Arg::with_name("FUNC")
                .help("Target instrument function.")
                .required(true), //.index(1),
        )
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap();
    
    match matches.value_of("FUNC").unwrap(){
    "null_reference" =>  
        wasm_instrumenter::test::null_reference(
        input,
        output,
        ),
    "implicit_signed_integer_truncation" =>  
        wasm_instrumenter::test::implicit_signed_integer_truncation(
        input,
        output,
        ),
    "implicit_unsigned_integer_truncation" =>  
        wasm_instrumenter::test::implicit_unsigned_integer_truncation(
        input,
        output,
        ),
    "float_cast_overflow" =>  
        wasm_instrumenter::test::float_cast_overflow(
        input,
        output,
        ),
    "non_return" =>  
        wasm_instrumenter::test::non_return(
        input,
        output,
        ),
    "implicit_integer_sign_change" =>  
        wasm_instrumenter::test::implicit_integer_sign_change(
        input,
        output,
        ),
    "shift" =>  
        wasm_instrumenter::test::shift(
        input,
        output,
        ),
    "copy_function_test" => {
        //println!("test copy function!");
        wasm_instrumenter::test::copy_function_test();
    },
    "unsigned_integer_overflow" => {
        //println!("test copy function!");
        wasm_instrumenter::test::unsigned_integer_overflow(
            input,
            output,
        );
    },
    "signed_integer_overflow" => {
        //println!("test copy function!");
        wasm_instrumenter::test::signed_integer_overflow(
            input,
            output,
        );
    },
    "float_divide_by_zero" => {
        wasm_instrumenter::test::float_divide_by_zero(
            input,
            output,
        );
    },
    "heap_canary" => {
        wasm_instrumenter::test::instrument_with_heap_canary_check(
            input,
            output,
        );
    },
    "stack_canary" => {
        wasm_instrumenter::test::instrument_with_stack_canary_check(
            input,
            output,
        );
    },
    "all" => {
        wasm_instrumenter::test::shift(input,output);
        wasm_instrumenter::test::null_reference(input,output);
        wasm_instrumenter::test::signed_integer_overflow(input,output);
        wasm_instrumenter::test::float_cast_overflow(input,output);
        wasm_instrumenter::test::float_divide_by_zero(input,output);
        wasm_instrumenter::test::implicit_signed_integer_truncation(input,output);
    },
        _ => println!("No func matched!")
    }
    
   
   
}
