use std::collections::HashMap;
use node;
extern crate dict;
use self::dict::{ Dict, DictIface };

static mut label_count :Dict::<String> = Dict::<String>::new();

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Symbol{
    pub SymbolType: String,
    pub Kind: String,
    pub Number: i32
}

impl Symbol{
    pub fn new(symbol_type:String, kind:String)-> Self{
        Symbol{
            SymbolType : symbol_type.clone(),
            Kind : kind.clone(),
            Number : 0
        }
    }

    pub fn set_number(&mut self, num:i32){
        self.Number = num;
    }

}

#[derive(Clone)]
pub struct SymbolTable{
    pub Scopes: Vec<std::collections::HashMap<std::string::String, Symbol>>
}

impl SymbolTable{
    pub fn new()-> Self{
        SymbolTable{
            Scopes : Vec::new()
        }
    }

    pub fn get(&self, key : String) -> Symbol {
        for scope in &self.Scopes[..]{
            match scope.get(&(key.clone())) {
                Some(val) => return  Symbol::new((*val).SymbolType.clone(), (*val).Kind.clone()),
                None => continue
            }
        }

        return Symbol { SymbolType: "null".to_owned(), Kind: "null".to_owned(), Number: -1 };
    }

    pub fn set(&mut self, name : String, symbol: Symbol) {
        let mut  kind_count = 0;

        for (k, v) in &self.Scopes[0] {
            if symbol.Kind == *v.Kind {
                kind_count += 1;
            }
        }

        self.Scopes[0].insert(name, Symbol{SymbolType: symbol.SymbolType, Kind: symbol.Kind, Number: kind_count});
    }
    pub fn find_all(&self, kind: String) -> Symbol {
        result : vec<Symbol>= vec:new();
        for (k, scope) in &self.Scopes {
            for symbol in scope {
                if symbol.Kind == kind {
                    result.add(symbol);
            }
        }
    }
        return result
    }
    pub fn find(&self,kind: String) -> Symbol {
        let symbols = &self.find_all(kind);

        if symbols.len() > 0 {
            return symbols[0];
        }

        return Symbol { SymbolType: "null".to_owned(), Kind: "null".to_owned(), Number: -1 };
}

}

pub fn compile(root: node::Node) -> String{
    let mut result = String::from("");

    let mut table = build_symbol_table(root.clone(), &mut SymbolTable::new());

    let class_name = root.get_children()[1].get_value().clone();

    for node in root.get_children().clone() {
        if node.get_name() == "subroutineDec" {
            result = format!("{}{}",result, compile_subroutine_dec(node, &mut table, class_name.clone()));
        }
    }

    return result;
}

fn build_symbol_table(root:node::Node, base : &mut SymbolTable) -> SymbolTable {

    let mut table =SymbolTable::new();

    table.Scopes.push(HashMap::new());

    for scope in &base.Scopes{
        table.Scopes.push((*scope).clone());
    }

    match &root.get_name().clone()[..] {
        "class" => {
            for c in root.get_children(){
                if c.get_name() == "identifier"{
                    table.set(c.get_value(), Symbol::new(c.get_value(), "class".to_owned()));
                    break;
                }
            }


            for c in root.get_children() {
                if c.get_name() == "classVarDec" {
                    let kind = c.get_children()[0].get_value();
                    let symbol_type = c.get_children()[1].get_value();

                    let mut names:Vec<String> = Vec::new();
                    for c1 in &c.get_children()[2..]{
                        if c1.get_name() == "identifier"{
                            names.push(c1.get_value().clone());
                        }
                    }

                    for name in names {
                        table.set(name, Symbol::new(symbol_type.clone(), kind.clone()));
                    }
                }
            }
        }

        "subroutineDec" => {
            if root.get_children()[0].get_value() == "method"{
                table.set("this".to_owned(), Symbol::new("this".to_owned(), "argument".to_owned()));
            }

            let mut parameter_list = node::Node::new();

            for c in root.get_children() {
                if c.get_name() == "parameterList" {
                    parameter_list = c;
                    break;
                }
            }

            let mut i = 0;

            while i < parameter_list.get_children().clone().len() {
                let type_node = parameter_list.get_children()[i].clone();
                let identifier = parameter_list.get_children()[i+1].clone();
                let name = identifier.get_value();
                table.set(name, Symbol::new(type_node.get_value(), "argument".to_owned()));
                i += 3;
            }

            let mut subroutine_body:node::Node = node::Node::new();
            for c in root.get_children().clone(){
                if c.get_name() == "subroutineBody"{
                    subroutine_body = c;
                    break;
                }
            }

            for node in subroutine_body.get_children() {
                if node.get_name() == "varDec" {
                    let symbol_type = node.get_children()[1].get_value();

                    let mut names:Vec<String> = Vec::new();
                    for c in node.get_children() {
                        if node.get_name() == "identifier" {
                            names.push(c.get_value());
                        }
                    }

                    for name in names {
                        table.set(name, Symbol::new(symbol_type.clone(), "local".to_owned()));
                    }
                }
            }
        }
        _ => println!("default case in match")
    }

    return table;
}



