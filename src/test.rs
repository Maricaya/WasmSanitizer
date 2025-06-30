// Wasm binary sanitizers test
// 2024.5.6
// By XiaoWu
//use std::error::Error;
//use std::path::Path;
use crate::shared::{find_func_by_name, get_param_n_idx, insert_postfix, insert_prefix};
use rand::distributions::{Distribution, Uniform};

use ordered_float::OrderedFloat;
use ansi_term::Colour::{Red, Green, Yellow};
use wasm::highlevel::{Global, GlobalOp, Instr, LoadOp, LocalOp, Module, Function, NumericOp,
    Local, ImportOrPresent, StoreOp};
use wasm::{BlockType, FunctionType, Idx, Label, Memarg, Mutability, Val, ValType};

const SIGNED_INTEGER_OVERFLOW_FUNCS_PATH: &str = "funcs/signed_integer_overflow.wasm";
const SIGNED_INTEGER_OVERFLOW_FUNCS_PATH_MUL: &str = "funcs/mul_check.wasm";
const SIGNED_INTEGER_OVERFLOW_FUNCS: [&str; 6] =
["is_i32_sign_add_overflow","is_i64_sign_add_overflow",
"is_i32_sign_sub_overflow","is_i64_sign_sub_overflow",
"i32_is_mul_overflow","i64_is_mul_overflow"];
const INI_FUNCS: [&str; 14] =
["__wasm_call_ctors", "_start", "emscripten_stack_init", "emscripten_stack_get_free",
"emscripten_stack_get_base", "emscripten_stack_get_end", "emscripten_stack_get_current",
"__memcpy", "__memset", "__fwritex", "__stdio_write", "fmt_fp", "__ashlti3", "__addtf3"
];

macro_rules! log_info {
    ($($arg:tt)*) => {{
        println!("{} {}",Green.paint("[INFO]") ,format!($($arg)*));
    }};
}

macro_rules! log_error {
    ($($arg:tt)*) => {{
        println!("{} {}",Red.paint("[ERROR]") ,format!($($arg)*));
    }};
}

fn is_string_in_ini_funcs(option_string: &Option<String>) -> bool {
    if let Some(string_value) = option_string {
        let s: &str = &string_value;
        return INI_FUNCS.contains(&s);
    }
    false
}

fn copy_function(func_name: &str, target_module: &mut Module, source_path: &str) -> Result<Idx<Function>, ()>{
    let s_module = Module::from_file(source_path);
    if let Ok(source_module) = s_module{
        let function_to_move = source_module.functions()
       .find(|f| f.1.name == Some(func_name.to_string()))
       .expect("Function not found!")
       .clone().1;
       //log_info!("Get function: {}",func_name);

        //let mut target_module =  Module::from_file(target_path).expect("Fail to load the target module!");
        let type_ = &function_to_move.type_;
        if let ImportOrPresent::Present(code) = &function_to_move.code{
            let idx = target_module.add_function_with_name(
                                type_.clone(),
                                code.locals.clone().into_iter().map(|local| local.type_).collect(),
                                code.body.clone(),
                                func_name.to_string());
            //target_module.remove_function_with_name("is_i32_sign_add_overflow".to_string());
            //target_module.remove_function_with_name("is_i64_sign_add_overflow".to_string());
            //let _ = target_module.to_file(target_path).expect("Fail to encode the Wasm Module!");
            //log_info!("Insert function({}) into target module",Yellow.paint(func_name));
            return Ok(idx);
        }else{
            log_error!("Can not insert import function!");
            return Err(());
        }

    }else if let Err(error) = s_module{
        log_error!("Fail to load the Module: {:?}", error);
        return Err(());
    }
    return Err(());
}

pub fn copy_function_test(){
    //let func_type = FunctionType::new(&[ValType::I32, ValType::I32], &[]);
    let func_name1 = "is_i32_sign_add_overflow";
    let func_name2 = "is_i64_sign_add_overflow";
    let source_path = "/home/WORK/fuzzm-project/wasm_instrumenter/funcs/signed_integer_overflow.wasm";
    let target_path = "/home/WORK/demo.wasm";
    let mut target_module = Module::from_file(target_path).unwrap();
    let _ = copy_function(func_name1,&mut target_module,source_path);
    let _ = copy_function(func_name2,&mut target_module,source_path);
    let _ = target_module.to_file("/home/WORK/demo_ins.wasm").unwrap();
}


pub fn add_proc_exit(input: &str, output: &str) -> Option<Idx<Function>> {
    let module_name = "wasi_snapshot_preview1".to_string();
    let func_name = "proc_exit".to_string();
    let result = Module::from_file(input);
    if let Ok(mut module) = result {
        for (idx, func) in module.functions() {
            match func.import() {
                None => continue,
                Some((import_module,import_name)) => {
                    if (import_module == module_name && import_name == func_name) {
                        log_info!("There is already wasi_proc_exit() function.");
                        let _ = module.to_file(output).expect("Fail to encode the Wasm Module!");
                        return Some(idx);
                    }
                },
            }
        }
        // if "proc_exit" have not been imported
        let proc_func_type = FunctionType {
            params: vec![ValType::I32].into(),
            results: vec![ValType::I32].into(),
        };
        let func_idx = module.add_imported_function(proc_func_type, module_name, func_name, "__wasi_proc_exit".to_string());
        log_info!("Add wasi_proc_exit() function.");
        let _ = module.to_file(output).expect("Fail to encode the Wasm Module!");

        return Some(func_idx);
        //Function.new_imported("wasi_snapshot_preview1","proc_exit",)

    }else if let Err(error) = result {
        log_error!("Fail to load the Module: {:?}", error);
        return None;
    }else {
        None
    }
}

pub fn null_reference(input: &str, output: &str) {
    let result = Module::from_file(input);
    let mut proc_exit_idx: Idx<Function> = Idx::<Function>::from(0);
    if let Ok(mut module) = result {
        for (idx, func) in module.functions() {
            match func.import() {
                None => continue,
                Some((import_module,import_name)) => {
                    if (import_module == "wasi_snapshot_preview1".to_string() && import_name == "proc_exit".to_string()) {
                        proc_exit_idx = idx;
                    }
                },
            }
        }
        for (_, func) in module.functions_mut() {
            if let Some(cur_func_name) = &func.name.clone(){
                if let Some(rest) = cur_func_name.strip_prefix("__"){
                    continue;
                }
                if SIGNED_INTEGER_OVERFLOW_FUNCS.contains(&cur_func_name.as_str()){
                    continue;
                }
                let func_name = &func.name.clone().unwrap();
                let func_param_count = func.param_count();
                if let Some(code) = func.code_mut() {
                    let mut new_instrs = Vec::new();
                    let mut i = 0;
                    while i < code.body.len() {
                        new_instrs.push(code.body[i].clone());
                        if i < 2 {
                            i += 1;
                            continue;
                        }
                        match(&code.body[i-2], &code.body[i-1], &code.body[i]){
                            (Instr::Local(LocalOp::Get, idx1),
                            Instr::Local(LocalOp::Get, idx2),
                            Instr::Store(..)) => {
                                // 在store指令后插入检查local是否为0的指令
                                let new_local1 = add_fresh_local(&mut code.locals, func_param_count, ValType::I32);
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx1));
                                new_instrs.push(Instr::Const(Val::I32(0)));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Eq));
                                new_instrs.push(Instr::Local(LocalOp::Set, new_local1));
                                new_instrs.push(Instr::Block(BlockType(None)));
                                    new_instrs.push(Instr::Local(LocalOp::Get, new_local1));
                                    new_instrs.push(Instr::BrIf(Label(0))); // 没有溢出，跳出
                                    //new_instrs.push(Instr::Const(Val::I32(11))); // return 11
                                    //new_instrs.push(Instr::Call(proc_exit_idx));
                                    new_instrs.push(Instr::Unreachable);
                                new_instrs.push(Instr::End);
                                //log_info!("null_reference instrumentation in {}",&func_name);
                            },
                            _ => { },
                        }
                        i += 1;
                    } // end for while
                    code.body = new_instrs;
                } // end for if let
            }
            else{
                continue;
            }
        } // end for for
        let _ = module.to_file(output).expect("Fail to encode the Wasm Module!");
    } else if let Err(error) = result {
        log_error!("Fail to load the Module: {:?}", error);
    }
}

pub fn add_fresh_local(locals:&mut Vec<Local>, param_count:usize, ty:ValType) -> Idx<Local>{
    let new_idx = param_count + locals.len();
    locals.push(Local::new(ty));
    new_idx.into()
}

pub fn implicit_signed_integer_truncation(input: &str, output: &str) {
    let result = Module::from_file(input);
    let mut proc_exit_idx: Idx<Function> = Idx::<Function>::from(0);
    if let Ok(mut module) = result {
        for (idx, func) in module.functions() {
            match func.import() {
                None => continue,
                Some((import_module,import_name)) => {
                    if (import_module == "wasi_snapshot_preview1".to_string() && import_name == "proc_exit".to_string()) {
                        proc_exit_idx = idx;
                    }
                },
            }
        }
        for (_, func) in module.functions_mut() {
            if let Some(cur_func_name) = &func.name.clone(){
                if let Some(rest) = cur_func_name.strip_prefix("__"){
                    continue;
                }
                if SIGNED_INTEGER_OVERFLOW_FUNCS.contains(&cur_func_name.as_str()){
                    continue;
                }
                let func_param_count = func.param_count();
                if let Some(code) = func.code_mut(){
                    let mut new_instrs = Vec::new();
                    let mut i = 0;
                    while i < code.body.len() {
                        new_instrs.push(code.body[i].clone());
                        if i >= 2 && matches!(code.body[i], Instr::Numeric(NumericOp::I32WrapI64)) {
                            if let (Instr::Local(LocalOp::Get, idx1), Instr::Local(LocalOp::Set, idx2)) = (&code.body[i - 1], &code.body[i+1]) {
                                let new_local1 = add_fresh_local(&mut code.locals, func_param_count, ValType::I32);
                                let new_local2 = add_fresh_local(&mut code.locals, func_param_count, ValType::I64);
                                new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                new_instrs.push(Instr::Local(LocalOp::Set, new_local1));
                                new_instrs.push(Instr::Local(LocalOp::Get, new_local1));

                                new_instrs.push(Instr::Numeric(NumericOp::I64ExtendI32S));
                                new_instrs.push(Instr::Local(LocalOp::Set, new_local2));
                                new_instrs.push(Instr::Local(LocalOp::Get, new_local2));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx1));
                                new_instrs.push(Instr::Numeric(NumericOp::I64Eq));
                                //new_instrs.push(Instr::Numeric(NumericOp::I64Eqz));
                                new_instrs.push(Instr::BrIf(Label(0))); // 如果相等，跳出
                                //new_instrs.push(Instr::Const(Val::I32(12))); // return 12
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable); // 停止代码执行
                                new_instrs.push(Instr::End);
                            }
                        }

                        i += 1;
                    }
                    code.body = new_instrs;
                }
            }
            else{ continue; }
        } // end for for
        let _ = module.to_file(output).expect("Fail to encode the Wasm Module!");
    } else if let Err(error) = result {
        log_error!("Fail to load the Module: {:?}", error);
    }
}

