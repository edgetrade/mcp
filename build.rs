use heck::{ToPascalCase, ToSnakeCase};
use std::fs;
use std::path::{Path, PathBuf};
use typify::{TypeSpace, TypeSpaceSettings};
use vergen_gitcl::{Emitter, GitclBuilder};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum RouteType {
    Query,
    Mutation,
    Subscription,
}

/// Schema metadata from JSON files
#[derive(Debug, Clone, serde::Deserialize)]
struct SchemaFile {
    procedure: String,
    name: String,
    #[serde(rename = "type")]
    route_type: RouteType,
    #[serde(rename = "inputSchema")]
    input_schema: serde_json::Value,
    #[serde(rename = "outputSchema")]
    output_schema: serde_json::Value,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Git version info
    let gitcl = GitclBuilder::default()
        .sha(true)
        .describe(true, true, None)
        .build()?;
    Emitter::default().add_instructions(&gitcl)?.emit()?;

    // Generate types from JSON schemas
    generate_types_from_schemas()?;

    Ok(())
}

fn generate_types_from_schemas() -> Result<(), Box<dyn std::error::Error>> {
    // Define source and output directories relative to CARGO_MANIFEST_DIR
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let chaos_routes_dir = manifest_dir.join("../chaos/src/generated/routes");
    let output_base_dir = manifest_dir.join("src/generated/routes");

    // Process both requests and subscriptions
    let categories = ["requests", "subscriptions"];

    for category in &categories {
        let source_dir = chaos_routes_dir.join(category);
        let output_dir = output_base_dir.join(category);

        // Ensure output directory exists
        fs::create_dir_all(&output_dir)?;

        // Collect generated modules
        let mut modules: Vec<String> = Vec::new();

        // Check if source directory exists
        if !source_dir.exists() {
            eprintln!("cargo::warning=Source directory does not exist: {:?}", source_dir);
            continue;
        }

        // Process each JSON file in the source directory
        for entry in fs::read_dir(&source_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }

            // Read and parse the schema file
            let schema_content = fs::read_to_string(&path)?;

            // Skip empty files
            if schema_content.trim().is_empty() {
                eprintln!("cargo::warning=Skipping empty schema file: {:?}", path);
                continue;
            }

            let schema_file: SchemaFile = serde_json::from_str(&schema_content)
                .map_err(|e| format!("Failed to parse schema file {:?}: {}", path, e))?;

            // Convert procedure name to snake_case for filename
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let snake_name = procedure_to_snake_case(file_stem);
            let output_file = output_dir.join(format!("{}.rs", snake_name));

            // Generate Rust code using typify
            let generated_code = generate_rust_types(&schema_file)?;

            // Write the generated file
            fs::write(&output_file, generated_code)?;

            modules.push(snake_name);

            // Tell cargo to rerun if schema changes
            println!("cargo::rerun-if-changed={}", path.display());
        }

        // Generate mod.rs for this category
        generate_mod_file(&output_dir, &modules)?;
    }

    // Generate top-level mod.rs for routes
    generate_routes_mod_file(&output_base_dir)?;

    // Generate top-level mod.rs for generated
    generate_generated_mod_file(&manifest_dir.join("src/generated"))?;

    Ok(())
}

