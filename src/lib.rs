pub mod database;
pub mod indexer;

const DATABASE_URL: &str = "DATABASE_URL";
const GITHUB_TOKEN: &str = "GITHUB_TOKEN";

pub struct EnviromentValues {
    pub database_url: String,
    pub github_token: Option<String>,
}

impl EnviromentValues {
    pub fn new(self) -> Self {
        dotenv::dotenv().expect("Отсутствует .env");

        let database_url = dotenv::var(DATABASE_URL).expect("Поле DATABASE_URL не найдено!");
        let github_token = Some(dotenv::var(GITHUB_TOKEN).expect("Поле GITHUB_TOKEN не найдено!"));

        Self {
            database_url,
            github_token,
        }
    }
}