pub fn implicit_unsigned_integer_truncation(input: &str, output: &str) {
    let result = Module::from_file(input);
    let mut proc_exit_idx: Idx<Function> = Idx::<Function>::from(0);
    if let Ok(mut module) = result {
        for (idx, func) in module.functions() {
            match func.import() {
                None => continue,
                Some((import_module,import_name)) => {
                    if (import_module == "wasi_snapshot_preview1".to_string() && import_name == "proc_exit".to_string()) {
                        proc_exit_idx = idx;
                    }
                },
            }
        }
        for (_, func) in module.functions_mut() {
            let func_param_count = func.param_count();
            if let Some(cur_func_name) = &func.name.clone(){
                if let Some(rest) = cur_func_name.strip_prefix("__"){
                    continue;
                }
                if SIGNED_INTEGER_OVERFLOW_FUNCS.contains(&cur_func_name.as_str()){
                    continue;
                }
                if let Some(code) = func.code_mut(){
                    let mut new_instrs = Vec::new();
                    let mut i = 0;
                    while i < code.body.len() {
                        new_instrs.push(code.body[i].clone());
                        if i >= 2 && matches!(code.body[i], Instr::Numeric(NumericOp::I32WrapI64)) {
                            if let (Instr::Local(LocalOp::Get, idx1), Instr::Local(LocalOp::Set, idx2)) = (&code.body[i - 1], &code.body[i+1]) {
                                let new_local1 = add_fresh_local(&mut code.locals, func_param_count, ValType::I32);
                                let new_local2 = add_fresh_local(&mut code.locals, func_param_count, ValType::I64);
                                new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                new_instrs.push(Instr::Local(LocalOp::Set, new_local1));
                                new_instrs.push(Instr::Local(LocalOp::Get, new_local1));

                                new_instrs.push(Instr::Numeric(NumericOp::I64ExtendI32U));
                                new_instrs.push(Instr::Local(LocalOp::Set, new_local2));
                                new_instrs.push(Instr::Local(LocalOp::Get, new_local2));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx1));
                                new_instrs.push(Instr::Numeric(NumericOp::I64Eq));
                                //new_instrs.push(Instr::Numeric(NumericOp::I64Eqz));
                                new_instrs.push(Instr::BrIf(Label(0))); // 如果相等，跳出
                                //new_instrs.push(Instr::Const(Val::I32(13))); // return 13
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable); // 停止代码执行
                                new_instrs.push(Instr::End);
                            }
                        }

                        i += 1;
                    }
                    code.body = new_instrs;
                }
            }
            else{
                continue;
            }
        } // end for for
        let _ = module.to_file(output).expect("Fail to encode the Wasm Module!");
    } else if let Err(error) = result {
        log_error!("Fail to load the Module: {:?}", error);
    }
}

// 不一定对所有的wasm生效，需要进一步测试
pub fn float_cast_overflow(input: &str, output: &str) {
    let result = Module::from_file(input);
    let mut proc_exit_idx: Idx<Function> = Idx::<Function>::from(0);
    if let Ok(mut module) = result {
        for (idx, func) in module.functions() {
            match func.import() {
                None => continue,
                Some((import_module,import_name)) => {
                    if (import_module == "wasi_snapshot_preview1".to_string() && import_name == "proc_exit".to_string()) {
                        proc_exit_idx = idx;
                    }
                },
            }
        }
        for (_, func) in module.functions_mut() {
            let func_param_count = func.param_count();
            if let Some(cur_func_name) = &func.name.clone(){
                if let Some(rest) = cur_func_name.strip_prefix("__"){
                    continue;
                }
                if SIGNED_INTEGER_OVERFLOW_FUNCS.contains(&cur_func_name.as_str()){
                    continue;
                }
                if let Some(code) = func.code_mut(){
                    let mut flag = false;
                    let mut new_instrs = Vec::new();
                    let mut i = 0;
                    while i < code.body.len() {
                        new_instrs.push(code.body[i].clone());
                        if i < 2{
                            i += 1;
                            continue;
                        }
    // -------------------------------------f32.demote_f64, double2float----------------------------------------------
                        match(&code.body[i-2], &code.body[i-1], &code.body[i]){
                            (Instr::Local(LocalOp::Get, idx1),
                            Instr::Numeric(NumericOp::F32DemoteF64),
                            Instr::Local(LocalOp::Set, idx2)) => {
                                let new_local1 = add_fresh_local(&mut code.locals, func_param_count, ValType::I32);
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                new_instrs.push(Instr::Numeric(NumericOp::F64PromoteF32));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx1));
                                new_instrs.push(Instr::Numeric(NumericOp::F64Eq));
                                new_instrs.push(Instr::Local(LocalOp::Set, new_local1));
                                new_instrs.push(Instr::Block(BlockType(None)));
                                    new_instrs.push(Instr::Local(LocalOp::Get, new_local1));
                                    new_instrs.push(Instr::BrIf(Label(0))); // 没有溢出，跳出
                                    //new_instrs.push(Instr::Const(Val::I32(14))); // return 14
                                    //new_instrs.push(Instr::Call(proc_exit_idx));
                                    new_instrs.push(Instr::Unreachable);
                                new_instrs.push(Instr::End);
                            },
                            _ => { },
                        }
    // ---------------------------------------------------------------------------------------------------------------

    // -------------------------------------i32.trunc_f32_s & i32.trunc_f64_s ----------------------------------------
                        match(&code.body[i]){
                            // i32.trunc_f32_s, float2int
                            Instr::Numeric(NumericOp::I32TruncF32S) => {
                                flag = true;
                            },
                            // i32.trunc_f64_s, double2int
                            Instr::Numeric(NumericOp::I32TruncF64S) => {
                                flag = true;
                            },
                            _ => { },
                        }
                        match(&code.body[i-1],&code.body[i]){
                            // end & i32.const -2147483648
                            (Instr::End, Instr::Const(Val::I32(-2147483648))) => {
                                if flag {
                                    //new_instrs.push(Instr::Const(Val::I32(14))); // return 14
                                    //new_instrs.push(Instr::Call(proc_exit_idx));
                                    new_instrs.push(Instr::Unreachable);
                                }
                            },
                            _ => { },
                        }
    // ---------------------------------------------------------------------------------------------------------------
                        i += 1;
                    }// end for while
                    code.body = new_instrs;
                } // end for if_let
            }
            else{
                continue;
            }
        }// end for for
        let _ = module.to_file(output).expect("Fail to encode the Wasm Module!");
    } else if let Err(error) = result {
        log_error!("Fail to load the Module: {:?}", error);
    }
}




pub fn non_return(input: &str, output: &str) {
    let result = Module::from_file(input);
    let mut proc_exit_idx: Idx<Function> = Idx::<Function>::from(0);
    if let Ok(mut module) = result {
        for (idx, func) in module.functions() {
            match func.import() {
                None => continue,
                Some((import_module,import_name)) => {
                    if (import_module == "wasi_snapshot_preview1".to_string() && import_name == "proc_exit".to_string()) {
                        proc_exit_idx = idx;
                    }
                },
            }
        }
        for (_, func) in module.functions_mut() {
            let result_len = func.type_.results.len();
            if let Some(code) = func.code_mut(){
                let mut new_instrs = Vec::new();
                let mut i = 0;
                while i < code.body.len() {
                    new_instrs.push(code.body[i].clone());
                    if (i == code.body.len() - 1) && (result_len > 0) { //最后一条指令
                        if !matches!(code.body[i], Instr::Return) && !matches!(code.body[i], Instr::Unreachable){ //如果最后一条指令不是return
                            //new_instrs.push(Instr::Const(Val::I32(15))); // return 15
                            //new_instrs.push(Instr::Call(proc_exit_idx));
                            new_instrs.push(Instr::Unreachable); // 添加unreachable
                        }
                    }
                    i += 1;
                }// end for while
                code.body = new_instrs;
            } // end for if_let
        }// end for for
        let _ = module.to_file(output).expect("Fail to encode the Wasm Module!");
    } else if let Err(error) = result {
        log_error!("Fail to load the Module: {:?}", error);
    }
}

