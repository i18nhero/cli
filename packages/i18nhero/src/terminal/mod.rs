use supports_hyperlinks::supports_hyperlinks;

use crate::error::CliError;

#[inline]
pub fn print_error(error: &CliError) {
    eprintln!("{}", console::style(format!("{error}")).red().bold());
}

#[inline]
pub fn print_configuration_file_created() {
    println!(
        "{}",
        console::style("Configuration file has been created!")
            .green()
            .bold()
    );
}

#[inline]
pub fn print_pulling_file(file_name: &str) {
    println!(
        "{}",
        console::style(format!("Pulling {file_name}"))
            .green()
            .bold()
    );
}

#[inline]
pub fn print_no_locales_to_push() {
    println!(
        "{}",
        console::style("Locale directory is empty").yellow().bold()
    );
}

#[inline]
pub fn print_pushed_locales(locale_count: usize) {
    println!(
        "{}",
        console::style(format!(
            "Pushed {locale_count} locale{}",
            if locale_count == 1 { "" } else { "s" }
        ))
        .green()
        .bold()
    );
}

#[inline]
pub fn hyperlink(text: &str, url: &str) -> String {
    if supports_hyperlinks() {
        terminal_link::Link::new(text, url).to_string()
    } else {
        url.to_string()
    }
}

#[inline]
pub fn print_logged_in() {
    println!(
        "{}",
        console::style("You are now signed in to i18nhero!")
            .green()
            .bold()
    );
}

#[inline]
pub fn print_logged_out() {
    println!(
        "{}",
        console::style("You are now signed out of i18nhero!")
            .green()
            .bold()
    );
}

#[inline]
pub fn print_not_authenticated() {
    println!(
        "{}",
        console::style("You are now signed out of i18nhero!")
            .yellow()
            .bold()
    );
}
