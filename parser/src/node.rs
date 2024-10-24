use crate::Token;

pub enum NodeDef {
    Asta(Node),
    Colon(Token),
    At(Token),
    Dollar(Token),
    CaccoOpen(Token),
    CaccoClose(Token),
    Sharp(Token),
    Greater(Token),
    GreaterGreater(Token),
    GreaterEqual(Token),
    Underbar(Token),
    Arrow(Token),
    Sentense(Token),
}

pub struct Node {
    pub token: Token,
    pub parent: Option<Box<NodeDef>>,
    pub children: Vec<NodeDef>,
}

impl Node {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn addChild(&mut self, node: Node) -> &mut Self {
        self.children.push(node);
        self
    }

    pub fn addParent(&mut self, node: Node) -> &mut Self {
        self.parent = Some(Box::new(node));
        self
    }
}
