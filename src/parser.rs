use tokenizer;
use node;


// every function slice tokens after adding token to the tree and add node of a callee function if its name is not "null"

fn parse_class(tokens: &mut Vec<tokenizer::Token>) -> node::Node {
    println!("class");
    // class
    if !(tokens.len() > 3 && tokens[0].get_type() == "keyword" && tokens[0].get_value() == "class") {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    let mut root = node::Node::new_with_params(String::from("class"), String::new(), false);
    root.add_token(tokens[0].get_type(), tokens[0].get_value());
    
    //className
    if tokens[1].get_type() != "identifier" {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_token(tokens[1].get_type(), tokens[1].get_value());

    // {
    if !(tokens[2].get_type() == "symbol" && tokens[2].get_value() == "{") {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_token(tokens[2].get_type(), tokens[2].get_value());
    
    slice_prefix_tokens(tokens, 3);

    // ClassVarDec
    while true {
        let class_var_dec = parse_class_var_dec(tokens);
        if class_var_dec.get_name() == "null"{
            break;
        }
        root.add_node(class_var_dec)
    }

    // SubRoutineDec
    while true {
        let subroutine_dec = parse_subroutine_dec(tokens);
        if subroutine_dec.get_name() == "null"{
            break;
        }
        root.add_node(subroutine_dec)
    }

    // }
    if !(tokens.len() > 0 && tokens[0].get_type() == "symbol" && tokens[0].get_value() == "}") {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_token(tokens[0].get_type(), tokens[0].get_value());

    slice_prefix_tokens(tokens, 1);

    return root;
}
    

fn parse_class_var_dec(tokens: &mut Vec<tokenizer::Token>) -> node::Node {
    println!("classVarDec");
    if !(tokens.len() > 3 && tokens[0].get_type() == "keyword" && (tokens[0].get_value() == "static" || tokens[0].get_value() == "field")) {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    let mut root = node::Node::new_with_params(String::from("classVarDec"), String::new(), false);

    // static | field
    root.add_token(tokens[0].get_type(), tokens[0].get_value());

    slice_prefix_tokens(tokens, 1);

    // type
    let type_ = parse_type(tokens);
    if type_.get_name() == "null"{
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_node(type_);

    //varName
    if tokens[0].get_type() != "identifier" {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_token(tokens[0].get_type(), tokens[0].get_value());

    slice_prefix_tokens(tokens, 1);

    while tokens[0].get_value() == ","{
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
        slice_prefix_tokens(tokens, 1);
        if tokens[0].get_type() != "identifier" {
            return node::Node::new_with_params(String::from("null"), String::new(), false);
        }
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
        slice_prefix_tokens(tokens, 1);
    }

    if tokens[0].get_value() != ";" {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }

    root.add_token(tokens[0].get_type(), tokens[0].get_value());
    slice_prefix_tokens(tokens, 1);
    return root;
}

fn parse_subroutine_dec(tokens: &mut Vec<tokenizer::Token>) -> node::Node {
    println!("subroutineDec");
    if tokens.len() < 2 {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }

    // constructor|function|method
    if !(tokens[0].get_type() == "keyword" && (tokens[0].get_value() == "constructor" || tokens[0].get_value() == "function" || tokens[0].get_value() == "method")){
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }

    let mut root = node::Node::new_with_params(String::from("subroutineDec"), String::new(), false);

    root.add_token(tokens[0].get_type(), tokens[0].get_value());
    slice_prefix_tokens(tokens, 1);

    //void | type
    if tokens[0].get_type() == "keyword" && tokens[0].get_value() == "void"{
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
        slice_prefix_tokens(tokens, 1);
    }
    else{
        let type_ = parse_type(tokens);
        if type_.get_name() == "null"{
            return node::Node::new_with_params(String::from("null"), String::new(), false);
        }
        else{
            root.add_node(type_);
        }
    }

    // subroutine name (indentifier)
    if tokens[0].get_type() != "identifier" {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    println!("-- {}", tokens[0].get_value());
    root.add_token(tokens[0].get_type(), tokens[0].get_value());
    slice_prefix_tokens(tokens, 1);

    //parameter list => '(' parameterList ')'
    // (
    if !(tokens.len() > 0 && tokens[0].get_type() == "symbol" && tokens[0].get_value() == "(") {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_token(tokens[0].get_type(), tokens[0].get_value());
    slice_prefix_tokens(tokens, 1);

    // parameterList
    let param_list = parse_parameter_list(tokens);
    if param_list.get_name() == "null"{
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    else{
        root.add_node(param_list);
    }

    // )
    if !(tokens.len() > 0 && tokens[0].get_type() == "symbol" && tokens[0].get_value() == ")") {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_token(tokens[0].get_type(), tokens[0].get_value());
    slice_prefix_tokens(tokens, 1);
    
    //subroutine body
    let body = parse_subroutine_body(tokens);
    if body.get_name() == "null"{
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    else{
        root.add_node(body);
    }
    
    return root;
}

fn parse_type(tokens: &mut Vec<tokenizer::Token>) -> node::Node {
    println!("type - {} {}",tokens[0].get_type(), tokens[0].get_value());
    if tokens.len() == 0 {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }

    let root = node::Node::new_with_params(tokens[0].get_type(), tokens[0].get_value(), true);

    // int|char|boolean|className(indentifier)
    if tokens[0].get_type() == "keyword" && (tokens[0].get_value() == "int" || tokens[0].get_value() == "char" || tokens[0].get_value() == "boolean"){
        slice_prefix_tokens(tokens, 1);
        return root;
    }
    else if tokens[0].get_type() == "identifier"{
        slice_prefix_tokens(tokens, 1);
        return root;
    }

    return node::Node::new_with_params(String::from("null"), String::new(), false);
}

fn parse_subroutine_body(tokens : &mut Vec<tokenizer::Token>) -> node::Node{
    println!("subroutineBody");
    if tokens.len() == 0 {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }

    let mut root = node::Node::new_with_params(String::from("subroutineBody"), String::new(), false);

    // {
    if !(tokens[0].get_type() == "symbol" && tokens[0].get_value() == "{") {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_token(tokens[0].get_type(), tokens[0].get_value());
    slice_prefix_tokens(tokens, 1);

    // varDec*
    while true {
        let var_dec = parse_var_dec(tokens);
        if var_dec.get_name() == "null"{
            break;
        }
        root.add_node(var_dec);
    }

    //statements
    let var_dec = parse_statements(tokens);
    if var_dec.get_name() == "null"{
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_node(var_dec);

    // }
    if !(tokens[0].get_type() == "symbol" && tokens[0].get_value() == "}") {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_token(tokens[0].get_type(), tokens[0].get_value());
    slice_prefix_tokens(tokens, 1);
    println!("finish parse_subroutine_body");
    return root;
}

fn parse_parameter_list(tokens : &mut Vec<tokenizer::Token>) -> node::Node{
    println!("parameterList");
    let mut root = node::Node::new_with_params(String::from("parameterList"), String::new(), false);

    if tokens[0].get_type() == "symbol" && tokens[0].get_value() == ")" {
        return root;
    }

    // type
    let type_ = parse_type(tokens);
    if type_.get_name() == "null"{
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_node(type_);

    //identifier
    if tokens.len() == 0 {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }

    if tokens[0].get_type() != "identifier" {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_token(tokens[0].get_type(), tokens[0].get_value());
    slice_prefix_tokens(tokens, 1);

    while true {
        // ,
        if !(tokens.len() > 0 && tokens[0].get_type() == "symbol" && tokens[0].get_value() == ","){
            break;
        }
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
        slice_prefix_tokens(tokens, 1);

        // type
        let type_ = parse_type(tokens);
        if type_.get_name() == "null"{
            return node::Node::new_with_params(String::from("null"), String::new(), false);
        }
        root.add_node(type_);
        
        //identifier
        if tokens.len() == 0 {
            return node::Node::new_with_params(String::from("null"), String::new(), false);
        }

        if tokens[0].get_type() != "identifier" {
            return node::Node::new_with_params(String::from("null"), String::new(), false);
        }
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
        slice_prefix_tokens(tokens, 1);

    }
    println!("/parameterList finish");
    return root;
}

fn parse_var_dec(tokens : &mut Vec<tokenizer::Token>) -> node::Node{
    println!("varDec => {}", tokens[0].get_value());
    if tokens.len() == 0 {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }

    // var
    if !(tokens[0].get_type() == "keyword" && tokens[0].get_value() == "var") {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }

    let mut root = node::Node::new_with_params(String::from("varDec"), String::new(), false);
    root.add_token(tokens[0].get_type(), tokens[0].get_value());
    slice_prefix_tokens(tokens, 1);

    // type
    let type_ = parse_type(tokens);
    if type_.get_name() == "null"{
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_node(type_);

    //identifier
    if tokens.len() > 0  && tokens[0].get_type() != "identifier"{
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_token(tokens[0].get_type(), tokens[0].get_value());
    slice_prefix_tokens(tokens, 1);

    while true {
        // ,
        if !(tokens.len() > 0 && tokens[0].get_type() == "symbol" && tokens[0].get_value() == ","){
            break;
        }
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
        slice_prefix_tokens(tokens, 1);

        //identifier
        if tokens.len() == 0 {
            return node::Node::new_with_params(String::from("null"), String::new(), false);
        }

        if tokens[0].get_type() != "identifier" {
            println!("---- {}", tokens[0].get_value());
            return node::Node::new_with_params(String::from("null"), String::new(), false);
        }
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
        slice_prefix_tokens(tokens, 1);
    }

    println!("---- {}", tokens[0].get_value());
    if tokens[0].get_value() != ";" {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_token(tokens[0].get_type(), tokens[0].get_value());
    slice_prefix_tokens(tokens, 1);

    return root;
}

fn parse_statements(tokens : &mut Vec<tokenizer::Token>) -> node::Node{
    println!("statements");
    let mut root = node::Node::new_with_params(String::from("statements"), String::new(), false);

    // statement *
    while true {
        let statement = parse_statement(tokens);
        if statement.get_name() == "null"{
            break;
        }
        root.add_node(statement)
    }
    return root;
}

fn parse_statement(tokens : &mut Vec<tokenizer::Token>) -> node::Node{
    println!("statement");
    println!("parse_statement => {}", tokens[0].get_value());
    match &tokens[0].get_value()[..]{
        "let" => {let let_statement = parse_let_statement(tokens);
            if let_statement.get_name() != "null"{
               return let_statement;
            }
        }
        "if" => {let if_statement = parse_if_statement(tokens);
            if if_statement.get_name() != "null"{
                return if_statement;
            }
        }
        "while" => {let while_statement = parse_while_statement(tokens);
            if while_statement.get_name() != "null"{
                return while_statement;
            }
        }
        "do" => {let do_statement = parse_do_statement(tokens);
            if do_statement.get_name() != "null"{
                return do_statement;
            }
        }
        "return" => {let return_statement = parse_return_statement(tokens);
            if return_statement.get_name() != "null"{
               return return_statement;
            }
        }
            _ => {return node::Node::new_with_params(String::from("null"), String::new(), false); }
    }

    return node::Node::new_with_params(String::from("null"), String::new(), false);
}

fn parse_let_statement(tokens : &mut Vec<tokenizer::Token>) -> node::Node{
    println!("letStatement");
    //let
    if !(tokens.len() > 2 && tokens[0].get_type() == "keyword" && tokens[0].get_value() == "let") {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    let mut root = node::Node::new_with_params(String::from("letStatement"), String::new(), false);
    root.add_token(tokens[0].get_type(), tokens[0].get_value());

    // varName
    if tokens[1].get_type() != "identifier" {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_token(tokens[1].get_type(), tokens[1].get_value());

    slice_prefix_tokens(tokens, 2);
    // [
    if tokens[0].get_type() == "symbol" && tokens[0].get_value() == "[" {
        root.add_token(tokens[0].get_type(), tokens[0].get_value());

        slice_prefix_tokens(tokens, 1);

        //expression
        let expression = parse_expression(tokens);
        if expression.get_name() == "null" {
            return node::Node::new_with_params(String::from("null"), String::new(), false);
        }
        root.add_node(expression);

        // 
        if tokens[0].get_type() == "symbol" && tokens[0].get_value() == "]" {
            root.add_token(tokens[0].get_type(), tokens[0].get_value());
            slice_prefix_tokens(tokens, 1);
        }
    }
    // =
    if tokens[0].get_type() == "symbol" && tokens[0].get_value() == "=" {
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
    }
    slice_prefix_tokens(tokens, 1);

    //expression
    let expression = parse_expression(tokens);
    if expression.get_name() == "null" {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_node(expression);

    // ;
    if tokens[0].get_type() == "symbol" && tokens[0].get_value() == ";" {
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
    }
    slice_prefix_tokens(tokens, 1);
    return root;
}

fn parse_if_statement(tokens : &mut Vec<tokenizer::Token>) -> node::Node{
    println!("ifStatement");
    //if
    if !(tokens.len() > 1 && tokens[0].get_type() == "keyword" && tokens[0].get_value() == "if") {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    let mut root = node::Node::new_with_params(String::from("ifStatement"), String::new(), false);
    root.add_token(tokens[0].get_type(), tokens[0].get_value());

    // (
    if tokens[1].get_type() == "symbol" && tokens[1].get_value() == "(" {
        root.add_token(tokens[1].get_type(), tokens[1].get_value());
    }
    slice_prefix_tokens(tokens, 2);

    //expression
    let expression = parse_expression(tokens);
    if expression.get_name() == "null" {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_node(expression);

    // )
    if tokens[0].get_type() == "symbol" && tokens[0].get_value() == ")" {
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
    }

    // {
    if tokens[1].get_type() == "symbol" && tokens[1].get_value() == "{" {
        root.add_token(tokens[1].get_type(), tokens[1].get_value());
    }
    slice_prefix_tokens(tokens, 2);

    // statements
    let expression = parse_statements(tokens);
    if expression.get_name() == "null" {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_node(expression);

    println!("-- {}", tokens[0].get_value());

    // }
    if tokens[0].get_type() == "symbol" && tokens[0].get_value() == "}" {
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
        slice_prefix_tokens(tokens, 1);
    }


    println!("-- {}", tokens[0].get_value());

    if tokens[0].get_type() == "keyword" && tokens[0].get_value() == "else" {
        // {
        if tokens[1].get_type() == "symbol" && tokens[1].get_value() == "{" {
            root.add_token(tokens[0].get_type(), tokens[0].get_value());
            root.add_token(tokens[1].get_type(), tokens[1].get_value());
            slice_prefix_tokens(tokens, 2);
        }
        else{
            return node::Node::new_with_params(String::from("null"), String::new(), false);
        }

        // statements
        let expression = parse_statements(tokens);
        if expression.get_name() == "null" {
            return node::Node::new_with_params(String::from("null"), String::new(), false);
        }
        root.add_node(expression);

        // }
        if tokens[0].get_type() == "symbol" && tokens[0].get_value() == "}" {
            root.add_token(tokens[0].get_type(), tokens[0].get_value());
            slice_prefix_tokens(tokens, 1);
        }
        else{
            return node::Node::new_with_params(String::from("null"), String::new(), false);
        }
        
    }

    return root;
}


fn parse_while_statement(tokens : &mut Vec<tokenizer::Token>) -> node::Node{
    println!("whileStatement");
    //while
    if !(tokens.len() > 1 && tokens[0].get_type() == "keyword" && tokens[0].get_value() == "while") {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    let mut root = node::Node::new_with_params(String::from("whileStatement"), String::new(), false);
    root.add_token(tokens[0].get_type(), tokens[0].get_value());

    // (
    if tokens[1].get_type() == "symbol" && tokens[1].get_value() == "(" {
        root.add_token(tokens[1].get_type(), tokens[1].get_value());
    }
    slice_prefix_tokens(tokens, 2);

    //expression
    let expression = parse_expression(tokens);
    if expression.get_name() == "null" {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_node(expression);

    // )
    if tokens[0].get_type() == "symbol" && tokens[0].get_value() == ")" {
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
    }

    // {
    if tokens[1].get_type() == "symbol" && tokens[1].get_value() == "{" {
        root.add_token(tokens[1].get_type(), tokens[1].get_value());
    }
    slice_prefix_tokens(tokens, 2);

    //expression
    let expression = parse_statements(tokens);
    if expression.get_name() == "null" {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    root.add_node(expression);

    // }
    if tokens[0].get_type() == "symbol" && tokens[0].get_value() == "}" {
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
    }
    slice_prefix_tokens(tokens, 1);
    return root;
}

fn parse_do_statement(tokens : &mut Vec<tokenizer::Token>) -> node::Node{
    println!("doStatement");
    //while
    if !(tokens.len() > 1 && tokens[0].get_type() == "keyword" && tokens[0].get_value() == "do") {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    let mut root = node::Node::new_with_params(String::from("doStatement"), String::new(), false);
    root.add_token(tokens[0].get_type(), tokens[0].get_value());

    slice_prefix_tokens(tokens, 1);
    
    //subroutineCall
    parse_subroutine_call(tokens, &mut root);

    // ;
    if tokens[0].get_type() == "symbol" && tokens[0].get_value() == ";" {
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
        slice_prefix_tokens(tokens, 1);
    }
    else{
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    
    return root;
}


fn parse_return_statement(tokens : &mut Vec<tokenizer::Token>) -> node::Node{
    println!("returnStatement => {}", tokens[1].get_value());
    //return
    if !(tokens.len() > 1 && tokens[0].get_type() == "keyword" && tokens[0].get_value() == "return") {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    let mut root = node::Node::new_with_params(String::from("returnStatement"), String::new(), false);
    root.add_token(tokens[0].get_type(), tokens[0].get_value());

    slice_prefix_tokens(tokens, 1);

    //expression
    let expression = parse_expression(tokens);
    if expression.get_name() != "null" {
        root.add_node(expression);
    }
    
    // ;
    if tokens[0].get_type() == "symbol" && tokens[0].get_value() == ";" {
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
    }
    slice_prefix_tokens(tokens, 1);
    return root;
}

fn parse_expression(tokens : &mut Vec<tokenizer::Token>) -> node::Node{
    println!("expression => {}", tokens[0].get_value());
    //term
    let term = parse_term(tokens);
    if term.get_name() == "null" {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    let mut root = node::Node::new_with_params(String::from("expression"), String::new(), false);
    root.add_node(term);

    // (op term) *
    let op_vec= vec!["+", "-", "*", "/", "&amp;", "|", "&lt;", "&gt;", "="];
    loop {
        if !(tokens[0].get_type() == "symbol" && op_vec.contains(&&tokens[0].get_value()[..])) {
            return root;
        } else {
            //op
            root.add_token(tokens[0].get_type(), tokens[0].get_value());
            slice_prefix_tokens(tokens, 1);

            //term
            let term = parse_term(tokens);
            if term.get_name() == "null" {
                break;
            }
            root.add_node(term);
        }
    }
    return root;

}

fn parse_term(tokens : &mut Vec<tokenizer::Token>) -> node::Node{
    println!("term");
    println!("parse_term => {}", tokens[0].get_value());
    if !(tokens.len() > 0) {
        return node::Node::new_with_params(String::from("null"), String::new(), false);
    }
    let mut root = node::Node::new_with_params(String::from("term"), String::new(), false);

    let keyWordConstant_vec= vec!["true", "false", "null", "this"];
    if tokens[0].get_type() == "integerConstant" || tokens[0].get_type() == "stringConstant" || keyWordConstant_vec.contains(&&tokens[0].get_value()[..]){
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
        slice_prefix_tokens(tokens, 1);
        return root;
    }

   //identifier
    else if tokens[0].get_type() == "identifier" {
        // varName [expression]
        if tokens[1].get_type() == "symbol" && tokens[1].get_value() == "[" {
            //varName
            root.add_token(tokens[0].get_type(), tokens[0].get_value());

            //[
            root.add_token(tokens[1].get_type(), tokens[1].get_value());
            slice_prefix_tokens(tokens, 2);

            //expression
            let expression = parse_expression(tokens);
            if expression.get_name() == "null" {
                return node::Node::new_with_params(String::from("null"), String::new(), false);
            }
            root.add_node(expression);

            //]
            if tokens[0].get_type() == "symbol" && tokens[0].get_value() == "]" {
                root.add_token(tokens[0].get_type(), tokens[0].get_value());
            }
            slice_prefix_tokens(tokens, 1);
            return root;
        }

        // subRoutineName (expressionList)
        else if tokens[1].get_type() == "symbol" && tokens[1].get_value() == "(" {
            parse_subroutine_call(tokens, &mut root);
            return root;
        }

        // (className | varName).subRoutineName(expressionList)
        else if tokens[1].get_type() == "symbol" && tokens[1].get_value() == "." {
            parse_subroutine_call(tokens, &mut root);
            return root;
        }

        // varName
        else {
            root.add_token(tokens[0].get_type(), tokens[0].get_value());
            slice_prefix_tokens(tokens, 1);
            return root;
        }
    }

    //(expression)
    else if tokens[0].get_type() == "symbol" && tokens[0].get_value() == "("{

        //(
        root.add_token(tokens[0].get_type(), tokens[0].get_value());

        slice_prefix_tokens(tokens, 1);

        // expression
        let expression = parse_expression(tokens);
        root.add_node(expression);

        //)
        if tokens[0].get_type() == "symbol" && tokens[0].get_value() == ")" {
            root.add_token(tokens[0].get_type(), tokens[0].get_value());
            slice_prefix_tokens(tokens, 1);
        }

        return root;
    }

    //unaryOp term
    else if  tokens[0].get_value() == "~" || tokens[0].get_value()=="-"  {

        // ~ | -
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
        slice_prefix_tokens(tokens, 1);
        //term
        let term = parse_term(tokens);
        root.add_node(term);
        return root;
    }

    return node::Node::new_with_params(String::from("null"), String::new(), false);
}

fn parse_expression_list(tokens : &mut Vec<tokenizer::Token>) -> node::Node{
    println!("expressionList => {}", tokens[0].get_value());

    let expression = parse_expression(tokens);
    if expression.get_name() == "null" {
        return node::Node::new_with_params(String::from("expressionList"), String::new(), false);
    }
    let mut root = node::Node::new_with_params(String::from("expressionList"), String::new(), false);
    root.add_node(expression);
    while true {
        if !(tokens[0].get_type() == "symbol" &&  tokens[0].get_value() == "," ) {
            break
        }
        root.add_token(tokens[0].get_type(), tokens[0].get_value());
        slice_prefix_tokens(tokens, 1);
        let expression = parse_expression(tokens);
        root.add_node(expression);
    }
    return root;
}

fn parse_subroutine_call(tokens : &mut Vec<tokenizer::Token>, root : &mut node::Node){
    println!("parse_subroutine_call => {}", tokens[0].get_value());

    //identifier
    if tokens[0].get_type() == "identifier" {
        // subRoutineName (expressionList)
        if tokens[1].get_type() == "symbol" && tokens[1].get_value() == "(" {

            //subRoutineName
            root.add_token(tokens[0].get_type(), tokens[0].get_value());

            //(
            root.add_token(tokens[1].get_type(), tokens[1].get_value());
            slice_prefix_tokens(tokens, 2);

            //expressionList
            let expression_list = parse_expression_list(tokens);
            root.add_node(expression_list);

            //)
            if tokens[0].get_type() == "symbol" && tokens[0].get_value() == ")" {
                root.add_token(tokens[0].get_type(), tokens[0].get_value());
                slice_prefix_tokens(tokens, 1);
            }
            println!("-- {}", tokens[0].get_value());
        }

        // (className | varName).subRoutineName(expressionList)
        else if tokens[1].get_type() == "symbol" && tokens[1].get_value() == "." {

            //className | varName
            root.add_token(tokens[0].get_type(), tokens[0].get_value());

            //.
            root.add_token(tokens[1].get_type(), tokens[1].get_value());
            slice_prefix_tokens(tokens, 2);

            //subRoutineName
            root.add_token(tokens[0].get_type(), tokens[0].get_value());

            //(
            root.add_token(tokens[1].get_type(), tokens[1].get_value());
            slice_prefix_tokens(tokens, 2);

            //expressionList
            let expression_list = parse_expression_list(tokens);
            if expression_list.get_name() == "null" {}
            root.add_node(expression_list);

            //)
            if tokens[0].get_type() == "symbol" && tokens[0].get_value() == ")" {
                root.add_token(tokens[0].get_type(), tokens[0].get_value());
                slice_prefix_tokens(tokens, 1);
            }
            
        }
    }
}

fn slice_prefix_tokens(tokens: &mut Vec<tokenizer::Token>, count:usize){
    tokens.drain(..count);
}

pub fn parse(tokens: &mut Vec<tokenizer::Token>) -> node::Node{
    println!("start parsing");
    return parse_class(tokens);
}

pub fn tokens2xml(root: node::Node) -> String{
    let mut result = "".to_owned();

    if root.get_is_leaf() {
        return format!("<{}> {} </{}>\n", root.get_name().clone(), root.get_value().clone(), root.get_name().clone());
    }

    result = format!("<{}>\n", root.get_name().clone());
    for c in root.get_children() {
        let ret = &tokens2xml(c)[..];
        result.push_str(ret);
    }
    
    let temp = &format!("</{}>\n", root.get_name().clone())[..];
    result.push_str(temp);

    return result;
}



