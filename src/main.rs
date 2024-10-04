use create_vite_rs::Frameworks;
use inquire::{
    error::InquireResult,
    required,
    ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
    Select, Text,
};

fn main() -> InquireResult<()> {
    inquire::set_global_render_config(get_render_config());
    let _project_name = Text::new("Project Name:")
        .with_validator(required!("This field is required"))
        .with_help_message("e.g. vite-project")
        .with_page_size(5)
        .prompt()?;
    let _templates = Select::new("Templates:", get_templates()).prompt()?;
    let _sub_templates = Select::new("Sub Templates:", get_sub_templates(&_templates)).prompt()?;

    println!("Your transaction has been successfully recorded.");
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
    render_config.selected_checkbox = Styled::new("☑").with_fg(Color::LightGreen);
    render_config.scroll_up_prefix = Styled::new("⇞");
    render_config.scroll_down_prefix = Styled::new("⇟");
    render_config.unselected_checkbox = Styled::new("☐");

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(Color::LightRed));

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightYellow);

    render_config.help_message = StyleSheet::new().with_fg(Color::DarkYellow);

    render_config
}