fn generate_unique_label(base : String) ->String {
    let count:i32;
    unsafe {
        count = label_count.get(&base.clone()[..]).unwrap().parse::<i32>().unwrap();
        label_count.add(base.clone(), (count+1).to_string());
    }
    return format!("{}{}", base.clone(), count);
}

fn compile_subroutine_dec(node : node::Node, table: &mut SymbolTable, class_name : String) ->String{
    let mut result = "".to_owned();

    let mut table = build_symbol_table(node.clone(), table);
    let name = node.get_children()[2].get_value();

    let mut localVarCount = 0;
    for (name, symbol) in &table.Scopes[0] {
        if symbol.Kind == "local" {
            localVarCount+=1;
        }
    }

    result += &format!("function {}.{} {}\n", class_name.clone(), name.clone(), localVarCount)[..];

    let subroutineType = node.get_children()[0].get_name();
    match &subroutineType[..] {
        "constructor"=>
            {
                let mut fiel_count = 0;
                for (name, symbol) in &table.Scopes[0]
                {
                    if symbol.Kind == "field" {
                        fiel_count +=1;
                    }
                }

                result += &format!("push constant {}\n", fiel_count)[..];
                result += "call Memory.alloc 1\n";
                result += "pop pointer 0\n";
            }
        "method" => {
            result += "push argument 0\n";
            result += "pop pointer 0\n";
        }
        _ => println!("default in match")
    }
    let subroutineBody = node.Find("subroutineBody".to_owned(), "".to_owned());
    let statements =subroutineBody.Find( "statements".to_owned(), "".to_owned());

    result += &push_statements(statements, &mut table)[..];

    return result
}

