use std::{fs, path::Path};

use create_vite_rs::Frameworks;
use fs_extra::dir::{copy, CopyOptions};
use inquire::{
    error::InquireResult,
    required,
    ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
    Select, Text,
};

fn main() -> InquireResult<()> {
    inquire::set_global_render_config(get_render_config());
    let project_name = Text::new("Project Name:")
        .with_validator(required!("This field is required"))
        .with_help_message("e.g. vite-project")
        .with_page_size(5)
        .prompt()?;
    let templates = Select::new("Templates:", get_templates()).prompt()?;
    let sub_templates = Select::new("Sub Templates:", get_sub_templates(&templates)).prompt()?;

    let template_dir = format!("template-{}", sub_templates);
    let source_path = Path::new(&template_dir);
    let temp_dest_path = Path::new("temp_dest");
    let dest_path = Path::new(&project_name);

    println!("source_path==>{:?}", source_path.exists());
    if source_path.exists() {
        // 确保临时目标目录存在
        if !temp_dest_path.exists() {
            fs::create_dir_all(temp_dest_path)
                .expect("Failed to create temporary destination directory");
        }

        let options = CopyOptions::new(); // 使用默认选项
        match copy(source_path, temp_dest_path, &options) {
            Ok(_) => {
                // 重命名临时目标目录为最终目标目录
                match fs::rename(
                    temp_dest_path.join(source_path.file_name().unwrap()),
                    dest_path,
                ) {
                    Ok(_) => println!("Project created successfully: {}", project_name),
                    Err(e) => eprintln!("Error renaming project: {}", e),
                }
            }
            Err(e) => eprintln!("Error copying project: {}", e),
        }
    } else {
        eprintln!("Template directory not found: {}", template_dir);
    }

    println!("Your transaction has been successfully recorded.");
    println!("{}", sub_templates);
    Ok(())
}

/// This could be retrieved from a database, for example.
fn get_templates() -> Vec<String> {
    let root_frameworks = Frameworks::new();
    root_frameworks.get_root_frameworks_names()
}

fn get_sub_templates(input: &str) -> Vec<String> {
    let input = input.to_lowercase();
    let frameworks = Frameworks::new();
    let variants = frameworks.get_variants_by_name(&input);
    match variants {
        Some(variants) => variants.iter().map(|v| v.name.to_string()).collect(),
        None => vec![],
    }
}

fn get_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("$").with_fg(Color::LightRed);
    render_config.highlighted_option_prefix = Styled::new("➠").with_fg(Color::LightYellow);
    render_config.scroll_up_prefix = Styled::new("⇞");
    render_config.scroll_down_prefix = Styled::new("⇟");

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(Color::LightRed));

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightYellow);

    render_config.help_message = StyleSheet::new().with_fg(Color::DarkYellow);

    render_config
}
