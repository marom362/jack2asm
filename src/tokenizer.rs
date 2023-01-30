use std::fs;
use std::io::{self, BufRead, Write};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
extern crate regex;
use self::regex::Regex;
use std::borrow::Borrow;


static KEYWORDS:&str = "class|constructor|function|method|field|static|var|int|char|boolean|void|true|false|null|this|let|do|if|else|while|return";

static SYMBOLS:&str = r"\{|\}|\(|\)|\[|\]|\.|,|;|\+|-|\*|/|&|<|>|\||=|~";

fn build_regex_str() -> String{

    let mut r = "\\d+|\"[^\"\n]+\"|[a-zA-Z_]\\w*".to_owned();

    r.push_str("|");
    r.push_str(KEYWORDS);

    r.push_str("|");
    r.push_str(SYMBOLS);

    return r;
}

fn remove_comments(file_content:&str)->String{
    let re = Regex::new(r"(?m)\s*//.+$").unwrap();
    let temp = re.replace_all(file_content, "");
    let re2 = Regex::new(r"(?ms)/\*.*?\*/").unwrap();
    let temp2 = re2.replace_all(temp.as_ref(), "");
    let temp3 = temp2.as_ref().to_owned();
    return temp3;
}

fn get_token_type(val:&str)->String {

    let KEYWORDS1:&str = "^class$|^constructor$|^function$|^method$|^field$|^static$|^var$|^int$|^char$|^boolean$|^void$|^true$|^false$|^null$|^this$|^let$|^do$|^if$|^else$|^while$|^return$";

    let SYMBOLS1:&str = r"^\{$|^\}$|^\($|^\)$|^\[$|^\]$|^\.$|^,$|^;$|^\+$|^-$|^\*$|^/$|^&$|^<$|^>$|^\|$|^=$|^~$";

    let re = Regex::new(KEYWORDS1).unwrap();
    if re.is_match(val) == true{
        return "keyword".to_owned();
    }

    let re = Regex::new(SYMBOLS1).unwrap();
    if re.is_match(val) == true{
        return "symbol".to_owned();
    }

    let re = Regex::new(r"^\d+$").unwrap();
    if re.is_match(val) == true{
        return "integerConstant".to_owned();
    }

    let re = Regex::new("^\"[^\"\n]+\"$").unwrap();
    if re.is_match(val) == true{
        return "stringConstant".to_owned();
    }

    let re = Regex::new(r"^[a-zA-Z_]\w*$").unwrap();
    if re.is_match(val) == true{
        return "identifier".to_owned();
    }

    return String::new();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calc_token_val(val:String, token_type:String) -> String{
    let mut result = String::new();

    if token_type.eq("stringConstant"){
        result = (&val[1..(val.len()-1)]).to_owned();
    }
    else{
        result = val;
    }

    let lt_r = Regex::new(r"<").unwrap();
    let gt_r =  Regex::new(r">").unwrap();
    let amp_r =  Regex::new(r"&").unwrap();
    let quote_r =  Regex::new("\"").unwrap();

    let temp = amp_r.replace_all(&result[..], "&amp;");
    let temp2 = gt_r.replace_all(temp.as_ref(), "&gt;");
    let temp3 = lt_r.replace_all(temp2.as_ref(), "&lt;");
    let temp4 = quote_r.replace_all(temp3.as_ref(), "&quet;");

    let mut result = String::new();
    result.push_str(temp4.borrow());
    return result;
}


#[derive(Clone)]
pub struct Token{
    Value: String,
    Type: String
}

impl Token{
    pub fn new()-> Self{
        Token{
            Value : String::new(),
            Type : String::new()
        }
    }

    pub fn new_with_params(value_:String, type_:String)-> Self{
        Token{
            Value : value_,
            Type : type_
        }
    }

    pub fn get_value(&self)->String{
        return self.Value.clone();
    }

    pub fn get_type(&self)->String{
        return self.Type.clone();
    }
}

pub struct JackTokenizer{
    xml: String,
    FileName:String,
    pub Tokens: Vec<Token>
}

impl JackTokenizer{
    pub fn new(file_name:String)->Self{
        let mut tokenizer = JackTokenizer{
            xml : String::new(),
            FileName:file_name,
            Tokens:Vec::new()
        };
        println!("start tokenize");
        tokenizer.xml.push_str("<tokens>\n");
        tokenizer.tokenize();
        tokenizer.xml.push_str("</tokens>\n");
        println!("tokenized successfuly!");
        return tokenizer;
    }

    pub fn get_xml(&self) -> String{
        return self.xml.clone();
    }

    pub fn get_tokens(&self) -> Vec<Token>{
        return self.Tokens.to_vec();
    }

    fn tokenize(&mut self){
        if let Ok(lines) = read_lines(&self.FileName[..]) {
            let mut file_str:String = "".to_owned();
            for line in lines {
                file_str.push_str(&line.unwrap()[..]);
                file_str.push_str("\n");
            }
            file_str = remove_comments(&file_str[..]);

            let rs:String = build_regex_str();
            let r = Regex::new(&rs[..]).unwrap();
            for token_val in r.captures_iter(&file_str[..]){
                let mut val = &token_val[0];
                let token_type = get_token_type(val);
                let val_string = String::from(val);
                let x = calc_token_val(val_string, token_type.clone());
                val = &x[..];
                let token:Token = Token::new_with_params(val.to_owned(), token_type.clone());
                self.Tokens.push(token);
                let line = format!("<{}> {} </{}>\n", token_type.clone(), val, token_type);
                self.xml.push_str(&line[..]);
            }
        }
    }
}
