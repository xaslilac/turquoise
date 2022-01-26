struct Scope {
    content: Vec<AstNode>,
}

struct Ident(String);

enum AstNode {
    FnDefinition(Ident),
    Ident(String),
}
