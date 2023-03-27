struct ImportInfo {
    source: String,
    named_imports: Vec<String>,
    default_import: Option<String>,
}

fn find_imports(file_path: &str) -> Vec<ImportInfo> {
    let code = fs::read_to_string(file_path).expect("Unable to read the file");

    let import_re = Regex::new(r"import\s+(?:(?:(\w+)\s*,\s*)?\{([^}]+)\}\s+from)?(?:\s+(\w+)\s+from)?\s*['\"]([^'\"]+)['\"]\s*;").unwrap();

    let mut imports = Vec::new();

    for cap in import_re.captures_iter(&code) {
        let default_import = cap.get(1).map(|m| m.as_str().to_string());
        let named_imports = cap.get(2).map(|m| {
        m.as_str()
            .split(',')
            .map(|import| import.trim().to_string())
            .collect::<Vec<String>>()
        }).unwrap_or_else(Vec::new);

        let source = cap[4].to_string();

        imports.push(ImportInfo {
            source,
            named_imports,
            default_import,
        });

    }

    imports
}


fn main() {
    let file_path = "path/to/your/typescript/file.ts";
    let imports = find_imports(file_path);

    for import_info in imports {
        println!("Source: {}", import_info.source);

        if let Some(default_import) = &import_info.default_import {
            println!("Default import: {}", default_import);
        }

        if !import_info.named_imports.is_empty() {
            println!("Named imports: {:?}", import_info.named_imports);
        }
    }
}