pub fn implicit_integer_sign_change(input: &str, output: &str) {
    let result = Module::from_file(input);
    let mut proc_exit_idx: Idx<Function> = Idx::<Function>::from(0);
    if let Ok(mut module) = result {
        for (idx, func) in module.functions() {
            match func.import() {
                None => continue,
                Some((import_module,import_name)) => {
                    if (import_module == "wasi_snapshot_preview1".to_string() && import_name == "proc_exit".to_string()) {
                        proc_exit_idx = idx;
                    }
                },
            }
        }
        for (_, func) in module.functions_mut() { // 1:for load instructions
            if is_string_in_ini_funcs(&func.name) { // skip funcs in INI_FUNCS
                continue;
            }
            let func_param_count = func.param_count();
            if let Some(cur_func_name) = &func.name.clone(){
                /* if let Some(rest) = cur_func_name.strip_prefix("__"){
                    continue;
                } */
                if SIGNED_INTEGER_OVERFLOW_FUNCS.contains(&cur_func_name.as_str()){
                    continue;
                }
                let func_name = &func.name.clone().unwrap();
                if let Some(code) = func.code_mut(){
                    let mut new_instrs = Vec::new();
                    let mut i = 0;
                    while i < code.body.len() {
                        new_instrs.push(code.body[i].clone());
                        if i < 1{
                            i += 1;
                            continue;
                        }
                        match(&code.body[i-1], &code.body[i]){
                            (Instr::Load(LoadOp::I32Load8U,_idx), Instr::Local(LocalOp::Set, idx)) => {
                                //let new_local1 = add_fresh_local(&mut code.locals, func_param_count, ValType::I32);
                                new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx));
                                new_instrs.push(Instr::Const(Val::I32(24)));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Shl));
                                new_instrs.push(Instr::Const(Val::I32(24)));
                                new_instrs.push(Instr::Numeric(NumericOp::I32ShrS));
                                new_instrs.push(Instr::Const(Val::I32(0)));
                                new_instrs.push(Instr::Numeric(NumericOp::I32GeS));
                                new_instrs.push(Instr::BrIf(Label(0))); // 如果小于0，跳转到unreachable
                                //new_instrs.push(Instr::Const(Val::I32(16))); // return 16
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable); // 停止代码执行
                                new_instrs.push(Instr::End);
                                log_info!("implicit_integer_sign_change:I32Load8U in {}",&func_name);

                            },
                            (Instr::Load(LoadOp::I32Load16U,_), Instr::Local(LocalOp::Set, idx))  => {
                                new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx));
                                new_instrs.push(Instr::Const(Val::I32(16)));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Shl));
                                new_instrs.push(Instr::Const(Val::I32(16)));
                                new_instrs.push(Instr::Numeric(NumericOp::I32ShrS));
                                new_instrs.push(Instr::Const(Val::I32(0)));
                                new_instrs.push(Instr::Numeric(NumericOp::I32GeS));
                                new_instrs.push(Instr::BrIf(Label(0))); // 如果小于0，跳转到unreachable
                                //new_instrs.push(Instr::Const(Val::I32(16))); // return 16
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable); // 停止代码执行
                                new_instrs.push(Instr::End);
                                log_info!("implicit_integer_sign_change:I32Load16U in {}",&func_name);
                            },
                            (Instr::Load(LoadOp::I64Load8U,_), Instr::Local(LocalOp::Set, idx)) => {
                                new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx));
                                new_instrs.push(Instr::Const(Val::I64(56)));
                                new_instrs.push(Instr::Numeric(NumericOp::I64Shl));
                                new_instrs.push(Instr::Const(Val::I64(56)));
                                new_instrs.push(Instr::Numeric(NumericOp::I64ShrS));
                                new_instrs.push(Instr::Const(Val::I64(0)));
                                new_instrs.push(Instr::Numeric(NumericOp::I64GeS));
                                new_instrs.push(Instr::BrIf(Label(0))); // 如果小于0，跳转到unreachable
                                //new_instrs.push(Instr::Const(Val::I32(16))); // return 16
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable); // 停止代码执行
                                new_instrs.push(Instr::End);
                                log_info!("implicit_integer_sign_change:I64Load8U in {}",&func_name);
                            },
                            (Instr::Load(LoadOp::I64Load16U,_), Instr::Local(LocalOp::Set, idx)) => {
                                new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx));
                                new_instrs.push(Instr::Const(Val::I64(48)));
                                new_instrs.push(Instr::Numeric(NumericOp::I64Shl));
                                new_instrs.push(Instr::Const(Val::I64(48)));
                                new_instrs.push(Instr::Numeric(NumericOp::I64ShrS));
                                new_instrs.push(Instr::Const(Val::I64(0)));
                                new_instrs.push(Instr::Numeric(NumericOp::I64GeS));
                                new_instrs.push(Instr::BrIf(Label(0))); // 如果小于0，跳转到unreachable
                                //new_instrs.push(Instr::Const(Val::I32(16))); // return 16
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable); // 停止代码执行
                                new_instrs.push(Instr::End);
                                log_info!("implicit_integer_sign_change:I64Load16U in {}",&func_name);
                            },
                            (Instr::Load(LoadOp::I64Load32U,_), Instr::Local(LocalOp::Set, idx)) => {
                                new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx));
                                new_instrs.push(Instr::Const(Val::I64(32)));
                                new_instrs.push(Instr::Numeric(NumericOp::I64Shl));
                                new_instrs.push(Instr::Const(Val::I64(32)));
                                new_instrs.push(Instr::Numeric(NumericOp::I64ShrS));
                                new_instrs.push(Instr::Const(Val::I64(0)));
                                new_instrs.push(Instr::Numeric(NumericOp::I64GeS));
                                new_instrs.push(Instr::BrIf(Label(0))); // 如果小于0，跳转到unreachable
                                //new_instrs.push(Instr::Const(Val::I32(16))); // return 16
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable); // 停止代码执行
                                new_instrs.push(Instr::End);
                                log_info!("implicit_integer_sign_change:I64Load32U in {}",&func_name);
                            },
                            _ => { },
                        }

                        i += 1;
                    }// end for while
                    code.body = new_instrs;
                } // end for if_let
            }
            else{
                continue;
            }
        }// end for for

        if let Some((_, malloc)) = find_func_by_name("dlmalloc", module.functions_mut()){ // 2:for functions
            let malloc_p0 = get_param_n_idx(malloc, 0);
            insert_prefix(
            malloc,
            vec![
                Instr::Block(BlockType(None)),
                    Instr::Local(LocalOp::Get, malloc_p0),
                    Instr::Const(Val::I32(0)),
                    Instr::Numeric(NumericOp::I32GtS),
                    Instr::BrIf(Label(0)), // if *malloc_p0 > 0, get out.
                    //Instr::Const(Val::I32(16)), // return 16
                    //Instr::Call(proc_exit_idx),
                    Instr::Unreachable,
                Instr::End,
            ],
        );
            log_info!("Implicit_integer_sign_change:dlmalloc instrumented.")
        }
        if let Some((_, malloc)) = find_func_by_name("__memcpy", module.functions_mut()){ // 2:for functions
            let malloc_p2 = get_param_n_idx(malloc, 2);
            insert_prefix(
            malloc,
            vec![
                Instr::Block(BlockType(None)),
                    Instr::Local(LocalOp::Get, malloc_p2),
                    Instr::Const(Val::I32(0)),
                    Instr::Numeric(NumericOp::I32GeS),
                    Instr::BrIf(Label(0)), // if *malloc_p0 > 0, get out.
                    //Instr::Const(Val::I32(16)), // return 16
                    //Instr::Call(proc_exit_idx),
                    Instr::Unreachable,
                Instr::End,
            ],
        );
            log_info!("Implicit_integer_sign_change:__memcpy instrumented.")
        }
        if let Some((_, malloc)) = find_func_by_name("memmove", module.functions_mut()){ // 2:for functions
            let malloc_p2 = get_param_n_idx(malloc, 2);
            insert_prefix(
            malloc,
            vec![
                Instr::Block(BlockType(None)),
                    Instr::Local(LocalOp::Get, malloc_p2),
                    Instr::Const(Val::I32(0)),
                    Instr::Numeric(NumericOp::I32GeS),
                    Instr::BrIf(Label(0)), // if *malloc_p0 > 0, get out.
                    //Instr::Const(Val::I32(16)), // return 16
                    //Instr::Call(proc_exit_idx),
                    Instr::Unreachable,
                Instr::End,
            ],
        );
            log_info!("Implicit_integer_sign_change:memmove instrumented.")
        }
        if let Some((_, malloc)) = find_func_by_name("strncpy", module.functions_mut()){ // 2:for functions
            let malloc_p2 = get_param_n_idx(malloc, 2);
            insert_prefix(
            malloc,
            vec![
                Instr::Block(BlockType(None)),
                    Instr::Local(LocalOp::Get, malloc_p2),
                    Instr::Const(Val::I32(0)),
                    Instr::Numeric(NumericOp::I32GeS),
                    Instr::BrIf(Label(0)), // if *malloc_p0 > 0, get out.
                    //Instr::Const(Val::I32(16)), // return 16
                    //Instr::Call(proc_exit_idx),
                    Instr::Unreachable,
                Instr::End,
            ],
        );
            log_info!("Implicit_integer_sign_change:strncpy instrumented.")
        }
        let _ = module.to_file(output).expect("Fail to encode the Wasm Module!");
    } else if let Err(error) = result {
        log_error!("Fail to load the Module: {:?}", error);
    }
}

pub fn shift(input: &str, output: &str) {
    let result = Module::from_file(input);
    let mut proc_exit_idx: Idx<Function> = Idx::<Function>::from(0);
    if let Ok(mut module) = result {
        for (idx, func) in module.functions() {
            match func.import() {
                None => continue,
                Some((import_module,import_name)) => {
                    if (import_module == "wasi_snapshot_preview1".to_string() && import_name == "proc_exit".to_string()) {
                        proc_exit_idx = idx;
                    }
                },
            }
        }

        for (_idx, func) in module.functions_mut() {
            //let result_len = func.type_.results.len();
            if let Some(cur_func_name) = &func.name.clone(){
                if let Some(rest) = cur_func_name.strip_prefix("__"){
                    continue;
                }
                if SIGNED_INTEGER_OVERFLOW_FUNCS.contains(&cur_func_name.as_str()){
                    continue;
                }
                if let Some(code) = func.code_mut(){
                    let mut new_instrs = Vec::new();
                    let mut i = 0;
                    while i < code.body.len() {
                        new_instrs.push(code.body[i].clone());
                        if i < 2 {
                            i += 1;
                            continue;
                        }
                        match (&code.body[i-2],&code.body[i-1],&code.body[i]) {
                            (Instr::Local(LocalOp::Get,idx1), // b
                            Instr::Local(LocalOp::Get,idx2), // a
                            Instr::Numeric(NumericOp::I32Shl)) => {                         // i32.shl b<<a
                                new_instrs.push(Instr::Block(BlockType(None)));
                                    new_instrs.push(Instr::Block(BlockType(None)));
                                        new_instrs.push(Instr::Local(LocalOp::Get, *idx1));
                                        new_instrs.push(Instr::Const(Val::I32(0)));
                                        new_instrs.push(Instr::Numeric(NumericOp::I32LtS)); // if b < 0 , err
                                        new_instrs.push(Instr::BrIf(Label(0)));

                                        new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                        new_instrs.push(Instr::Const(Val::I32(32)));
                                        new_instrs.push(Instr::Numeric(NumericOp::I32GeS)); // if a > 32, err
                                        new_instrs.push(Instr::BrIf(Label(0)));

                                        new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                        new_instrs.push(Instr::Const(Val::I32(0)));
                                        new_instrs.push(Instr::Numeric(NumericOp::I32LtS)); // if a < 0, err
                                        new_instrs.push(Instr::BrIf(Label(0)));
                                        new_instrs.push(Instr::Br(Label(1)));
                                    new_instrs.push(Instr::End);
                                    //new_instrs.push(Instr::Const(Val::I32(17))); // return 17
                                    //new_instrs.push(Instr::Call(proc_exit_idx));
                                    new_instrs.push(Instr::Unreachable); // 停止代码执行
                                new_instrs.push(Instr::End);
                                log_info!("Shift(i32) check in func({})",Yellow.paint(cur_func_name));
                            },
                            (Instr::Local(LocalOp::Get,idx1),
                            Instr::Local(LocalOp::Get,idx2),
                            Instr::Numeric(NumericOp::I64Shl)) => { // i64.shl
                                new_instrs.push(Instr::Block(BlockType(None)));
                                    new_instrs.push(Instr::Block(BlockType(None)));
                                        new_instrs.push(Instr::Local(LocalOp::Get, *idx1));
                                        new_instrs.push(Instr::Const(Val::I64(0)));
                                        new_instrs.push(Instr::Numeric(NumericOp::I64LtS));
                                        new_instrs.push(Instr::BrIf(Label(0)));

                                        new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                        new_instrs.push(Instr::Const(Val::I64(64)));
                                        new_instrs.push(Instr::Numeric(NumericOp::I64GeS));
                                        new_instrs.push(Instr::BrIf(Label(0)));

                                        new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                        new_instrs.push(Instr::Const(Val::I64(0)));
                                        new_instrs.push(Instr::Numeric(NumericOp::I64LtS));
                                        new_instrs.push(Instr::BrIf(Label(0)));
                                        new_instrs.push(Instr::Br(Label(1)));
                                    new_instrs.push(Instr::End);
                                    //new_instrs.push(Instr::Const(Val::I32(17))); // return 17
                                    //new_instrs.push(Instr::Call(proc_exit_idx));
                                    new_instrs.push(Instr::Unreachable); // 停止代码执行
                                new_instrs.push(Instr::End);
                                log_info!("Shift(i64) check in func({})",Yellow.paint(cur_func_name));
                            },
                            _ => { },
                        }
                        i += 1;
                    }// end for while
                    code.body = new_instrs;
                } // end for if_let
            }
            else{
                continue;
            }
        }// end for for
        let _ = module.to_file(output).expect("Fail to encode the Wasm Module!");
    } else if let Err(error) = result {
        log_error!("Fail to load the Module: {:?}", error);
    }
}


