use tree_sitter::Language;

pub struct LangDef {
    pub name: &'static str,
    pub extensions: Vec<&'static str>,
}

pub fn rust_lang() -> (Language, LangDef) {
    (tree_sitter_rust::LANGUAGE.into(), LangDef { name: "rust", extensions: vec!["rs"] })
}

pub fn python_lang() -> (Language, LangDef) {
    (tree_sitter_python::LANGUAGE.into(), LangDef { name: "python", extensions: vec!["py"] })
}

pub fn javascript_lang() -> (Language, LangDef) {
    (tree_sitter_javascript::LANGUAGE.into(), LangDef { name: "javascript", extensions: vec!["js", "jsx"] })
}

pub fn c_lang() -> (Language, LangDef) {
    (tree_sitter_c::LANGUAGE.into(), LangDef { name: "c", extensions: vec!["c", "h"] })
}

pub fn go_lang() -> (Language, LangDef) {
    (tree_sitter_go::LANGUAGE.into(), LangDef { name: "go", extensions: vec!["go"] })
}

pub fn all_languages() -> Vec<(Language, LangDef)> {
    vec![
        rust_lang(),
        python_lang(),
        javascript_lang(),
        c_lang(),
        go_lang(),
    ]
}
