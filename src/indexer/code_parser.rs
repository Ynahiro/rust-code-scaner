// Парсер кода для избавления от комментариев
// На вход приходит файл с кодом
// На выходе временный файл с нормализированным кодом


// TODO 
// [ ] Сделать функцию для разделения на токены
// [ ] Сделать представление в виде строки
// [ ] Попробовать сделать поддержку другого языка (хотя бы просто создание сниппетов, чтобы проетстить)

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufReader, Read},
    path::Path,
};

use thiserror::Error;

use tree_sitter::{Language, Node, Parser, Query, QueryCursor, StreamingIterator};

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Faild to remove comments: {0}")]
    RemoveCommentsError(#[source] Box<dyn std::error::Error>),

    #[error("Faild to tokenize code: {0}")]
    TokenizeCodeError(#[source] Box<dyn std::error::Error>),
}

pub enum SourceLanguage {
    Python,
}

#[derive(Debug)]
pub struct RawSnippet {
    pub original_code: String,
    pub start_line: u32,
    pub end_line: u32,
    pub name: Option<String>,
    pub snippet_type: SnippetType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SnippetType {
    // Функциональные блоки
    Function,
    Method, // Метод класса (def внутри class)
    Constructor,
    Destructor,
    Lambda,
    Closure,
    Callback,

    // Классы и структуры
    Class,
    Struct,
    Interface,
    Enum,

    // Блоки логики
    IfBlock,
    ForLoop,
    WhileLoop,
    DoWhileLoop,
    SwitchCase,
    MatchBlock,

    // Обработка ошибок
    TryCatch,
    Finally,
    ErrorHandler,

    // Декларации
    VariableDeclaration,
    ConstantDeclaration,
    TypeAlias,
    ImportBlock,

    // Тело класса/модуля
    ClassBody,
    Module,
    Namespace,

    // Геттеры/сеттеры
    Getter,
    Setter,
    Property,

    // Асинхронность
    AsyncFunction, // Асинхронная функция
    Coroutine,     // Корутина
    Generator,     // Генератор (yield)

    // Шаблоны/дженерики
    TemplateFunction,
    GenericClass,
    GenericMethod,

    // Макросы
    MacroDefinition, // Определение макроса
    MacroCall,       // Вызов макроса

    // Другие
    MainFunction,
    Initializer,
    Deinitializer,
    Unknown,
}

#[derive(Debug)]
pub struct NormalizeSnippet {
    pub tokens: Vec<String>,
    pub normalize_code: String,
    pub ast_simplified: String,
}

pub trait CodeParser: Send + Sync {
    fn new() -> Self;

    // Извлечение фрагментов кода (функции, структуры)
    fn extract_snippets(&mut self, file_path: &str) -> Result<Vec<RawSnippet>, ParserError>;

    // Нормализация (удаление комментариев, замена имен и тд)
    fn normalize_snippet(&mut self, snippet: &RawSnippet) -> Result<NormalizeSnippet, ParserError>;

    // Преобразование дерева в строку для хеширования
    fn get_ast_representation(&self, content: &str) -> Result<String, ParserError>;

    fn collect_declarations(
        &self,
        source: &String,
        node: Node,
        raw_snippets: &mut Vec<RawSnippet>,
    ) {
        let node_type = node.kind();

        let is_declaration = match node_type {
            "function_definition"
            | "lambda"
            | "async_function_def"
            | "if_statement"
            | "for_statement"
            | "while_statement"
            | "try_statement"
            | "with_statement" => true,
            _ => false,
        };

        if is_declaration {
            let original_code = source[node.start_byte()..node.end_byte()].to_string();
            let start_line: u32 = node.start_position().row.try_into().unwrap();
            let end_line: u32 = node.end_position().row.try_into().unwrap();

            let function_name = node
                .child_by_field_name("name")
                .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                .unwrap_or("anonymous")
                .to_string();
            let name = Some(function_name);

            let parent = node.parent().unwrap().kind();
            let snippet_type = SnippetTypeMapper::from_python_node(node_type, Some(parent));

            let raw_snippet = RawSnippet {
                original_code,
                start_line,
                end_line,
                name,
                snippet_type,
            };

            raw_snippets.push(raw_snippet);

            return;
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.collect_declarations(source, child, raw_snippets);
        }
    }
}

pub struct PythonParser {
    parser: Parser,
    language: Language,
}

impl CodeParser for PythonParser {
    fn new() -> Self {
        let language: Language = tree_sitter_python::LANGUAGE.into();
        let mut parser = Parser::new();
        parser
            .set_language(&language)
            .expect("Ошибка создания парсера Python");

        Self { parser, language }
    }

    fn extract_snippets(&mut self, file_path: &str) -> Result<Vec<RawSnippet>, ParserError> {
        let mut raw_snippets: Vec<RawSnippet> = Vec::new();

        let path = Path::new(file_path);
        let mut source = String::new();
        match read_file(&path) {
            Ok(s) => source = s,
            Err(e) => {
                eprintln!("Ошибка чтения файла: {:?}", e);
            }
        }

        let tree = self.parser.parse(&source, None).unwrap();
        let root_node = tree.root_node();

        self.collect_declarations(&source, root_node, &mut raw_snippets);

        Ok(raw_snippets)
    }

    fn normalize_snippet(&mut self, snippet: &RawSnippet) -> Result<NormalizeSnippet, ParserError> {
        let no_com_code = remove_comments(&mut self.parser, &self.language, &snippet.original_code)
            .map_err(|e| ParserError::RemoveCommentsError(Box::new(e)))?;

        let (normalize_code, tokens) = tokenize_code(&mut self.parser, &self.language, no_com_code)
            .map_err(|e| ParserError::TokenizeCodeError(Box::new(e)))?;

        Ok(NormalizeSnippet {
            tokens,
            normalize_code,
            ast_simplified: "".to_string(),
        })
    }

    fn get_ast_representation(&self, content: &str) -> Result<String, ParserError> {
        todo!()
    }
}

pub struct SnippetTypeMapper;

impl SnippetTypeMapper {
    pub fn from_python_node(node_type: &str, parent_type: Option<&str>) -> SnippetType {
        match node_type {
            "function_definition" => {
                if let Some(parent) = parent_type {
                    if parent == "class_definition" {
                        SnippetType::Method
                    } else {
                        SnippetType::Function
                    }
                } else {
                    SnippetType::Function
                }
            }
            "class_definition" => SnippetType::Class,
            "lambda" => SnippetType::Lambda,
            "async_function_def" => SnippetType::AsyncFunction,
            "if_statement" => SnippetType::IfBlock,
            "for_statement" => SnippetType::ForLoop,
            "while_statement" => SnippetType::WhileLoop,
            "try_statement" => SnippetType::TryCatch,
            "with_statement" => SnippetType::ErrorHandler,
            _ => SnippetType::Unknown,
        }
    }

    pub fn from_java_node(node_type: &str) -> SnippetType {
        match node_type {
            "method_declaration" => SnippetType::Method,
            "class_declaration" => SnippetType::Class,
            "interface_declaration" => SnippetType::Interface,
            "enum_declaration" => SnippetType::Enum,
            "constructor_declaration" => SnippetType::Constructor,
            "lambda_expression" => SnippetType::Lambda,
            "if_statement" => SnippetType::IfBlock,
            "for_statement" => SnippetType::ForLoop,
            "while_statement" => SnippetType::WhileLoop,
            "try_statement" => SnippetType::TryCatch,
            "catch_clause" => SnippetType::ErrorHandler,
            _ => SnippetType::Unknown,
        }
    }

    pub fn from_cpp_node(node_type: &str) -> SnippetType {
        match node_type {
            "function_definition" => SnippetType::Function,
            "method_definition" => SnippetType::Method,
            "class_specifier" => SnippetType::Class,
            "struct_specifier" => SnippetType::Struct,
            "lambda_expression" => SnippetType::Lambda,
            "if_statement" => SnippetType::IfBlock,
            "for_statement" => SnippetType::ForLoop,
            "while_statement" => SnippetType::WhileLoop,
            "try_statement" => SnippetType::TryCatch,
            "catch_clause" => SnippetType::ErrorHandler,
            "template_declaration" => SnippetType::TemplateFunction,
            _ => SnippetType::Unknown,
        }
    }
}

fn read_file(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();

    let metadata = fs::metadata(path)?;
    content.reserve(metadata.len() as usize);

    reader.read_to_string(&mut content)?;

    Ok(content)
}

fn remove_comments(
    parser: &mut Parser,
    lang: &Language,
    original_code: &String,
) -> Result<String, ParserError> {
    let tree = parser
        .parse(original_code, None)
        .ok_or_else(|| ParserError::ParseError("Parsing failed".to_string()))?;
    let root_node = tree.root_node();

    // Удаление комментариев
    let query = match lang.name().unwrap() {
        "python" => Query::new(
            lang,
            "(comment) @comment
                    (expression_statement (string) @string)",
        )
        .unwrap(),
        _ => todo!(),
    };

    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&query, root_node, original_code.as_bytes());

    let mut comment_ranges: Vec<(usize, usize)> = Vec::new();

    matches.for_each(|match_| {
        for capture in match_.captures {
            let node = capture.node;
            comment_ranges.push((node.start_byte(), node.end_byte()));
        }
    });

    comment_ranges.sort_by(|a, b| b.0.cmp(&a.0));

    let mut result = original_code.clone();
    for (start, end) in comment_ranges {
        result.replace_range(start..end, "");
    }

    Ok(result)
}

fn tokenize_code(
    parser: &mut Parser,
    lang: &Language,
    code: String,
) -> Result<(String, Vec<String>), ParserError> {
    let tree = parser
        .parse(&code, None)
        .ok_or_else(|| ParserError::ParseError("Parsing error".to_string()))?;
    let root_node = tree.root_node();
    // Замена переменных
    let query_all = Query::new(&lang, "(identifier) @id").unwrap();
    let mut cursor_all = QueryCursor::new();
    let all_identifires = cursor_all.matches(&query_all, root_node, code.as_bytes());

    let query_dec = match lang.name().unwrap() {
        "python" => Query::new(
            &lang,
            "(function_definition name: (identifier) @func_name)
            (class_definition name: (identifier) @class_name)",
        )
        .unwrap(),
        _ => todo!(),
    };

    let mut cursor_dec = QueryCursor::new();
    let dec_identifiers = cursor_dec.matches(&query_dec, root_node, code.as_bytes());

    // Названия классов, функций, перечислений и тд остаются неизменными
    let mut excluded_ranges: Vec<(usize, usize)> = Vec::new();
    let mut functions_identifiers = Vec::new();
    dec_identifiers.for_each(|match_| {
        for capture in match_.captures {
            let node = capture.node;
            excluded_ranges.push((node.start_byte(), node.end_byte()));
            functions_identifiers.push(node);
        }
    });

    let mut identifiers_to_replace = Vec::new();
    all_identifires.for_each(|match_| {
        for capture in match_.captures {
            let node = capture.node;
            let range = (node.start_byte(), node.end_byte());
            if !excluded_ranges.contains(&range) {
                identifiers_to_replace.push(node);
            }
        }
    });

    let normalize_code = replace_token(code, identifiers_to_replace, "identifier");
    let normalize_code = replace_token(normalize_code, functions_identifiers, "function");

    let tokens: Vec<String> = Vec::new();

    Ok((normalize_code, tokens))
}

fn replace_token(mut code: String, mut tokens: Vec<Node<'_>>, token_type: &str) -> String {
    let mut replacement_map = HashMap::new();
    let mut counter = 1;

    tokens.sort_by(|a, b| b.start_byte().cmp(&a.start_byte()));

    let mut replacements: Vec<(usize, usize, String)> = Vec::new();

    for node in tokens {
        let start = node.start_byte();
        let end = node.end_byte();
        let original_node = &code[start..end];
        if !replacement_map.contains_key(original_node) {
            let new_name = match token_type {
                "identifier" => format!("var{}", counter),
                "function" => format!("func{}", counter),
                _ => continue,
            };
            counter += 1;
            replacement_map.insert(original_node.to_string(), new_name);
        }
        let new_name = replacement_map.get(original_node).unwrap().clone();
        replacements.push((start, end, new_name));
    }

    for (start, end, new_name) in replacements {
        code.replace_range(start..end, &new_name);
    }

    code
}