pub fn unsigned_integer_overflow(input: &str, output: &str) {
    let result = Module::from_file(input);
    let mut proc_exit_idx: Idx<Function> = Idx::<Function>::from(0);
    if let Ok(mut module) = result {
        for (idx, func) in module.functions() {
            match func.import() {
                None => continue,
                Some((import_module,import_name)) => {
                    if (import_module == "wasi_snapshot_preview1".to_string() && import_name == "proc_exit".to_string()) {
                        proc_exit_idx = idx;
                    }
                },
            }
        }

        for (_, func) in module.functions_mut() {
            //let result_len = func.type_.results.len();
            if is_string_in_ini_funcs(&func.name) { // skip funcs in INI_FUNCS
                continue;
            }
            if let Some(cur_func_name) = &func.name.clone(){
                if let Some(rest) = cur_func_name.strip_prefix("__"){
                    continue;
                }
                if SIGNED_INTEGER_OVERFLOW_FUNCS.contains(&cur_func_name.as_str()){
                    continue;
                }
                let func_param_count = func.param_count();
                if let Some(code) = func.code_mut(){
                    let mut new_instrs = Vec::new();
                    let mut i = 0;
                    while i < code.body.len() {
                        new_instrs.push(code.body[i].clone());
                        if i < 2 || i+1 == code.body.len(){
                            i += 1;
                            continue;
                        }
                        match (&code.body[i-2],&code.body[i-1],&code.body[i],&code.body[i+1]) {
                            (Instr::Local(LocalOp::Get,idx1),
                            Instr::Local(LocalOp::Get,_idx2),
                            Instr::Numeric(NumericOp::I32Add),
                            Instr::Local(LocalOp::Set,idx3)) => { // I32Add
                                new_instrs.push(Instr::Local(LocalOp::Tee,*idx3));
                                new_instrs.push(Instr::Block(BlockType(None)));
                                    new_instrs.push(Instr::Local(LocalOp::Get, *idx3));
                                    new_instrs.push(Instr::Local(LocalOp::Get, *idx1));
                                    new_instrs.push(Instr::Numeric(NumericOp::I32GeU)); // 若sum >= x, 则直接跳出
                                    new_instrs.push(Instr::BrIf(Label(0)));
                                    //new_instrs.push(Instr::Const(Val::I32(18))); // return 18
                                    //new_instrs.push(Instr::Call(proc_exit_idx));
                                    new_instrs.push(Instr::Unreachable);
                                new_instrs.push(Instr::End);
                            },
                            (Instr::Local(LocalOp::Get,idx1),
                            Instr::Local(LocalOp::Get,_idx2),
                            Instr::Numeric(NumericOp::I64Add),
                            Instr::Local(LocalOp::Set,idx3)) => { // I64Add
                                new_instrs.push(Instr::Local(LocalOp::Tee,*idx3));
                                new_instrs.push(Instr::Block(BlockType(None)));
                                    new_instrs.push(Instr::Local(LocalOp::Get, *idx3));
                                    new_instrs.push(Instr::Local(LocalOp::Get, *idx1));
                                    new_instrs.push(Instr::Numeric(NumericOp::I64GeU)); // 若sum >= x, 则直接跳出
                                    new_instrs.push(Instr::BrIf(Label(0)));
                                    //new_instrs.push(Instr::Const(Val::I32(18))); // return 18
                                    //new_instrs.push(Instr::Call(proc_exit_idx));
                                    new_instrs.push(Instr::Unreachable);
                                new_instrs.push(Instr::End);
                            },
                            (Instr::Local(LocalOp::Get,idx1),
                            Instr::Local(LocalOp::Get,idx2),
                            Instr::Numeric(NumericOp::I32Sub),
                            Instr::Local(LocalOp::Set,_idx3)) => { // I32Sub
                                /* new_instrs.push(Instr::Block(BlockType(None)));
                                    new_instrs.push(Instr::Local(LocalOp::Get, *idx1)); // b
                                    new_instrs.push(Instr::Local(LocalOp::Get, *idx2)); // a (sub = b - a)
                                    new_instrs.push(Instr::Numeric(NumericOp::I32GeU)); // 若 b >= a, 则直接跳出
                                    new_instrs.push(Instr::BrIf(Label(0)));
                                    //new_instrs.push(Instr::Const(Val::I32(18))); // return 18
                                    //new_instrs.push(Instr::Call(proc_exit_idx));
                                    new_instrs.push(Instr::Unreachable);
                                new_instrs.push(Instr::End); */
                            },
                            (Instr::Local(LocalOp::Get,idx1),
                            Instr::Local(LocalOp::Get,idx2),
                            Instr::Numeric(NumericOp::I64Sub),
                            Instr::Local(LocalOp::Set,_idx3)) => { // I64Sub
                                new_instrs.push(Instr::Block(BlockType(None)));
                                    new_instrs.push(Instr::Local(LocalOp::Get, *idx1)); // b
                                    new_instrs.push(Instr::Local(LocalOp::Get, *idx2)); // a (sub = b - a)
                                    new_instrs.push(Instr::Numeric(NumericOp::I64GeU)); // 若 b >= a, 则直接跳出
                                    new_instrs.push(Instr::BrIf(Label(0)));
                                    //new_instrs.push(Instr::Const(Val::I32(18))); // return 18
                                    //new_instrs.push(Instr::Call(proc_exit_idx));
                                    new_instrs.push(Instr::Unreachable);
                                new_instrs.push(Instr::End);
                            },
                            (Instr::Local(LocalOp::Get,idx1),
                            Instr::Local(LocalOp::Get,idx2),
                            Instr::Numeric(NumericOp::I32Mul),
                            Instr::Local(LocalOp::Set,idx3)) => { // I32Mul
                                new_instrs.push(Instr::Local(LocalOp::Tee,*idx3));
                                new_instrs.push(Instr::Block(BlockType(None)));
                                    new_instrs.push(Instr::Local(LocalOp::Get, *idx1)); // b
                                    new_instrs.push(Instr::Numeric(NumericOp::I64ExtendI32U)); // 扩展为无符号64位整数
                                    new_instrs.push(Instr::Local(LocalOp::Get, *idx2)); // a
                                    new_instrs.push(Instr::Numeric(NumericOp::I64ExtendI32U)); // 扩展为无符号64位整数
                                    new_instrs.push(Instr::Numeric(NumericOp::I64Mul));
                                    new_instrs.push(Instr::Local(LocalOp::Get, *idx3));
                                    new_instrs.push(Instr::Numeric(NumericOp::I64ExtendI32U)); // 扩展为无符号64位整数
                                    new_instrs.push(Instr::Numeric(NumericOp::I64Eq));
                                    new_instrs.push(Instr::BrIf(Label(0)));
                                    //new_instrs.push(Instr::Const(Val::I32(18))); // return 18
                                    //new_instrs.push(Instr::Call(proc_exit_idx));
                                    new_instrs.push(Instr::Unreachable);
                                new_instrs.push(Instr::End);
                            },
                            _ => { } ,
                        }
                        i += 1;
                    }// end for while
                    code.body = new_instrs;
                } // end for if_let
            }
            else{
                continue;
            }
        }// end for for
        let _ = module.to_file(output).expect("Fail to encode the Wasm Module!");
    } else if let Err(error) = result {
        log_error!("Fail to load the Module: {:?}", error);
    }
}


