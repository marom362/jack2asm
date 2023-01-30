use std::fs;
use std::io::{self, BufRead, Write};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::env;
extern crate dict;
use self::dict::{ Dict, DictIface };

static mut FILE_PATH : String = String::new();
static mut TRUE_LABEL_COUNTER: i32 =0;
static mut FALSE_LABEL_COUNTER: i32 =0;
static mut CALL_LABEL_COUNTER: i32 =0;
static mut SEGMENTS:Dict::<String> = Dict::<String>::new();

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| c.is_digit(10)).collect()
    //let t : String = s.chars().filter(|c| c.is_digit(10)).collect();
    //return t;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn write_line(new:&str, file_name:&str){
    let mut file_content="".to_owned();
    if let Ok(lines) =read_lines(file_name) {
        for line_res in lines {
            if let Ok(line) = line_res {
                file_content.push_str(&line[..]);
                file_content.push_str("\n");
            }
        }
    }
    file_content.push_str(new);
    let mut buffer = File::create(file_name).unwrap();
    buffer.write_all(file_content.as_bytes()).unwrap();
}


fn handle_pop (seg:&str, num:&str, current_vm_name:&str) -> String {
    let segment:&str = seg;
    let index:&str = &remove_whitespace(num)[..];
    let mut line:String;
    if segment.eq("pointer") {
        if index.eq("0"){
            line = format!("@SP\nA=M-1\nD=M\n@THIS\nM=D\n@SP\nM=M-1\n");
        }
        else if index.eq("1") {
            line = format!("@SP\nA=M-1\nD=M\n@THAT\nM=D\n@SP\nM=M-1\n");
        }
        else{
            line = "".to_owned();
        }
        return line;
    }
    let mut temp_index:i32 = index.parse().unwrap();
    temp_index+=5;
    match segment {
        "temp" => line = format!("@SP\nA=M-1\nD=M\n@{}\nM=D\n@SP\nM=M-1\n", temp_index),
        "static" => line = format!("@SP\nA=M-1\nD=M\n@{}.{}\nM=D\n@SP\nM=M-1", current_vm_name, index),
        _ => { // v
            unsafe{
                line = format!("@SP\nA=M-1\nD=M\n@{}\nA=M\n", SEGMENTS.get(segment).unwrap());
            };
            let mut n:i32=0;
            while n < FromStr::from_str(index).unwrap(){
                line.push_str("A=A+1\n");
                n+=1;
            }
            line.push_str("M=D\n@SP\nM=M-1\n");
        }
    }
    return line;

}

