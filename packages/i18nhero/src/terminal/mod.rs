use crate::error::CliError;

#[inline]
pub fn print_error(error: &CliError) {
    eprintln!("{}", console::style(format!("{error}")).red().bold());
}

#[inline]
pub fn print_configuration_file_created() {
    println!(
        "\n{}",
        console::style("Configuration file has been created!")
            .green()
            .bold()
    );
}
