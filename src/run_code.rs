pub fn run_code(Parse<Script> script) -> Project {
    let variables = HashMap::new();
    for stmt in script.syntax().children_with_tokens() {
        match (stmt.kind()) {
            VAR_DELC => "",
            RETURN_STMT => "",
        };
        println!("{:?}", stmt.kind())
    }
}