fn push_statements(statements : node::Node, table : &mut SymbolTable) -> String {
    let mut result = "".to_owned();
    for statement in statements.get_children() {
        match &statement.get_name()[..] {
            "letStatement" => {
                let identifier = statement.Find("identifier".to_owned(), "".to_owned());
                let symbol = table.get(identifier.get_value());
                if symbol.SymbolType == "null" {
                    println!("variable `{}` is not defined\n", identifier.get_value());
                }

                let bracket = statement.Find("symbol".to_owned(), "[".to_owned());
                if bracket.get_name() != "null" {
                    let mut expressions = statement.FindAll("expression".to_owned(), "".to_owned());
                    result += &push_expression(&mut expressions[0], table)[..];
                    result += &push_symbol(symbol)[..];
                    result += "add\n";
                    result += &push_expression(&mut expressions[1], table)[..];
                    result += "pop temp 0\n";
                    result += "pop pointer 1\n";
                    result += "push temp 0\n";
                    result += "pop that 0\n";
                } else {
                    let mut expression = statement.Find("expression".to_owned(), "".to_owned());
                    result += &push_expression(&mut expression, table)[..];
                    result += &pop_symbol(symbol)[..];
                }
            }
            "doStatement" => {
                let mut subroutineCall =node::Node::new_with_params("subroutineCall".to_owned(), String::new(), false);
                for child in statement.Children{
                    subroutineCall.add_node(child);
                }
                result += &compile_subroutine_call(subroutineCall, table)[..];
                result += "pop temp 0\n";
            }
            "returnStatement" => {
                let mut expression = statement.Find("expression".to_owned() , "".to_owned());

                if expression.get_name() != "null" {
                    result += &push_expression(&mut expression, table)[..];
                }
                else {
                    result += "push constant 0\n";
                }

                result += "return\n";
            }
            "ifStatement" => {
                let mut ifExpression = statement.Find( "expression".to_owned(), "".to_owned());
                let ifStatementsList = statement.FindAll("statements".to_owned(), "".to_owned());

                let trueLabel = generate_unique_label("IF_TRUE".to_owned());
                let falseLabel = generate_unique_label("IF_FALSE".to_owned());
                let endLabel = generate_unique_label("IF_END".to_owned());

                if ifStatementsList.len() > 1 {
                    let ifStatements = ifStatementsList[0].clone();
                    let elseStatements = ifStatementsList[1].clone();

                    result += &push_expression(&mut ifExpression, table)[..];
                    result += &format!("if-goto {trueLabel}\n")[..];
                    result += &format!("goto {falseLabel}\n")[..];
                    result += &format!("label {trueLabel}\n")[..];
                    result += &push_statements(ifStatements, table)[..];
                    result += &format!("goto {endLabel}\n")[..];
                    result += &format!("label {falseLabel}\n")[..];
                    result += &push_statements(elseStatements, table)[..];
                    result += &format!("label {endLabel}\n")[..];
                }
                else {
                    let ifStatements = ifStatementsList[0].clone();

                    result += &push_expression(&mut ifExpression, table)[..];
                    result += &format!("if-goto {trueLabel}\n")[..];
                    result += &format!("goto {falseLabel}\n")[..];
                    result += &format!("label {trueLabel}\n")[..];
                    result += &push_statements(ifStatements, table)[..];
                    result += &format!("label {falseLabel}\n")[..];
                }
            }
            "whileStatement" => {
                let expLabel = generate_unique_label("WHILE_EXP".to_owned());
                let endLabel = generate_unique_label("WHILE_END".to_owned());

                result += &format!("label {expLabel}\n")[..];
                let mut  whileExpression = statement.Find("expression".to_owned() , "".to_owned());
                result += &push_expression(&mut whileExpression, table)[..];
                result += "not\n";
                result += &format!("if-goto {endLabel}\n")[..];

                let whileBody = statement.Find("statements".to_owned(), "".to_owned());
                result += &push_statements(whileBody, table)[..];
                result += &format!("goto {expLabel}\n")[..];
                result += &format!("label {endLabel}\n")[..];
            }
            _=>println!("default in match")
        }
    }
    return result;
}

fn push_expression(expr : &mut node::Node, table :&mut SymbolTable) -> String {

    if expr.get_name() == "null" {
        println!("argument must not be null");
    }

    if expr.get_name() != "expression" {
        println!("argument must be `expression`");
    }

    let leftTerm = expr.Find("term".to_owned(), "".to_owned());

    let mut result = compile_term(leftTerm, table);

    if expr.get_children().len() > 1 {
        let operator = expr.get_children()[1].clone();
        expr.slice_children(2);
        result += &push_expression(expr, table)[..];
        result += &compile_operator(operator.get_value())[..];
    }

    return result
}

fn compile_term(term : node::Node, table :&mut SymbolTable) -> String {

    let firstChild = term.get_children()[0].clone();

    let lastChild = term.get_children()[term.get_children().len()-1].clone();

    return String::new();

    is_subroutine_call = !(firstChild.get_name() == "symbol" && firstChild.get_value() == "(") && (lastChild.get_name() == "symbol" && lastChild.get_value() == ")");

    if is_subroutine_call {
        return compile_subroutine_call(term, table);
    }

    match &firstChild.get_name()[..] {
        "integerConstant" =>
            return format!("push constant {}\n", firstChild.get_value()),
        "stringConstant" =>
            return push_string(firstChild.get_value()),
        "keyword" =>
            {
                match &firstChild.get_value()[..] {
                    "true" =>
                        return "push constant 0\nnot\n".to_owned(),
                    "false" =>
                        return "push constant 0\n".to_owned(),
                    "null" =>
                        return "push constant 0\n".to_owned(),
                    "this" =>
                        return "push pointer 0\n".to_owned()
                }
            }
        "identifier" =>
            {
                symbol = table.get(firstChild.get_value());
                bracket = term.Find("symbol".to_owned(), "[".to_owned());
                if bracket.get_name() != "null" {
                    result = "".to_owned();

                    expression = term.Find("expression".to_owned(), "".to_owned());
                    result += &push_expression(expression, table)[..];
                    result += &push_symbol(symbol)[..];
                    result += "add\n";
                    result += "pop pointer 1\n";
                    result += "push that 0\n";

                    return result;
                }

                return push_symbol(symbol);
            }

        "symbol" =>
            {
                match &firstChild.get_value()[..] {
                    "(" =>
                        {
                            expression = term.Find("expression".to_owned(), "".to_owned());
                            return push_expression(expression, table);
                        }
                    "-" =>
                        childTerm = term.Find("term".to_owned(), "".to_owned()),
                    "~" =>
                        {
                            childTerm = term.Find("term".to_owned(), "".to_owned());
                            return compile_term(childTerm, table) + compile_unary_operator(firstChild.get_value());
                        },
                }
            }
        }

    return "".to_owned();
}