pub fn signed_integer_overflow(input: &str, output: &str) {
    let mut i32_add_flag = false;
    let mut i64_add_flag = false;
    let mut i32_sub_flag = false;
    let mut i64_sub_flag = false;
    let mut proc_exit_idx: Idx<Function> = Idx::<Function>::from(0);
    let mut module = Module::from_file(input).expect("Fail to load the Module");
    for (idx, func) in module.functions() {
        match func.import() {
            None => continue,
            Some((import_module,import_name)) => {
                if (import_module == "wasi_snapshot_preview1".to_string() && import_name == "proc_exit".to_string()) {
                    proc_exit_idx = idx;
                }
            },
        }
    }
    let i32_add_target_index: Idx<Function> = copy_function("is_i32_sign_add_overflow",
                                                &mut module,
                                                SIGNED_INTEGER_OVERFLOW_FUNCS_PATH,)
                                                .expect("Fail to insert the func!");
    let i64_add_target_index: Idx<Function> = copy_function("is_i64_sign_add_overflow",
                                                &mut module,
                                                SIGNED_INTEGER_OVERFLOW_FUNCS_PATH,)
                                                .expect("Fail to insert the func!");
    let i32_sub_target_index: Idx<Function> = copy_function("is_i32_sign_sub_overflow",
                                                &mut module,
                                                SIGNED_INTEGER_OVERFLOW_FUNCS_PATH,)
                                                .expect("Fail to insert the func!");
    let i64_sub_target_index: Idx<Function> = copy_function("is_i64_sign_sub_overflow",
                                                &mut module,
                                                SIGNED_INTEGER_OVERFLOW_FUNCS_PATH,)
                                                .expect("Fail to insert the func!");
    let i32_mul_target_index: Idx<Function> = copy_function("i32_is_mul_overflow",
                                                &mut module,
                                                SIGNED_INTEGER_OVERFLOW_FUNCS_PATH_MUL,)
                                                .expect("Fail to insert the func!");
    let i64_mul_target_index: Idx<Function> = copy_function("i64_is_mul_overflow",
                                                &mut module,
                                                SIGNED_INTEGER_OVERFLOW_FUNCS_PATH_MUL,)
                                                .expect("Fail to insert the func!");
    for (idx, func) in module.functions_mut() {
        let func_param_count = func.param_count();
        if is_string_in_ini_funcs(&func.name) { // skip funcs in INI_FUNCS
                continue;
        }
        let result_len = func.type_.results.len();
        //log_info!("idx = {}",idx.into_inner());
        if let Some(cur_func_name) = &func.name.clone(){
            if let Some(rest) = cur_func_name.strip_prefix("__"){
                continue;
            }
            if let Some(rest) = cur_func_name.strip_prefix("std::"){
            continue;
        }
            if SIGNED_INTEGER_OVERFLOW_FUNCS.contains(&cur_func_name.as_str()){
                continue;
            }
            if let Some(code) = func.code_mut(){
                let mut new_instrs = Vec::new();
                let mut i = 0;
                while i < code.body.len() {
                    new_instrs.push(code.body[i].clone());
                    if i < 2 || i+1 == code.body.len(){
                        i += 1;
                        continue;
                    }
                    match (&code.body[i-2],&code.body[i-1],&code.body[i],&code.body[i+1]) {
                        (Instr::Local(LocalOp::Get,idx1),
                        Instr::Local(LocalOp::Get,idx2),
                        Instr::Numeric(NumericOp::I32Add),
                        Instr::Local(LocalOp::Set,_idx3)) => { // I32Add
                            if !i32_add_flag {
                                i32_add_flag = true;
                            }
                            log_info!("Call function({}) in func({})"
                                ,Yellow.paint("is_i32_sign_add_overflow")
                                ,Yellow.paint(cur_func_name));
                            new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx1));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                new_instrs.push(Instr::Call(i32_add_target_index));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Eqz));
                                new_instrs.push(Instr::BrIf(Label(0))); // 没有溢出，跳出
                                //new_instrs.push(Instr::Const(Val::I32(19))); // return 19
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable);
                            new_instrs.push(Instr::End);
                        },
                        (Instr::Local(LocalOp::Get,idx1),
                        Instr::Local(LocalOp::Get,idx2),
                        Instr::Numeric(NumericOp::I64Add),
                        Instr::Local(LocalOp::Set,_idx3)) => { // I64Add
                            if !i32_add_flag {
                                i64_add_flag = true;
                            }
                            log_info!("Call function({}) in func({})"
                                ,Yellow.paint("is_i64_sign_add_overflow")
                                ,Yellow.paint(cur_func_name));
                            new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx1));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                new_instrs.push(Instr::Call(i64_add_target_index));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Eqz)); // 注意这里还是I32Eqz
                                new_instrs.push(Instr::BrIf(Label(0))); // 没有溢出，跳出
                                //new_instrs.push(Instr::Const(Val::I32(19))); // return 19
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable);
                            new_instrs.push(Instr::End);
                        },
                        (Instr::Local(LocalOp::Get,idx1),
                        Instr::Local(LocalOp::Get,idx2),
                        Instr::Numeric(NumericOp::I32Sub),
                        Instr::Local(LocalOp::Set,_idx3)) => { // I32Sub
                            if !i32_sub_flag {
                                i32_sub_flag = true;
                            }
                            log_info!("Call function({}) in func({})"
                                ,Yellow.paint("is_i32_sign_sub_overflow")
                                ,Yellow.paint(cur_func_name));
                            new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx1));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                new_instrs.push(Instr::Call(i32_sub_target_index));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Eqz));
                                new_instrs.push(Instr::BrIf(Label(0))); // 没有溢出，跳出
                                //new_instrs.push(Instr::Const(Val::I32(19))); // return 19
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable);
                            new_instrs.push(Instr::End);
                        },
                        (Instr::Local(LocalOp::Get,idx1),
                        Instr::Local(LocalOp::Get,idx2),
                        Instr::Numeric(NumericOp::I64Sub),
                        Instr::Local(LocalOp::Set,_idx3)) => { // I64Sub
                            if !i64_sub_flag {
                                i64_sub_flag = true;
                            }
                            log_info!("Call function({}) in func({})"
                                ,Yellow.paint("is_i64_sign_sub_overflow")
                                ,Yellow.paint(cur_func_name));
                            new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx1));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                new_instrs.push(Instr::Call(i64_sub_target_index));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Eqz)); // 注意这里还是I32Eqz
                                new_instrs.push(Instr::BrIf(Label(0))); // 没有溢出，跳出
                                //new_instrs.push(Instr::Const(Val::I32(19))); // return 19
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable);
                            new_instrs.push(Instr::End);
                        },
                        (Instr::Local(LocalOp::Get,idx1),
                        Instr::Local(LocalOp::Get,idx2),
                        Instr::Numeric(NumericOp::I32Mul), // I32Mul
                        Instr::Local(LocalOp::Set,_idx3)) => {

                            log_info!("Call function({}) in func({})"
                                ,Yellow.paint("i32_is_mul_overflow")
                                ,Yellow.paint(cur_func_name));
                            new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx1));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                new_instrs.push(Instr::Call(i32_mul_target_index));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Eqz));
                                new_instrs.push(Instr::BrIf(Label(0))); // 没有溢出，跳出
                                //new_instrs.push(Instr::Const(Val::I32(19))); // return 19
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable);
                            new_instrs.push(Instr::End);
                        },
                        (Instr::Local(LocalOp::Get,idx1),
                        Instr::Local(LocalOp::Get,idx2),
                        Instr::Numeric(NumericOp::I64Mul), // I64Mul
                        Instr::Local(LocalOp::Set,_idx3)) => {
                            log_info!("Call function({}) in func({})"
                                ,Yellow.paint("i64_is_mul_overflow")
                                ,Yellow.paint(cur_func_name));
                            new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx1));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                new_instrs.push(Instr::Call(i64_mul_target_index));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Eqz));
                                new_instrs.push(Instr::BrIf(Label(0))); // 没有溢出，跳出
                                //new_instrs.push(Instr::Const(Val::I32(19))); // return 19
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable);
                            new_instrs.push(Instr::End);
                        },
                        _ => {},
                    }

                    if i < 12 || i+1 == code.body.len(){
                        i += 1;
                        continue;
                    }
                    match (&code.body[i-12],&code.body[i-11],&code.body[i-10],&code.body[i-9],
                            &code.body[i-8],&code.body[i-7],&code.body[i-6],&code.body[i-5],
                            &code.body[i-4],&code.body[i-3],&code.body[i-2],&code.body[i-1],
                            &code.body[i],&code.body[i+1]) {
                        (Instr::Const(Val::I32(num)),
                        Instr::Local(LocalOp::Set,_1),
                        Instr::Local(LocalOp::Get,_2),
                        Instr::Local(LocalOp::Get,_3),
                        Instr::Numeric(NumericOp::I32Shl),
                        Instr::Local(LocalOp::Set,_4),
                        Instr::Local(LocalOp::Get,_5),
                        Instr::Local(LocalOp::Get,_6),
                        Instr::Numeric(NumericOp::I32ShrS),
                        Instr::Local(LocalOp::Set,_7),
                        Instr::Local(LocalOp::Get,idx1),
                        Instr::Local(LocalOp::Get,idx2),
                        Instr::Numeric(NumericOp::I32Mul),
                        Instr::Local(LocalOp::Set,idx3)) => { // char mul
                            if *num == 24 {
                                log_info!("Char mul check in func({})"
                                ,Yellow.paint(cur_func_name));

                                let new_local1 = add_fresh_local(&mut code.locals, func_param_count, ValType::I32);
                                let new_local2 = add_fresh_local(&mut code.locals, func_param_count, ValType::I32);
                                new_instrs.push(Instr::Local(LocalOp::Tee, new_local1));
                                new_instrs.push(Instr::Local(LocalOp::Get, new_local1));
                                //new_instrs.push(Instr::Const(Val::I32(24)));
                                //new_instrs.push(Instr::Numeric(NumericOp::I32Shl)); // result << 24
                                //new_instrs.push(Instr::Const(Val::I32(24)));
                                //new_instrs.push(Instr::Numeric(NumericOp::I32ShrS));  // result >> 24
                                new_instrs.push(Instr::Const(Val::I32(255)));
                                new_instrs.push(Instr::Numeric(NumericOp::I32GtS));
                                new_instrs.push(Instr::Local(LocalOp::Get, new_local1));
                                new_instrs.push(Instr::Const(Val::I32(0)));
                                new_instrs.push(Instr::Numeric(NumericOp::I32LtS));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Or));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Eqz));
                                new_instrs.push(Instr::Local(LocalOp::Set, new_local2));
                                new_instrs.push(Instr::Block(BlockType(None)));
                                    new_instrs.push(Instr::Local(LocalOp::Get, new_local2));
                                    new_instrs.push(Instr::BrIf(Label(0)));
                                    //new_instrs.push(Instr::Const(Val::I32(19))); // return 19
                                    //new_instrs.push(Instr::Call(proc_exit_idx));
                                    new_instrs.push(Instr::Unreachable);
                                new_instrs.push(Instr::End);
                            }
                            else if *num == 16 { // short mul
                                log_info!("Short mul check in func({})"
                                ,Yellow.paint(cur_func_name));

                                let new_local1 = add_fresh_local(&mut code.locals, func_param_count, ValType::I32);
                                let new_local2 = add_fresh_local(&mut code.locals, func_param_count, ValType::I32);
                                new_instrs.push(Instr::Local(LocalOp::Tee, new_local1));
                                new_instrs.push(Instr::Local(LocalOp::Get, new_local1));
                                //new_instrs.push(Instr::Const(Val::I32(16)));
                                //new_instrs.push(Instr::Numeric(NumericOp::I32Shl)); // result << 16
                                //new_instrs.push(Instr::Const(Val::I32(16)));
                               //new_instrs.push(Instr::Numeric(NumericOp::I32ShrS));  // result >> 16
                                new_instrs.push(Instr::Const(Val::I32(65535)));
                                new_instrs.push(Instr::Numeric(NumericOp::I32GtS));
                                new_instrs.push(Instr::Local(LocalOp::Get, new_local1));
                                new_instrs.push(Instr::Const(Val::I32(0)));
                                new_instrs.push(Instr::Numeric(NumericOp::I32LtS));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Or));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Eqz));
                                new_instrs.push(Instr::Local(LocalOp::Set, new_local2));
                                new_instrs.push(Instr::Block(BlockType(None)));
                                    new_instrs.push(Instr::Local(LocalOp::Get, new_local2));
                                    new_instrs.push(Instr::BrIf(Label(0)));
                                    //new_instrs.push(Instr::Const(Val::I32(19))); // return 19
                                    //new_instrs.push(Instr::Call(proc_exit_idx));
                                    new_instrs.push(Instr::Unreachable);
                                new_instrs.push(Instr::End);
                            }

                        },
                        _ => {},
                    }

                    if i+ 4 >= code.body.len(){
                        i += 1;
                        continue;
                    }
                    match (&code.body[i],&code.body[i+1],&code.body[i+2],&code.body[i+3],&code.body[i+4]) {
                        (Instr::Numeric(NumericOp::I32Add),
                        Instr::Local(LocalOp::Set,_1),
                        Instr::Local(LocalOp::Get,_2),
                        Instr::Local(LocalOp::Get,_3),
                        Instr::Store(StoreOp::I32Store8,..)) => { // char add
                            log_info!("Char add check in func({})"
                                ,Yellow.paint(cur_func_name));
                            let new_local1 = add_fresh_local(&mut code.locals, func_param_count, ValType::I32);
                            let new_local2 = add_fresh_local(&mut code.locals, func_param_count, ValType::I32);
                            new_instrs.push(Instr::Local(LocalOp::Tee, new_local1));
                            new_instrs.push(Instr::Local(LocalOp::Get, new_local1));
                            new_instrs.push(Instr::Const(Val::I32(24)));
                            new_instrs.push(Instr::Numeric(NumericOp::I32Shl)); // result << 24
                            new_instrs.push(Instr::Const(Val::I32(24)));
                            new_instrs.push(Instr::Numeric(NumericOp::I32ShrS));  // result >> 24
                            new_instrs.push(Instr::Const(Val::I32(127)));
                            new_instrs.push(Instr::Numeric(NumericOp::I32GtS));
                            new_instrs.push(Instr::Local(LocalOp::Get, new_local1));
                            new_instrs.push(Instr::Const(Val::I32(-128)));
                            new_instrs.push(Instr::Numeric(NumericOp::I32LtS));
                            new_instrs.push(Instr::Numeric(NumericOp::I32Or));
                            new_instrs.push(Instr::Numeric(NumericOp::I32Eqz));
                            new_instrs.push(Instr::Local(LocalOp::Set, new_local2));
                            new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, new_local2));
                                new_instrs.push(Instr::BrIf(Label(0)));
                                //new_instrs.push(Instr::Const(Val::I32(19))); // return 19
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable);
                            new_instrs.push(Instr::End);
                        },
                        (Instr::Numeric(NumericOp::I32Add),
                        Instr::Local(LocalOp::Set,_1),
                        Instr::Local(LocalOp::Get,_2),
                        Instr::Local(LocalOp::Get,_3),
                        Instr::Store(StoreOp::I32Store16,..)) => { // short add
                            log_info!("Short add check in func({})"
                                ,Yellow.paint(cur_func_name));
                            let new_local1 = add_fresh_local(&mut code.locals, func_param_count, ValType::I32);
                            let new_local2 = add_fresh_local(&mut code.locals, func_param_count, ValType::I32);
                            new_instrs.push(Instr::Local(LocalOp::Tee, new_local1));
                            new_instrs.push(Instr::Local(LocalOp::Get, new_local1));
                            new_instrs.push(Instr::Const(Val::I32(16)));
                            new_instrs.push(Instr::Numeric(NumericOp::I32Shl)); // result << 16
                            new_instrs.push(Instr::Const(Val::I32(16)));
                            new_instrs.push(Instr::Numeric(NumericOp::I32ShrS));  // result >> 16
                            new_instrs.push(Instr::Const(Val::I32(32767)));
                            new_instrs.push(Instr::Numeric(NumericOp::I32GtS));
                            new_instrs.push(Instr::Local(LocalOp::Get, new_local1));
                            new_instrs.push(Instr::Const(Val::I32(-32768)));
                            new_instrs.push(Instr::Numeric(NumericOp::I32LtS));
                            new_instrs.push(Instr::Numeric(NumericOp::I32Or));
                            new_instrs.push(Instr::Numeric(NumericOp::I32Eqz));
                            new_instrs.push(Instr::Local(LocalOp::Set, new_local2));
                            new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, new_local2));
                                new_instrs.push(Instr::BrIf(Label(0)));
                                //new_instrs.push(Instr::Const(Val::I32(19))); // return 19
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable);
                            new_instrs.push(Instr::End);
                        },
                        _  => {},

                    }

                    /* if (i+ 4 >= code.body.len() || i < 2){
                        i += 1;
                        continue;
                    }
                    match (&code.body[i-2],&code.body[i-1],&code.body[i],&code.body[i+1],&code.body[i+2],
                            &code.body[i+3],&code.body[i+4]){
                        (Instr::Local(LocalOp::Get,idx1), // b
                        Instr::Local(LocalOp::Get,idx2), // a
                        Instr::Numeric(NumericOp::I32Shl), // I32Shl
                        Instr::Local(LocalOp::Set,_idx3),
                        Instr::Local(LocalOp::Get,idx4),
                        Instr::Local(LocalOp::Get,idx5),
                        Instr::Numeric(NumericOp::I32ShrS), // I32Shr_S
                        ) => {

                            log_info!("Call function({}) in func({})"
                                ,Yellow.paint("i32_is_shift_overflow")
                                ,Yellow.paint(cur_func_name));
                                new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx1));

                                new_instrs.push(Instr::Const(Val::I32(1)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Shl));

                                new_instrs.push(Instr::Call(i32_mul_target_index));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Eqz));
                                new_instrs.push(Instr::BrIf(Label(0))); // 没有溢出，跳出
                                //new_instrs.push(Instr::Const(Val::I32(19))); // return 19
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable);
                            new_instrs.push(Instr::End);
                        },
                        (Instr::Local(LocalOp::Get,idx1),
                        Instr::Local(LocalOp::Get,idx2),
                        Instr::Numeric(NumericOp::I64Shl), // I64SHl
                        Instr::Local(LocalOp::Set,_idx3)) => {
                            log_info!("Call function({}) in func({})"
                                ,Yellow.paint("i64_is_shift_overflow")
                                ,Yellow.paint(cur_func_name));
                            new_instrs.push(Instr::Block(BlockType(None)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx1));

                                new_instrs.push(Instr::Const(Val::I64(1)));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                new_instrs.push(Instr::Numeric(NumericOp::I64Shl));

                                new_instrs.push(Instr::Call(i64_mul_target_index));
                                new_instrs.push(Instr::Numeric(NumericOp::I32Eqz));
                                new_instrs.push(Instr::BrIf(Label(0))); // 没有溢出，跳出
                            // new_instrs.push(Instr::Const(Val::I32(19))); // return 19
                                //new_instrs.push(Instr::Call(proc_exit_idx));
                                new_instrs.push(Instr::Unreachable);
                            new_instrs.push(Instr::End);
                        },
                        _ => {},
                    } */
                    i += 1;
                }// end for while
                code.body = new_instrs;
            } // end for if_let
        }
        else{
            continue;
        }
    }// end for for
