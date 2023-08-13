use unreact::prelude::*;

// Where the site is hosted
const URL: &str = "https://www.dressmakermovie.com";

fn main() -> Result<(), Error> {
    let mut app = Unreact::new(Config::default(), is_dev(), URL)?;

    app
        // Index page
        .index("~", object! { secret: "Hello!" })?
        // 404 page
        .not_found("404", object! {})?
        // Complete app
        .run()?;

    for file in std::fs::read_dir("ssl")
        .expect("Failed to read directory 'ssl'")
        .flatten()
    {
        let filename = file.file_name();
        let filename = filename.to_string_lossy().to_string();

        dircpy::copy_dir(format!("ssl/{filename}"), format!("build/{filename}"))
            .expect("Failed to copy file from 'ssl'");
    }

    println!("Compiled for production.");
    Ok(())
}
