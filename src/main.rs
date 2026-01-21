mod indexer;
use indexer::code_parser::*;

fn main() {
    let mut pyt = PythonParser::new();
    let res = match pyt.extract_snippets("main.py") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Ошибка создания снипетов {:?}", e);
            Vec::new()
        }
    };

    /* let norm = match pyt.normalize_snippet(&res[4]) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Ошибка нормализации {:?}", e);
            NormalizeSnippet {
                tokens: Vec::new(),
                normalize_code: "".to_string(),
                ast_simplified: "".to_string(),
            }
        }
    }; */

    for snip in res {
        match pyt.normalize_snippet(&snip) {
            Ok(norm) => println!("\n\n Нормализованный снипет: {:#?}", norm.normalize_code),
            Err(e) => {
                eprintln!("Ошибка нормалищации {:?}", e);
                NormalizeSnippet {
                    tokens: Vec::new(),
                    normalize_code: "".to_string(),
                    ast_simplified: "".to_string(),
                };
                continue;
            }
        }
    }

    /* res.iter().for_each(|n| println!("{:#?}", n)); */
}
