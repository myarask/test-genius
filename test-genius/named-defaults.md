To change the find_exports function to accept a source file path instead of a code string, you can modify the function as follows:

1. Import the necessary modules:

```
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use swc_common::{errors::Handler, input::StringInput, FileName, SourceFile};
use swc_ecma_ast::*;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
```

2. Modify the find_exports function to take a source file path as an argument:

```
struct ExportInfo {
    source: String,
    named_exports: Vec<String>,
    default_export: Option<String>,
}

fn find_exports(file_path: &str) -> Vec<ExportInfo> {
    let source_file = Arc::new(SourceFile::new(FileName::Real(file_path.into()), "".into()));
    let mut file = File::open(&Path::new(file_path)).expect("Unable to open the file");
    let mut code = String::new();
    file.read_to_string(&mut code).expect("Unable to read the file");

    let input = StringInput::new(&code, source_file);

    let handler = Handler::with_tty_emitter(swc_common::errors::ColorConfig::Auto, true, false, Some(source_file.clone()));
    let lexer = Lexer::new(Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }), Default::default(), input, None);

    let mut parser = Parser::new_from(lexer);
    let program = parser.parse_program().expect("Failed to parse TypeScript code");

    let mut exports = Vec::new();

    for stmt in &program.body {
        match stmt {
            ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(export_named)) => {
                let mut named_exports = Vec::new();
                for specifier in &export_named.specifiers {
                    if let ExportSpecifier::Named(named_specifier) = specifier {
                        named_exports.push(named_specifier.local.sym.to_string());
                    }
                }
                exports.push(ExportInfo {
                    source: "".to_string(),
                    named_exports,
                    default_export: None,
                });
            }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(export_default)) => {
                exports.push(ExportInfo {
                    source: "".to_string(),
                    named_exports: Vec::new(),
                    default_export: Some("default".to_string()),
                });
            }
            _ => (),
        }
    }

    exports
}
```

Now the find_exports function accepts a file path instead of a code string. When you call the function, provide the path to your TypeScript file as an argument, and the function will read the file content, parse it, and return a list of ExportInfo objects containing the named and default exports.

Here's an example of how to use the find_exports function:

```
fn main() {
    let file_path = "path/to/your/typescript/file.ts";
    let exports = find_exports(file_path);
    
    for export_info in exports {
        println!("Source: {}", export_info.source);
        
        if let Some(default_export) = &export_info.default_export {
            println!("Default export: {}", default_export);
        }

        if !export_info.named_exports.is_empty() {
            println!("Named exports: {:?}", export_info.named_exports);
        }
    }
}

```

Replace "path/to/your/typescript/file.ts" with the path to the TypeScript file you want to analyze. The program will then print the named and default exports found in the file.