use clap::Parser;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use syn::{Fields, Item, ItemStruct, Type};
use tera::{Context, Tera};

#[derive(Parser, Debug)]
#[command(name = "CodeGen", about = "Generate Rust service and repo")]
struct Args {
    #[arg()]
    base: String,

    #[arg()]
    module: String,

    #[arg(short, long)]
    struct_file: Option<String>,

    #[arg(short, long, default_value = "../src")]
    output_dir: String,
}

fn parse_struct_fields(content: &str, struct_name: &str) -> Option<Vec<(String, String)>> {
    let syntax = syn::parse_file(content).ok()?;

    for item in syntax.items {
        if let Item::Struct(ItemStruct {
            ident,
            fields: Fields::Named(fields),
            ..
        }) = item
        {
            if ident == struct_name {
                let mut result = Vec::new();
                for field in fields.named {
                    let name = field.ident?.to_string();
                    let ty = match &field.ty {
                        Type::Path(type_path) => quote::quote!(#type_path).to_string(),
                        other => quote::quote!(#other).to_string(),
                    };
                    result.push((name, ty));
                }
                return Some(result);
            }
        }
    }

    None
}

/// Convert snake_case to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// Render a Tera template to an output file
fn render_template(template_path: &str, context: &Context, output_path: &str) {
    let tera = Tera::new("templates/**/*").expect("Failed to load templates");
    let rendered = tera
        .render(template_path, context)
        .expect("Failed to render template");
    fs::create_dir_all(Path::new(output_path).parent().unwrap()).unwrap();
    fs::write(output_path, rendered).unwrap();
    println!("✅ Wrote: {}", output_path);
}

/// Append a module entry to mod.rs if not already present
fn append_to_mod_rs(dir: &Path, mod_entry: &str, mod_override: Option<&str>) {
    let mod_rs_path = dir.join("mod.rs");
    let entry = mod_override.unwrap_or(mod_entry);

    let line_to_append = format!("pub mod {};\n", entry);

    let append = match fs::read_to_string(&mod_rs_path) {
        Ok(contents) => !contents.contains(&line_to_append),
        Err(_) => true, // Create new mod.rs if it doesn't exist
    };

    if append {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(mod_rs_path)
            .expect("Failed to open mod.rs for appending");
        file.write_all(line_to_append.as_bytes())
            .expect("Failed to write to mod.rs");
        println!("📦 Updated mod.rs with: {}", line_to_append);
    }
}

fn main() {
    let args = Args::parse();

    let type_snake = args.base.to_lowercase();
    let module = args.module.to_lowercase();
    let type_name = to_pascal_case(&type_snake);
    let table_name = format!("{}s", &type_snake);
    let module_name = type_snake.clone();

    // Ensure struct_file is provided
    let struct_file_path = match &args.struct_file {
        Some(path) => path,
        None => {
            eprintln!("❌ You must provide --struct-file with path to Rust struct definition.");
            std::process::exit(1);
        }
    };

    // Read and parse the struct file
    let struct_file_content = fs::read_to_string(struct_file_path)
        .unwrap_or_else(|_| panic!("❌ Failed to read struct file at: {}", struct_file_path));

    let type_fields_vec = parse_struct_fields(&struct_file_content, &type_name)
        .unwrap_or_else(|| panic!("❌ Failed to find struct {}", type_name));

    let type_fields: Vec<_> = type_fields_vec
        .into_iter()
        .map(|(field, typ)| {
            let mut map = std::collections::HashMap::new();
            map.insert("field", field);
            map.insert("typ", typ);
            map
        })
        .collect();

    let create_type_fields_vec =
        parse_struct_fields(&struct_file_content, &format!("Create{}", type_name))
            .unwrap_or_else(|| panic!("❌ Failed to find struct Create{}", type_name));
    let create_type_fields: Vec<_> = create_type_fields_vec
        .into_iter()
        .map(|(field, typ)| {
            let mut map = std::collections::HashMap::new();
            map.insert("field", field);
            map.insert("typ", typ);
            map
        })
        .collect();

    // Prepare template context
    let mut context = Context::new();
    context.insert("type_name", &type_name);
    context.insert("type_snake", &type_snake);
    context.insert("table_name", &table_name);
    context.insert("module_name", &module_name);
    context.insert("type_fields", &type_fields);
    context.insert("create_type_fields", &create_type_fields);

    // Define output files
    let files_to_generate = vec![
        (
            "domain_service.rs.tera",
            format!(
                "{}/modules/{}/src/domain/services/{}.rs",
                args.output_dir, module, type_snake
            ),
        ),
        (
            "domain_repository.rs.tera",
            format!(
                "{}/modules/{}/src/domain/repositories/{}.rs",
                args.output_dir, module, type_snake
            ),
        ),
        (
            "domain_error.rs.tera",
            format!(
                "{}/modules/{}/src/domain/errors/{}_errors.rs",
                args.output_dir, module, type_snake
            ),
        ),
        (
            "infrastructure_repository.rs.tera",
            format!(
                "{}/modules/{}/src/infrastructure/repositories/{}.rs",
                args.output_dir, module, type_snake
            ),
        ),
        (
            "services.rs.tera",
            format!(
                "{}/modules/{}/src/infrastructure/services/{}.rs",
                args.output_dir, module, type_snake
            ),
        ),
        (
            "api_controller.rs.tera",
            format!(
                "{}/modules/{}/src/api/controllers/{}_handler.rs",
                args.output_dir, module, type_snake
            ),
        ),
        (
            "api_dto.rs.tera",
            format!(
                "{}/modules/{}/src/api/dto/{}.rs",
                args.output_dir, module, type_snake
            ),
        ),
        (
            "api_routes.rs.tera",
            format!(
                "{}/app/src/routers/{}/{}.rs",
                args.output_dir, module, type_snake
            ),
        ),
        (
            "api_mapper.rs.tera",
            format!(
                "{}/modules/{}/src/api/mappers/{}.rs",
                args.output_dir, module, type_snake
            ),
        ),
    ];

    for (template, output_path) in files_to_generate {
        render_template(template, &context, &output_path);

        let dir_path = Path::new(&output_path).parent().unwrap();

        // Check if it's a handler file
        let is_handler = output_path.contains("api/controllers");
        let is_errors = output_path.contains("domain/errors");
        let mod_override = if is_handler {
            Some(format!("{}_handler", type_snake))
        } else if is_errors {
            Some(format!("{}_errors", type_snake))
        } else {
            None
        };

        append_to_mod_rs(dir_path, &type_snake, mod_override.as_deref());
    }
}
