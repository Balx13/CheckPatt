use std::{
    io::Error,
    fs
};
fn read_themes() -> Result<(), Error> {
    let themes = fs::read_to_string("themes.ron")?;
    

    return Ok(())
}