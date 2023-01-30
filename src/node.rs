
#[derive(Clone)]
pub struct Node{
    Value: String,
    Name: String,
    IsLeaf: bool,
    pub Children: Vec<Node>
}

impl Node{
    pub fn new()-> Self{
        Node{
            Value : String::new(),
            Name : String::new(),
            IsLeaf : false,
            Children : Vec::new()
        }
    }

    pub fn get_name(&self)-> String{
        return self.Name.clone();
    }

    pub fn get_value(&self)-> String{
        return self.Value.clone();
    }

    pub fn get_is_leaf(&self)-> bool{
        return self.IsLeaf;
    }

    pub fn get_children_len(&self)-> usize{
        return self.Children.len();
    }

    pub fn get_children(&self) -> Vec<Node>{
        return self.Children.to_vec();
    }

    pub fn new_with_params(name_:String, value_:String, is_leaf_:bool)-> Self{
        Node{
            Value : value_,
            Name : name_,
            IsLeaf : is_leaf_,
            Children : Vec::new()
        }
    }

    pub fn add_node(&mut self, node_:Node) {
        self.Children.push(node_);
    }

    pub fn add_token(&mut self, type_:String, value_:String) {
        let node_:Node = Node::new_with_params(type_, value_, true);
        self.Children.push(node_);
    }

    pub(crate) fn Find(&self, name : string, value : String) ->Node{
        for child in self.Children {
            if child.get_name() == name && (value == "" || child.get_value() == value) {
                return child.clone();
            }
        }

        return Node::new_with_params(String::from("null"), String::new(), false);
    }

    fn FindAll(&self, name : string, value : String) -> vec<Node> {
        result: vec<Node> = Vec::new();
        for child in self.Children {
            if child.Name == name && (value == ""|| child.Value == value) {
                result.push(child);
            }
        }
        return result;
    }
}