/*     if !i32_add_flag{
        module.remove_function_with_name("is_i32_sign_add_overflow".to_string());
        log_info!("Remove function({}) from target module",Cyan.paint("is_i32_sign_add_overflow"));
    }
    if !i64_add_flag{
        module.remove_function_with_name("is_i64_sign_add_overflow".to_string());
        log_info!("Remove function({}) from target module",Cyan.paint("is_i64_sign_add_overflow"));
    }
    if !i32_sub_flag{
        module.remove_function_with_name("is_i32_sign_sub_overflow".to_string());
        log_info!("Remove function({}) from target module",Cyan.paint("is_i32_sign_sub_overflow"));
    }
    if !i64_sub_flag{
        module.remove_function_with_name("is_i64_sign_sub_overflow".to_string());
        log_info!("Remove function({}) from target module",Cyan.paint("is_i64_sign_sub_overflow"));
    } */
    let _ = module.to_file(output).expect("Fail to encode the Wasm Module!");

}

pub fn float_divide_by_zero(input: &str, output: &str){
    let result = Module::from_file(input);
    let mut proc_exit_idx: Idx<Function> = Idx::<Function>::from(0);
    if let Ok(mut module) = result {
        for (idx, func) in module.functions() {
            match func.import() {
                None => continue,
                Some((import_module,import_name)) => {
                    if (import_module == "wasi_snapshot_preview1".to_string() && import_name == "proc_exit".to_string()) {
                        proc_exit_idx = idx;
                    }
                },
            }
        }
        for (_, func) in module.functions_mut() {
            if let Some(cur_func_name) = &func.name.clone(){

                if let Some(rest) = cur_func_name.strip_prefix("__"){
                    continue;
                }
                if SIGNED_INTEGER_OVERFLOW_FUNCS.contains(&cur_func_name.as_str()){
                    continue;
                }
                let func_param_count = func.param_count();
                if let Some(code) = func.code_mut(){
                    let mut new_instrs = Vec::new();
                    let mut i = 0;
                    while i < code.body.len() {
                        new_instrs.push(code.body[i].clone());
                        if i < 1 || i+1 == code.body.len(){
                            i += 1;
                            continue;
                        }

                        match (&code.body[i-1],&code.body[i],&code.body[i+1]) {
                            (Instr::Local(LocalOp::Get,_idx1),
                            Instr::Local(LocalOp::Get,idx2),
                            Instr::Numeric(NumericOp::F32Div)) => {
                                let new_local1 = add_fresh_local(&mut code.locals, func_param_count, ValType::I32);
                                new_instrs.push(Instr::Const(Val::F32(OrderedFloat(0.0))));
                                new_instrs.push(Instr::Local(LocalOp::Get, *idx2));
                                new_instrs.push(Instr::Numeric(NumericOp::F32Ne));
                                new_instrs.push(Instr::Local(LocalOp::Set, new_local1));
                                new_instrs.push(Instr::Block(BlockType(None)));
                                    new_instrs.push(Instr::Local(LocalOp::Get, new_local1));
                                    new_instrs.push(Instr::BrIf(Label(0))); // 没有溢出，跳出
                                    //new_instrs.push(Instr::Const(Val::I32(20))); // return 20
                                    //new_instrs.push(Instr::Call(proc_exit_idx));
                                    new_instrs.push(Instr::Unreachable);
                                new_instrs.push(Instr::End);

                            },
                            _ => { } ,
                        }
                        i += 1;
                    }// end for while
                    code.body = new_instrs;
                } // end for if_let
            }
            else{
                continue;
            }
        }// end for for
        let _ = module.to_file(output).expect("Fail to encode the Wasm Module!");
    } else if let Err(error) = result {
        log_error!("Fail to load the Module: {:?}", error);
    }
}

