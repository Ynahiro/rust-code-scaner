use rust_code_scaner::{database, indexer::code_parser::*};
use tokio;

#[tokio::main]
async fn main() {
    let pool = database::create_pool().await.unwrap();
    let client = pool.get().await.unwrap();
    let rows = client.query("select * from users", &[]).await.unwrap();
    for row in rows {
        let id: i64 = row.get(0);
        let username: String = row.get(1);
        let role: String = row.get(2);

        println!("ID: {}, username: {}, role: {}", id, username, role);
    }
}
/*
*
*    let mut pyt = PythonParser::new();
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

* */