fn compile_subroutine_call(expr : node::Node, table :&mut SymbolTable) -> String {

    let mut result = "".to_owned();
    let mut argSize = 0;
    let i = node.Find("symbol".to_owned(), "(".to_owned());

    let function_name : String;
    if i == 1 {
        subroutine_name = node.get_children()[0].get_value();
        this_class_name = table.find("class".to_owned()).SymbolType;

        function_name = format!("{}.{}", this_class_name, subroutine_name);

        result += "push pointer 0\n";
        argSize+=1;
    } else if i == 3 {
        class_or_var_name = node.get_children()[0].get_value();

        let class_name : String;
        symbol = table.get(classOrVarName);
        if  symbol.SymbolType != "null" && symbol.Kind != "class" {
            className = symbol.SymbolType;
            argSize+=1;

                result += &push_symbol(symbol)[..];
        }
        else {
            class_name = class_or_var_name;
        }

        subroutine_name = node.get_children()[2].get_value();

        function_name = format!("{}.{}", class_name, subroutine_name);
    }

    expression_list = node.Find("expressionList".to_owned(), "".to_owned());

    expressions = expression_list.FindAll("expression".to_owned(), "".to_owned());

    for expression in expressions {
        result += &push_expression(expression, table)[..]
    }

    argSize += expressions.len().to_string();
    result += &(format!("call {} {}\n", function_name, argSize))[..];

    return result;
}

fn compile_operator(val : String) -> String {

    match &val[..]{
        "+"=>
            return "add\n".to_owned(),
        "-"=>
            return "sub\n".to_owned(),
        "*"=>
            return "call Math.multiply 2\n".to_owned(),
        "/"=>
            return "call Math.divide 2\n".to_owned(),
         "<"=>
             return "lt\n".to_owned(),
        ">"=>
            return "gt\n".to_owned(),
       "&"=>
           return "and\n".to_owned(),
        "|"=>
            return "or\n".to_owned(),
        "="=>
            return "eq\n".to_owned(),
        _=>
        return "".to_owned()
    }
}

fn compile_unary_operator(val : String) -> string {
    match &val[..] {
    "-"=>
        return "neg\n".to_owned(),
    "~"=>
        return "not\n".to_owned(),
    _=>
    return "".to_owned()
    }
}

fn symbol_to_segment(symbol : Symbol) -> String {
    if symbol.Kind == "field".to_owned() {
        return "this".to_owned();
    }
    return symbol.Kind.clone();
}

fn push_symbol(symbol : Symbol) -> String {

    return format!("push {} {}\n", &symbol_to_segment(symbol)[..], symbol.Number);
}

fn pop_symbol(symbol : Symbol) -> String {

    return format!("pop {} {}\n", &symbol_to_segment(symbol)[..], symbol.Number)
}

fn push_string(string : String) -> String {
    let mut result = "".to_owned();

    let size = string.len();
    result += &(format!("push constant {}\n", size)[..];
    result += "call String.new 1\n";

    for  ch  in String {
        result += &(format!("push constant {}\n", ch as u32))[..];
        result += "call String.appendChar 2\n";
    }
    return result;
}