pub fn instrument_with_heap_canary_check(input: &str, output: &str) {
    let result = Module::from_file(input);
    let mut proc_exit_idx: Idx<Function> = Idx::<Function>::from(0);
    if let Ok(mut module) = result {
        for (idx, func) in module.functions() {
            match func.import() {
                None => continue,
                Some((import_module,import_name)) => {
                    if (import_module == "wasi_snapshot_preview1".to_string() && import_name == "proc_exit".to_string()) {
                        proc_exit_idx = idx;
                    }
                },
            }
        }
        // We disable canary checks in calls to free that occur
        // within the realloc function since the block passed to free does not have canary values.
        // Notice, realloc itself performs canary checking at its entry, so this does not
        // result in missed buffer overflows.
        let m: &mut Module = &mut module;
        let disable_canary_glob = m.add_global(
            ValType::I32,
            Mutability::Mut,
            vec![Instr::Const(Val::I32(0)), Instr::End],
        );

        // We disable canary insertion when malloc is called within calloc.
        // calloc itself will insert the canaries at its exit.
        // calloc requires access to metadata of the heap chunk returned by malloc, which is why this
        // is required.
        let disable_canary_insertion_in_malloc = m.add_global(
            ValType::I32,
            Mutability::Mut,
            vec![Instr::Const(Val::I32(0)), Instr::End],
        );

        // TODO detect malloc by other heuristics than by name.
        let (_, malloc) = find_func_by_name("dlmalloc", m.functions_mut())
            .expect("Aborting: Could not find malloc function");

        let malloc_p0 = get_param_n_idx(malloc, 0);
        let malloc_size = malloc.add_fresh_local(ValType::I32);

        insert_prefix(
            malloc,
            // preamble for malloc (reserve an extra 20 bytes)
            // 2 canaries take 16 bytes and the size takes 4 bytes.
            // Notice, we do not reuse the size field from the chunk since
            // 1. it may contain flag bits which we have to account for
            // 2. the returned chunk may the larger than the requested size
            // resulting in canaries being placed unnecessarily far apart.
            vec![
                // check if canary insertion is disabled
                Instr::Block(BlockType(None)),
                Instr::Global(GlobalOp::Get, disable_canary_insertion_in_malloc),
                Instr::Const(Val::I32(1)),
                Instr::Numeric(NumericOp::I32Eq),
                Instr::BrIf(Label(0)),
                Instr::Local(LocalOp::Get, malloc_p0),
                //TODO do we also need to 16 byte align the heap values?
                //I don't think that is necessary, but it might be worth while to check.
                Instr::Const(Val::I32(20)),
                Instr::Numeric(NumericOp::I32Add),
                // store the size in a fresh local in case the first parameter gets overwritten
                Instr::Local(LocalOp::Tee, malloc_size),
                Instr::Local(LocalOp::Set, malloc_p0),
                Instr::End,
            ],
        );

        // wrap the postfix in a function since we need to use it for both malloc
        // and for realloc, but we need different locals to store the tmp result and size.
        let get_malloc_postfix = |func: &mut Function,
                                size_local: Idx<Local>,
                                disable_check: Option<Vec<Instr>>|
        -> Vec<Instr> {
            let tmp_result_local = func.add_fresh_local(ValType::I32);
            let mut postfix = vec![
                Instr::Local(LocalOp::Set, tmp_result_local),
                Instr::Block(BlockType(Some(ValType::I32))),
                // if result is NULL, then don't attempt to insert canaries.
                Instr::Local(LocalOp::Get, tmp_result_local),
                Instr::Local(LocalOp::Get, tmp_result_local),
                Instr::Numeric(NumericOp::I32Eqz),
                Instr::BrIf(Label(0)),
            ];
            if disable_check.is_some() {
                postfix.append(&mut disable_check.unwrap());
                //postfix.append(&mut );
            }
            // store malloc result (ptr to data) in a new local
            // store the size at the first 4 bytes
            postfix.append(&mut vec![
                Instr::Local(LocalOp::Get, size_local),
                Instr::Store(
                    StoreOp::I32Store,
                    Memarg {
                        alignment_exp: 0,
                        offset: 0,
                    },
                ),
                // store the first canary at byte 4 to 11
                Instr::Local(LocalOp::Get, tmp_result_local),
                Instr::Const(Val::I64(0)),
                Instr::Store(
                    StoreOp::I64Store,
                    Memarg {
                        alignment_exp: 0,
                        offset: 4,
                    },
                ),
                // get the size
                Instr::Local(LocalOp::Get, size_local),
                // add with x to get the index of the last data chunk
                Instr::Local(LocalOp::Get, tmp_result_local),
                Instr::Numeric(NumericOp::I32Add),
                // index (ptr + size - 8) where we want to place the canary
                Instr::Const(Val::I32(8)),
                Instr::Numeric(NumericOp::I32Sub),
                // store post data canary
                Instr::Const(Val::I64(0)),
                Instr::Store(
                    StoreOp::I64Store,
                    Memarg {
                        alignment_exp: 0,
                        offset: 0,
                    },
                ),
                // return ptr + 12, i.e., ptr to first element after the first canary.
                Instr::Local(LocalOp::Get, tmp_result_local),
                Instr::Const(Val::I32(12)),
                Instr::Numeric(NumericOp::I32Add),
                Instr::End,
            ]);
            postfix
        };
        let mut post_fix = get_malloc_postfix(
            malloc,
            malloc_size,
            Some(vec![
                // check if canary insertion is disabled
                Instr::Global(GlobalOp::Get, disable_canary_insertion_in_malloc),
                Instr::Const(Val::I32(1)),
                Instr::Numeric(NumericOp::I32Eq),
                // returns tmp_result_local pushed in malloc postfix
                Instr::BrIf(Label(0)),
            ]),
        );
        insert_postfix(malloc, &mut post_fix);

        let (_, free) = find_func_by_name("dlfree", m.functions_mut())
            .expect("Aborting: Could not find free function");

        fn get_canary_check_block(
            chunk_data_ptr: Idx<Local>,
            disable_canary_glob: Idx<Global>,
        ) -> Vec<Instr> {
            vec![
                // if disable_canary_glob then skip the checks
                Instr::Block(BlockType(None)),
                Instr::Global(GlobalOp::Get, disable_canary_glob),
                Instr::Const(Val::I32(1)),
                Instr::Numeric(NumericOp::I32Eq),
                Instr::BrIf(Label(0)),
                // if ptr == NULL, then skip the canary checks
                Instr::Block(BlockType(None)),
                Instr::Local(LocalOp::Get, chunk_data_ptr),
                Instr::Numeric(NumericOp::I32Eqz),
                Instr::BrIf(Label(0)),
                // block to check first canary
                Instr::Block(BlockType(None)),
                // decrease ptr by 12
                Instr::Local(LocalOp::Get, chunk_data_ptr),
                Instr::Const(Val::I32(12)),
                Instr::Numeric(NumericOp::I32Sub),
                Instr::Local(LocalOp::Set, chunk_data_ptr),
                // retrieve ptr to first canary (param_0 - 8)
                Instr::Local(LocalOp::Get, chunk_data_ptr),
                Instr::Const(Val::I32(4)),
                Instr::Numeric(NumericOp::I32Add),
                // check first parameter
                Instr::Load(
                    LoadOp::I64Load,
                    Memarg {
                        alignment_exp: 0,
                        offset: 0,
                    },
                ),
                Instr::Numeric(NumericOp::I64Eqz),
                Instr::BrIf(Label(0)),
                //Instr::Const(Val::I32(21)), // return 21
                //Instr::Call(proc_exit_idx),
                Instr::Unreachable,
                Instr::End,
                // block to check the second canary
                Instr::Block(BlockType(None)),
                // fetch the size of the chunk
                Instr::Local(LocalOp::Get, chunk_data_ptr),
                Instr::Load(
                    LoadOp::I32Load,
                    Memarg {
                        alignment_exp: 0,
                        offset: 0,
                    },
                ),
                // get the index of the last data item + 12 bytes (since param 0 is real data ptr + 12)
                Instr::Local(LocalOp::Get, chunk_data_ptr),
                Instr::Numeric(NumericOp::I32Add),
                // get the index of the post-chunk canary
                Instr::Const(Val::I32(8)),
                Instr::Numeric(NumericOp::I32Sub),
                // check the canary value
                Instr::Load(
                    LoadOp::I64Load,
                    Memarg {
                        alignment_exp: 0,
                        offset: 0,
                    },
                ),
                Instr::Numeric(NumericOp::I64Eqz),
                Instr::BrIf(Label(0)),
                Instr::Unreachable,
                Instr::End,
                Instr::End,
                Instr::End,
            ]
        }

        let free_p0 = get_param_n_idx(free, 0);
        let free_prefix = get_canary_check_block(free_p0, disable_canary_glob);
        insert_prefix(free, free_prefix);

        let mut realloc = find_func_by_name("dlrealloc", m.functions_mut());
        realloc = match realloc {
            None => find_func_by_name("realloc", &mut m.functions_mut()),
            x => x,
        };
        match realloc {
            Some((_, realloc)) => {
                // the ptr
                let realloc_p0 = get_param_n_idx(realloc, 0);
                // the size
                let realloc_p1 = get_param_n_idx(realloc, 1);
                let realloc_size = realloc.add_fresh_local(ValType::I32);

                returns_to_outer_block_jmp(realloc);

                let mut realloc_prefix = get_canary_check_block(realloc_p0, disable_canary_glob);

                realloc_prefix.append(&mut vec![
                    // reserve another 20 bytes for the two canaries and the size
                    Instr::Local(LocalOp::Get, realloc_p1),
                    Instr::Const(Val::I32(20)),
                    Instr::Numeric(NumericOp::I32Add),
                    Instr::Local(LocalOp::Tee, realloc_size),
                    Instr::Local(LocalOp::Set, realloc_p1),
                    // disable canary checking while the body of realloc is executing
                    Instr::Const(Val::I32(1)),
                    Instr::Global(GlobalOp::Set, disable_canary_glob),
                ]);
                insert_prefix(realloc, realloc_prefix);

                // reuse the malloc postfix
                let mut post_fix = get_malloc_postfix(realloc, realloc_size, None);
                // re-enable the canary checking
                post_fix.push(Instr::Const(Val::I32(0)));
                post_fix.push(Instr::Global(GlobalOp::Set, disable_canary_glob));
                insert_postfix(realloc, &mut post_fix);
            }
            None => println!("realloc not detected. Skipping instrumentation"),
        };

        let mut calloc = find_func_by_name("dlcalloc", m.functions_mut());
        calloc = match calloc {
            None => find_func_by_name("calloc", &mut m.functions_mut()),
            x => x,
        };

        match calloc {
            Some((_, calloc)) => {
                // the ptr
                let calloc_p0 = get_param_n_idx(calloc, 0);
                // the size
                let calloc_p1 = get_param_n_idx(calloc, 1);
                let calloc_size = calloc.add_fresh_local(ValType::I32);

                // set to 1 if calloc overflows
                let calloc_overflow = calloc.add_fresh_local(ValType::I32);

                returns_to_outer_block_jmp(calloc);

                // calloc (nitems, item_size)
                let calloc_prefix = vec![
                    // disable canary insertion in malloc
                    Instr::Const(Val::I32(1)),
                    Instr::Global(GlobalOp::Set, disable_canary_insertion_in_malloc),
                    // check if nitems * item_size overflows.
                    // In that case, calloc should fail, so it's essential we don't modify the args.
                    Instr::Local(LocalOp::Get, calloc_p0),
                    Instr::Numeric(NumericOp::I64ExtendI32U),
                    Instr::Local(LocalOp::Get, calloc_p1),
                    Instr::Numeric(NumericOp::I64ExtendI32U),
                    Instr::Numeric(NumericOp::I64Mul),
                    Instr::Const(Val::I64(0xFFFFFFFF)),
                    Instr::Numeric(NumericOp::I64GeU),
                    Instr::If(BlockType(None)),
                    Instr::Const(Val::I32(1)),
                    Instr::Local(LocalOp::Set, calloc_overflow),
                    Instr::Else,
                    Instr::Const(Val::I32(0)),
                    Instr::Local(LocalOp::Set, calloc_overflow),
                    // calloc (nitems, item_size) = calloc (1, item_size * nitems)
                    Instr::Local(LocalOp::Get, calloc_p0),
                    Instr::Local(LocalOp::Get, calloc_p1),
                    Instr::Numeric(NumericOp::I32Mul),
                    Instr::Local(LocalOp::Set, calloc_p1),
                    Instr::Const(Val::I32(1)),
                    Instr::Local(LocalOp::Set, calloc_p0),
                    // reserve another 20 bytes for canaries and size
                    Instr::Local(LocalOp::Get, calloc_p1),
                    Instr::Const(Val::I32(20)),
                    Instr::Numeric(NumericOp::I32Add),
                    Instr::Local(LocalOp::Tee, calloc_size),
                    Instr::Local(LocalOp::Set, calloc_p1),
                    Instr::End,
                ];

                insert_prefix(calloc, calloc_prefix);

                // reuse the malloc postfix
                let mut post_fix = get_malloc_postfix(
                    calloc,
                    calloc_size,
                    Some(vec![
                        Instr::Local(LocalOp::Get, calloc_overflow),
                        // check for overflow
                        Instr::Const(Val::I32(1)),
                        Instr::Numeric(NumericOp::I32Eq),
                        // returns tmp_result_local pushed in malloc postfix
                        Instr::BrIf(Label(0)),
                    ]),
                );
                // re-enable the canary insertion in malloc
                post_fix.push(Instr::Const(Val::I32(0)));
                post_fix.push(Instr::Global(
                    GlobalOp::Set,
                    disable_canary_insertion_in_malloc,
                ));
                insert_postfix(calloc, &mut post_fix);
            }
            None => println!("calloc not detected. Skipping instrumentation"),
        };
        let _ = m.to_file(output).expect("Fail to encode the Wasm Module!");
    }else if let Err(error) = result {
        log_error!("Fail to load the Module: {:?}", error);
    }

}

