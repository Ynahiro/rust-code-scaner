use clap::{self, Parser};
use rust_code_scaner::{
    cli::{Cli, Commands, ConfigArgs, ReportArgs, SourceArgs, UserArgs},
    database,
};
use tokio;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let pool = database::create_pool().await.unwrap();
    let cli = Cli::parse();

    match cli.command {
        Commands::Dashboard(args) => info!("Команда DASHBOARD"),
        Commands::Scan(args) => println!("Команда SCAN"),
        Commands::Report(subcmd) => match subcmd {
            ReportArgs::List => print!("Список отчетов"),
            ReportArgs::Show { id } => print!("Отчет под id"),
            ReportArgs::Export { id, format, output } => print!("Экспорт"),
        },
        Commands::Source(subcmd) => match subcmd {
            SourceArgs::List => print!("Список источников"),
            SourceArgs::Add { name, url_or_path } => print!("Добавление нового источника"),
            SourceArgs::Update { id } => print!("Обновление источника по id"),
            SourceArgs::Remove { id } => print!("Удаление источника по id"),
            SourceArgs::Status => print!("Статус источников"),
        },
        Commands::User(subcmd) => match subcmd {
            UserArgs::List => print!("Список пользователей"),
            UserArgs::Create { name, role } => print!("Создание нового пользователя"),
            UserArgs::Delete { name } => print!("Удаление пользователя"),
            UserArgs::SetRole { name, role } => print!("Присвоение роли пользователю"),
            UserArgs::ResetPassword { name } => print!("Смена пароля"),
        },
        Commands::Config(subcmd) => match subcmd {
            ConfigArgs::Show => print!("Просмотр конфига"),
            ConfigArgs::Set { key, value } => print!("Изменение конфига"),
            ConfigArgs::Reset => print!("Сброс конфига"),
        },
    }

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
