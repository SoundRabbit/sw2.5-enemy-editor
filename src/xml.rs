pub struct Node {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Child>,
}

enum Child {
    Node(Node),
    Value(String),
}

impl Node {
    pub fn new(name: impl Into<String>) -> Self {
        Node {
            name: name.into(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_attribute(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.attributes.push((name.into(), value.into()));
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(Child::Node(child));
    }

    pub fn add_value(&mut self, value: impl Into<String>) {
        self.children.push(Child::Value(value.into()));
    }

    pub fn to_string(&self) -> String {
        let mut serialized = String::new();
        serialized = serialized + "<" + &self.name;
        for attr in &self.attributes {
            serialized = serialized + " " + &attr.0 + "=\"" + &attr.1 + "\"";
        }
        serialized = serialized + ">";
        for child in &self.children {
            match &child {
                Child::Node(node) => {
                    serialized = serialized + &node.to_string();
                }
                Child::Value(value) => {
                    serialized = serialized + value;
                }
            };
        }
        serialized = serialized + "</" + &self.name + ">";
        serialized
    }
}