static CANARY_ALLOC_BLOCK_SIZE: i32 = 16;
static CANARY_SIZE_BYTES: u32 = 8;

/**
 * Add a stack canary check to functions.
 * An 8-byte all-zero value is pushed to the stack at function entry.
 * If, at function exit, this value has changed, then the program will terminate.
 *
 * The instrumentation wraps the function body in a block.
 * Every return in the block is changed to a break that exits the block.
 * Instructions for inserting the canary are inserted before the block,
 * and instructions for checking the canary are inserted after.
 */
pub fn instrument_with_stack_canary_check(input: &str, output: &str) {
    let result = Module::from_file(input);
    let mut proc_exit_idx: Idx<Function> = Idx::<Function>::from(0);
    if let Ok(mut module) = result {
        for (idx, func) in module.functions() {
            match func.import() {
                None => continue,
                Some((import_module,import_name)) => {
                    if (import_module == "wasi_snapshot_preview1".to_string() && import_name == "proc_exit".to_string()) {
                        proc_exit_idx = idx;
                    }
                },
            }
        }
        let m: &mut Module = &mut module;
        let stack_ptr_idx = find_stack_ptr(m);
        //let rng = StdRng::seed_from_u64(1542);
        let mut rng = rand::thread_rng();
        let base: i64 = 2;
        let distribution = Uniform::from(0..(base.pow((CANARY_SIZE_BYTES * 8) - 2)));

        m.functions_mut().for_each(|(_, f)| {
            if requires_canary_check(f, &stack_ptr_idx) {
                let canary_value = distribution.sample(&mut rng);

                if f.code().is_some() {
                    returns_to_outer_block_jmp(f);

                    // create canary insertion instructions
                    insert_prefix(
                        f,
                        vec![
                            Instr::Global(GlobalOp::Get, stack_ptr_idx),
                            Instr::Const(Val::I32(CANARY_ALLOC_BLOCK_SIZE)),
                            Instr::Numeric(NumericOp::I32Sub),
                            Instr::Global(GlobalOp::Set, stack_ptr_idx),
                            Instr::Global(GlobalOp::Get, stack_ptr_idx),
                            Instr::Const(Val::I64(canary_value)),
                            Instr::Store(
                                StoreOp::I64Store,
                                Memarg {
                                    alignment_exp: 0,
                                    offset: 0,
                                },
                            ),
                        ],
                    );

                    // if the return type is non-void (tmp_result_local exists)
                    // then insert an instruction inserting the block return value
                    // into this tmp local.
                    let mut post_fix: Vec<Instr> = vec![];

                    // create canary check instructions
                    post_fix.append(&mut vec![
                        Instr::Block(BlockType(None)),
                        Instr::Global(GlobalOp::Get, stack_ptr_idx),
                        Instr::Load(
                            LoadOp::I64Load,
                            Memarg {
                                alignment_exp: 0,
                                offset: 0,
                            },
                        ),
                        Instr::Const(Val::I64(canary_value)),
                        Instr::Numeric(NumericOp::I64Eq),
                        Instr::BrIf(Label(0)),
                        //Instr::Const(Val::I32(21)), // return 22
                        //Instr::Call(proc_exit_idx),
                        Instr::Unreachable,
                        Instr::End,
                        Instr::Global(GlobalOp::Get, stack_ptr_idx),
                        Instr::Const(Val::I32(CANARY_ALLOC_BLOCK_SIZE)),
                        Instr::Numeric(NumericOp::I32Add),
                        Instr::Global(GlobalOp::Set, stack_ptr_idx),
                    ]);

                    insert_postfix(f, &mut post_fix);
                }
            }
        });
        let _ = m.to_file(output).expect("Fail to encode the Wasm Module!");
    }else if let Err(error) = result {
        log_error!("Fail to load the Module: {:?}", error);
    }
}

fn find_stack_ptr(m: &Module) -> Idx<Global> {
    //TODO is the stack pointer always the first global?
    m.globals()
        .nth(0)
        .expect("could not detect stack pointer")
        .0
}

fn requires_canary_check(f: &Function, stack_ptr: &Idx<Global>) -> bool {
    // emscripten adds the stackAlloc and stackRestore functions
    // that both break the invarint that the stack size prior to function
    // entrance is equal to the stack size after function exit.
    // The stackAlloc function is used to, for example, allocate space for argv
    // before the main function is called.
    let breaks_stack_invariant = is_stack_alloc(f, stack_ptr) || is_stack_restore(f, stack_ptr);
    // we check instr_count > 1 since all functions implicitly have an end instruction.
    f.instr_count() > 1 && !breaks_stack_invariant
    // possible other optimizations?
    // TODO check if f even uses the stack pointer.
}

/**
 * Checks if the instructions of f matches the instructions of the emscripten stackAlloc function.
 * This function will break if the instructions of stackAlloc changes.
 */
fn is_stack_alloc(f: &Function, stack_ptr: &Idx<Global>) -> bool {
    let instrs = f.instrs();
    if instrs.len() != 9 {
        return false;
    }

    match (
        &instrs[0], &instrs[1], &instrs[2], &instrs[3], &instrs[4], &instrs[5], &instrs[6],
        &instrs[7], &instrs[8],
    ) {
        (
            Instr::Global(GlobalOp::Get, sp1),
            Instr::Local(LocalOp::Get, _),
            Instr::Numeric(NumericOp::I32Sub),
            Instr::Const(Val::I32(-16)),
            Instr::Numeric(NumericOp::I32And),
            Instr::Local(LocalOp::Tee, l1),
            Instr::Global(GlobalOp::Set, sp2),
            Instr::Local(LocalOp::Get, l1_alt),
            Instr::End,
        ) => sp1 == stack_ptr && sp2 == stack_ptr && l1 == l1_alt,
        _ => false,
    }
}

fn is_stack_restore(f: &Function, stack_ptr: &Idx<Global>) -> bool {
    let instrs = f.instrs();
    if instrs.len() != 3 {
        return false;
    }

    match (&instrs[0], &instrs[1], &instrs[2]) {
        (Instr::Local(LocalOp::Get, _), Instr::Global(GlobalOp::Set, sp), Instr::End) => {
            sp == stack_ptr
        }
        _ => false,
    }
}

/**
 * Returns None for void functions
 */
fn get_return_type(f: &Function) -> Option<ValType> {
    if f.type_.results.len() > 0 {
        // There can always only be 1 return type.
        Some(f.type_.results[0])
    } else {
        None
    }
}

struct ReturnInstrLoc {
    // The index of the return instruction relative to the first instruction
    idx: usize,
    // The depth of the scope containing the return instruction.
    // where 0 is the scope of the function body.
    level: i32,
}

/**
 * wraps instrs in a block and replaces all returns in instrs with a jmp to that block
 */
fn returns_to_outer_block_jmp(f: &mut Function) {
    let return_type = get_return_type(f);
    if let Some(code) = f.code_mut() {
        let instrs = &mut code.body;
        let return_locs = get_return_instr_locs(&instrs);

        for r_loc in return_locs {
            instrs[r_loc.idx] = Instr::Br(Label(r_loc.level as u32));
        }

        // insert the block wrapper around the function body.
        instrs.insert(0, Instr::Block(BlockType(return_type)));
        instrs.push(Instr::End);
    }
}

fn get_return_instr_locs(instrs: &Vec<Instr>) -> Vec<ReturnInstrLoc> {
    let mut level = 0;
    let return_instr_locs = instrs
        .iter()
        .zip(0..instrs.len())
        .fold::<Vec<ReturnInstrLoc>, _>(vec![], |mut acc, (instr, idx)| {
            match instr {
                Instr::Return => acc.push(ReturnInstrLoc { level, idx }),
                Instr::Block(_) | Instr::If(_) | Instr::Loop(_) => level += 1,
                Instr::End => level -= 1,
                _ => (),
            };
            acc
        });
    assert!(
        level == -1,
        "Function body unexpectedly does not end in the function body scope"
    );
    return_instr_locs
}
