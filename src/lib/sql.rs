#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub source: String,
    pub path: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sql {
    pub select_clause: Vec<Field>,
    pub from_clause: Vec<Field>,
    pub left_join_clause: Vec<Field>,
    // pub where_clause: Vec<Field>,
}