fn handle_push(seg:&str, num:&str, current_vm_name:&str) -> String{
    let segment:&str = seg;
    let index:&str = &remove_whitespace(num)[..];
    let line:String;
    if segment.eq("pointer") {
        if index.eq("0"){
            line= format!("@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
        }
        else if index.eq("1") {
            line=format!("@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
        }
        else{
            line = "".to_owned();
        }
        return line;
    }
    let mut temp_index:i32=FromStr::from_str(index).unwrap();
    temp_index+=5;
    match segment {
        "constant" => line = format!("@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", index),
        "temp" => line = format!("@{}\nD=A\n@5\nA=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", temp_index),
        "static" => line = format!("@{}.{}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", current_vm_name, index),
        _ => line = unsafe{format!("@{}\nD=A\n@{}\nA=D+M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", index, SEGMENTS.get(segment).unwrap())}
    }

    return line;
}

fn handle_add() -> String {
    let mut line="".to_owned();
    line.push_str("@SP\nA=M-1\nD=M\nA=A-1\nM=D+M\n@SP\nM=M-1\n");
    return line;
}

fn handle_sub() -> String {
    let mut line="".to_owned();
    line.push_str("@SP\nA=M-1\nD=M\nA=A-1\nM=M-D\n@SP\nM=M-1\n");
    return line;
}

fn handle_neg() -> String {
    let mut line="".to_owned();
    line.push_str("@SP\nA=M-1\nM=-M\n");
    return line;
}

unsafe fn generate_true_label() -> String{
    let str_counter: String=  format!("IF_TRUE{}", TRUE_LABEL_COUNTER);
    TRUE_LABEL_COUNTER +=1;
    return str_counter;
}

unsafe fn generate_false_label()->String{
    let str_counter: String=  format!("IF_FALSE{}", FALSE_LABEL_COUNTER);
    FALSE_LABEL_COUNTER +=1;
    return str_counter;
}

unsafe fn generate_return_label(function_name:&str) ->String{
    let str_counter: String=  format!("{}.ReturnAddress{}", &function_name[..], CALL_LABEL_COUNTER);
    CALL_LABEL_COUNTER +=1;
    return str_counter;
}



fn handle_eq() -> String {
    let line:String;
    unsafe{
        let true_label : &str= &generate_true_label()[..];
        let false_label : &str= &generate_false_label()[..];
        line = format!("@SP\nA=M-1\nD=M\nA=A-1\nD=D-M\n@{}\nD;JEQ\nD=0\n@SP\nA=M-1\nA=A-1\nM=D\n@{}\n0;JMP\n({})\nD=-1\n@SP\nA=M-1\nA=A-1\nM=D\n({})\n@SP\nM=M-1\n", true_label, false_label, true_label, false_label);
    }
    return line;
}

fn handle_gt() -> String {
    let line:String;
    unsafe{
        let true_label : &str= &generate_true_label()[..];
        let false_label : &str= &generate_false_label()[..];
        line = format!("@SP\nA=M-1\nD=M\nA=A-1\nD=M-D\n@{}\nD;JGT\nD=0\n@SP\nA=M-1\nA=A-1\nM=D\n@{}\n0;JMP\n({})\nD=-1\n@SP\nA=M-1\nA=A-1\nM=D\n({})\n@SP\nM=M-1\n", true_label, false_label, true_label, false_label);
    }
    return line;
}

fn handle_lt() -> String {
    let line:String;
    unsafe{
        let true_label : &str= &generate_true_label()[..];
        let false_label : &str= &generate_false_label()[..];
        line = format!("@SP\nA=M-1\nD=M\nA=A-1\nD=M-D\n@{}\nD;JLT\nD=0\n@SP\nA=M-1\nA=A-1\nM=D\n@{}\n0;JMP\n({})\nD=-1\n@SP\nA=M-1\nA=A-1\nM=D\n({})\n@SP\nM=M-1\n", true_label, false_label, true_label, false_label);
    }
    return line;
}

fn handle_and() -> String {
    let mut line="".to_owned();
    line.push_str("@SP\nA=M-1\nD=M\nA=A-1\nM=D&M\n@SP\nM=M-1\n");
    return line;
}

fn handle_or() -> String {
    let mut line="".to_owned();
    line.push_str("@SP\nA=M-1\nD=M\nA=A-1\nM=D|M\n@SP\nM=M-1\n");
    return line;
}

fn handle_not() -> String {
    let mut line="".to_owned();
    line.push_str("@SP\nA=M-1\nM=!M\n");
    return line;
}

fn handle_label (label_name:&str, current_vm_name:&str) -> String {
    let line = format! ("({}.{})", current_vm_name, label_name);
    return line;
}

fn handle_goto (label_name:&str, current_vm_name:&str) -> String {
    let line = format! ("@{}.{}\n0;JMP\n", current_vm_name, label_name);
    return line;
}

fn handle_if_goto (label_name:&str, current_vm_name:&str) -> String {
    let line = format! ("@SP\nM=M-1\nA=M\nD=M\n@{}.{}\nD;JNE\n", current_vm_name, label_name);
    return line;
}

fn handle_function (function_name:&str, num_of_locals:&str) -> String {
    let line = format! ("\n({})\n\n@{}\nD=A\n@{}.End\nD;JEQ\n({}.Loop)\n@SP\nA=M\nM=0\n@SP\nM=M+1\n@{}.Loop\nD=D-1;JNE\n({}.End)\n",
    function_name, num_of_locals, function_name, function_name, function_name, function_name);
    return line;
}

fn handle_call (function_name:&str, num:&str) -> String {
    let line:String;
    let num_of_args_str=&remove_whitespace(num)[..];
    let mut num_of_args:i32=FromStr::from_str(num_of_args_str).unwrap();
    num_of_args = num_of_args + 5;
    unsafe{
        let label= generate_return_label(function_name);
        let call_label =&label[..];
        line = format!("@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@SP\nD=M\n@{}\nD=D-A\n@ARG\nM=D\n@SP\nD=M\n@LCL\nM=D\n@{}\n0; JMP\n({})\n", 
        call_label, num_of_args, function_name, call_label);
    }
    return line;
}

fn handle_return() -> String {
    let mut line="".to_owned();
    line.push_str("@LCL\nD=M\n@5\nA=D-A\nD=M\n@13\nM=D\n\n@SP\nM=M-1\nA=M\nD=M\n@ARG\nA=M\nM=D\n\n@ARG\nD=M\n@SP\nM=D+1\n@LCL\nM=M-1\nA=M\nD=M\n@THAT\nM=D\n\n@LCL\nM=M-1\nA=M\nD=M\n@THIS\nM=D\n\n@LCL\nM=M-1\nA=M\nD=M\n@ARG\nM=D\n\n@LCL\nM=M-1\nA=M\nD=M\n@LCL\nM=D\n\n@13\nA=M\n0; JMP\n");
    return line;
}


fn handle_file(src_file_name:&str){
    let without_type:&str = src_file_name.split(".").collect::<Vec<&str>>().first().unwrap();
    let name:&str=without_type.split("\\").collect::<Vec<&str>>().last().unwrap();
    unsafe{
        write_line(&format!("// file name: {}", name)[..], &FILE_PATH[..]);
    }
    if let Ok(lines) = read_lines(src_file_name) {
        for line_res in lines {
            if let Ok(line) = line_res {
                let command_vec: Vec<&str>=line.split(" ").collect();
                let hack:String;

                match command_vec[0] {
                    "push" => hack = handle_push(command_vec[1], command_vec[2], name),
                    "pop" => hack = handle_pop(command_vec[1], command_vec[2], name),
                    "add" => hack = handle_add(),
                    "sub" => hack = handle_sub(),
                    "neg" => hack = handle_neg(),
                    "eq" => hack = handle_eq(),
                    "gt" => hack = handle_gt(),
                    "lt" => hack = handle_lt(),
                    "and" => hack = handle_and(),
                    "or" => hack = handle_or(),
                    "not" => hack = handle_not(),
                    "label" => hack = handle_label(command_vec[1], name),
                    "goto" => hack = handle_goto(command_vec[1], name),
                    "if-goto" => hack = handle_if_goto(command_vec[1], name),
                    "function" => hack = handle_function(command_vec[1], command_vec[2]),
                    "call" => hack = handle_call(command_vec[1], command_vec[2]),
                    "return" => hack = handle_return(),
                    _ => { println!("{}", line); hack = "".to_owned()}
                }
                let hack_str =&hack[..];
                unsafe{
                    write_line(&format!("// {}", line)[..], &FILE_PATH[..]);
                    write_line(hack_str, &FILE_PATH[..]);
                }
            }
        }
    }
    println!("successfully handled {}", src_file_name);
}

fn create_asm(directory_name:&str){
    let v: Vec<&str> = directory_name.split("\\").collect();
    let name:&str=v.last().unwrap();
    let mut new_file:String=directory_name.to_owned();
    new_file.push_str("\\");
    new_file.push_str(name);
    new_file.push_str(".asm");
    File::create(&new_file).unwrap();
    let temp:&str=&new_file[..];
    unsafe {
        FILE_PATH.push_str(temp);
    }
}

fn vms_to_asm(directory_name:&str){
    let mut files:Vec<String> = Vec::new();
    let mut SYSVM:String = "".to_owned();
    for file in fs::read_dir(directory_name).unwrap() {
        let path_string: String = file.unwrap().path().display().to_string();
        let file_type: &str = path_string.split(".").last().unwrap();
        let name: &str = path_string.split("\\").last().unwrap();
        if file_type == "vm" {
            if name == "Sys.vm"{
                SYSVM = path_string;
            }
            else{
                files.push(path_string);
            }
        }
    }
    if SYSVM == "" {
        if files.len() > 1 {
            println!("ERROR: no Sys.vm file");
        }
        else if files.len() == 1 {
            handle_file(&files[0][..]);
        }
        else {
            println!("ERROR: empty directory");
        }
        return;
    }

    start();
    
    handle_file(&SYSVM[..]);
    for file in files {
        handle_file(&file[..]);
    }

    println!("finish successfully!");
}

fn start(){
    let call_sys:String = handle_call("Sys.init", "0");
    let call_sys_str =&call_sys[..];
    unsafe{
    write_line("@256\nD=A\n@SP\nM=D\n",  &FILE_PATH[..]);
    write_line(call_sys_str, &FILE_PATH[..])
    }
}

fn vm_to_hack(directory_name:&str){
    init_segment_dict();
    create_asm(directory_name);
    vms_to_asm(directory_name);
}

fn init_segment_dict(){
    unsafe{
        SEGMENTS.add("local".to_string(), "LCL".to_string());
        SEGMENTS.add("argument".to_string(), "ARG".to_string());
        SEGMENTS.add("this".to_string(), "THIS".to_string());
        SEGMENTS.add("that".to_string(), "THAT".to_string());
    }
}

pub fn vm2asm(directory_name:&str) {
    vm_to_hack(directory_name);
}
