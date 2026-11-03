//!Turns parse tree into HTML

use crate::parser::Node;
use std::collections::HashMap;

///Searches for a variable in the scopes and returns its value if found
fn find_variable(scopes: &Vec<HashMap<String, String>>, name: &str) -> Option<String> {
    for scope in scopes.iter().rev() {
        if let Some(value) = scope.get(name) {
            return Some(value.clone());
        }
    }
    None
}

///Generates HTML from the parse tree
pub fn generate_html(tree: &Vec<Node>) -> Result<String, String> {
    let mut html = String::new();
    let mut scopes: Vec<HashMap<String, String>> = Vec::new();
    let mut closing_tags: Vec<String> = Vec::new();
    let mut i = 0;

    while i < tree.len() {
        let token = tree[i].token.as_str();

        match token {
            "#HAI" => {
                html.push_str("<html>\n");
                scopes.push(HashMap::new());
            }
            "#KBYE" => {
                html.push_str("\n</html>\n");
                scopes.pop();
            }
            "#OBTW" => html.push_str("<!-- "),
            "#TLDR" => html.push_str("-->\n"),
            "#NEWLINE" => html.push_str("<br> "),
            "#MAEK HEAD" => {
                html.push_str("<head>\n");
                closing_tags.push(String::from("</head>\n"));
                scopes.push(HashMap::new());
            }
            "#GIMMEH TITLE" => {
                html.push_str("<title> ");
                closing_tags.push(String::from("</title>\n"));
                scopes.push(HashMap::new());
            }
            "#MAEK PARAGRAF" => {
                html.push_str("\n<p> ");
                closing_tags.push(String::from("</p>\n"));
                scopes.push(HashMap::new());
            }
            "#GIMMEH BOLD" => {
                html.push_str("<b> ");
                closing_tags.push(String::from("</b> "));
                scopes.push(HashMap::new());
            }
            "#MAEK LIST" => {
                html.push_str("\n<ul>\n");
                closing_tags.push(String::from("</ul>\n"));
                scopes.push(HashMap::new());
            }
            "#GIMMEH ITEM" => {
                html.push_str("<li> ");
                closing_tags.push(String::from("</li>\n"));
                scopes.push(HashMap::new());
            }
            "#MKAY" | "#OIC" => {
                if let Some(tag) = closing_tags.pop() {
                    html.push_str(&tag);
                }
                scopes.pop();
            }
            "#GIMMEH LINX" => {
                let address = &tree[i + 1].token;
                html.push_str(&format!("<a href=\"{}\">{}</a> ", address, address));
                i += 2;
            }
            "#GIMMEH VAR" => {
                let name = &tree[i + 1].token;
                match find_variable(&scopes, name) {
                    Some(value) => {
                        html.push_str(&value);
                        html.push(' ');
                    }
                    None => {
                        return Err(format!(
                            "Static semantic error on line {}: variable {} is not defined",
                            tree[i + 1].line, name
                        ));
                    }
                }
                i += 2;
            }
            "#IHAZ" => {
                let name = tree[i + 1].token.clone();
                let mut value = String::new();
                let mut j = i + 3;
                while tree[j].token != "#MKAY" {
                    if !value.is_empty() {
                        value.push(' ');
                    }
                    value.push_str(&tree[j].token);
                    j += 1;
                }
                let last = scopes.len() - 1;
                scopes[last].insert(name, value);
                i = j;
            }
            _ => {
                html.push_str(token);
                html.push(' ');
            }
        }

        i += 1;
    }

    Ok(html)
}