/// Generate Rust types from schema file using typify
fn generate_rust_types(schema_file: &SchemaFile) -> Result<String, Box<dyn std::error::Error>> {
    let settings = TypeSpaceSettings::default();

    // Create TypeSpace with settings
    let mut type_space = TypeSpace::new(&settings);

    // Generate type name base (e.g., "CreateEncryptedWallet" from "create_encrypted_wallet")
    let base_name = schema_file.name.to_pascal_case();

    // Create wrapper schemas with title for proper naming
    let request_name = format!("{}Request", base_name);
    let response_name = format!("{}Response", base_name);

    let wrapped_input = wrap_schema_with_title(&schema_file.input_schema, &request_name)?;
    let wrapped_output = wrap_schema_with_title(&schema_file.output_schema, &response_name)?;

    // Parse schemas as RootSchema which handles complex JSON schema structures
    let input_root: schemars::schema::RootSchema = serde_json::from_value(wrapped_input)?;
    type_space.add_ref_types(input_root.definitions)?;
    type_space.add_type_with_name(
        &schemars::schema::Schema::Object(input_root.schema),
        Some(request_name.clone()),
    )?;

    let output_root: schemars::schema::RootSchema = serde_json::from_value(wrapped_output)?;
    type_space.add_ref_types(output_root.definitions)?;
    type_space.add_type_with_name(
        &schemars::schema::Schema::Object(output_root.schema),
        Some(response_name.clone()),
    )?;

    // Generate the code
    let token_stream = type_space.to_stream();

    // Generate Route struct using quote
    let procedure = &schema_file.procedure;
    let route_type_variant = match schema_file.route_type {
        RouteType::Query => quote::quote! { RouteType::Query },
        RouteType::Mutation => quote::quote! { RouteType::Mutation },
        RouteType::Subscription => quote::quote! { RouteType::Subscription },
    };
    let response_type = proc_macro2::Ident::new(&response_name, proc_macro2::Span::call_site());

    // Check if input schema is empty (no properties) and use () if so
    let has_input = schema_file
        .input_schema
        .get("properties")
        .and_then(|p| p.as_object())
        .map(|p| !p.is_empty())
        .unwrap_or(false);

    let route_impl = if has_input {
        let request_type = proc_macro2::Ident::new(&request_name, proc_macro2::Span::call_site());
        quote::quote! {
            use crate::client::{Route, RouteType};
            use std::marker::PhantomData;

            /// Route metadata for this procedure
            pub const ROUTE: Route<#request_type, #response_type> = Route {
                procedure: #procedure,
                route_type: #route_type_variant,
                input_schema: PhantomData,
                output_schema: PhantomData,
            };
        }
    } else {
        quote::quote! {
            use crate::client::{Route, RouteType};
            use std::marker::PhantomData;

            /// Route metadata for this procedure
            pub const ROUTE: Route<(), #response_type> = Route {
                procedure: #procedure,
                route_type: #route_type_variant,
                input_schema: PhantomData,
                output_schema: PhantomData,
            };
        }
    };

    // Combine typify output with Route impl - typify types first, then ROUTE
    let combined = quote::quote! {
        #route_impl
        #token_stream
    };

    // Format the output using prettyplease
    let parsed = syn::parse2::<syn::File>(combined)?;
    let formatted = prettyplease::unparse(&parsed);

    // Add clippy allow for typify's manual Default impls (necessary for flattened fields)
    let with_allow = format!("#![allow(clippy::derivable_impls)]\n\n{}", formatted);

    Ok(with_allow)
}

/// Wrap a schema with a title to control the generated type name
fn wrap_schema_with_title(
    schema: &serde_json::Value,
    title: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Check if it's already a RootSchema
    let is_root = schema.get("schema").is_some() || schema.get("$schema").is_some();

    if is_root {
        // Already wrapped, just add title
        let mut wrapped = schema.clone();
        if let Some(obj) = wrapped.as_object_mut()
            && let Some(schema_obj) = obj.get_mut("schema").and_then(|s| s.as_object_mut())
        {
            schema_obj.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        }
        return Ok(wrapped);
    }

    // Wrap in RootSchema format
    let wrapped = serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": title,
        "schema": schema
    });

    Ok(wrapped)
}

/// Generate mod.rs file for a category directory
fn generate_mod_file(output_dir: &Path, modules: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mod_path = output_dir.join("mod.rs");
    let mut content = String::from("// Generated by build.rs\n\n");

    // Sort modules for consistent output
    let mut sorted_modules = modules.to_vec();
    sorted_modules.sort();

    for module in sorted_modules {
        content.push_str(&format!("pub mod {};\n", module));
    }

    fs::write(mod_path, content)?;

    Ok(())
}

/// Generate top-level mod.rs for routes
fn generate_routes_mod_file(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mod_path = output_dir.join("mod.rs");
    let content = "// Generated by build.rs\n\npub mod requests;\npub mod subscriptions;\n";
    fs::write(mod_path, content)?;
    Ok(())
}

/// Generate top-level mod.rs for generated
fn generate_generated_mod_file(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mod_path = output_dir.join("mod.rs");
    let content = "// Generated by build.rs\n\npub mod routes;\n";
    fs::write(mod_path, content)?;
    Ok(())
}

/// Convert a procedure name like "agent.createEncryptedWallet" to snake_case "agent_create_encrypted_wallet"
fn procedure_to_snake_case(procedure: &str) -> String {
    procedure.replace('.', "_").to_snake_case()
}
