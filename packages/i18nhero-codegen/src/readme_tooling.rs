use anyhow::{Ok, Result};
use regex::RegexBuilder;

pub fn update_readme(key: &str, value: &str) -> Result<()> {
    let readme = std::fs::read_to_string("./README.md")?;

    let update = format!("<!-- START_SECTION:{key} -->\n\n{value}\n\n<!-- END_SECTION:{key} -->");

    let re = RegexBuilder::new(
        format!(r"(<!-- START_SECTION:{key} -->)[^{{}}]*?<!-- END_SECTION:{key} -->").as_str(),
    )
    .multi_line(true)
    .build()?;

    let updated = re.replace(&readme, update);

    std::fs::write("./README.md", updated.to_string())?;

    Ok(())
}
