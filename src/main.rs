use unreact::prelude::*;

// Where the site is hosted
const URL: &str = "https://www.dressmakermovie.com";

fn main() -> Result<(), Error> {
    let mut app = Unreact::new(Config::default(), is_dev(), URL)?;

    app
        // Index page
        .index("~", object!())?
        // Characters
        .route("characters", "characters", object!())?
        // 404 page
        .not_found("404", object!())?
        // Complete app
        .run()?;

    // Add ssl information to root path (not /public)
    expose_ssl();

    println!("Compiled for production.");
    Ok(())
}

/// Copy all files from `/ssl` to `/build`
fn expose_ssl() {
    for file in std::fs::read_dir("ssl")
        .expect("Failed to read directory 'ssl'")
        .flatten()
    {
        let filename = file.file_name();
        let filename = filename.to_string_lossy().to_string();

        dircpy::copy_dir(format!("ssl/{filename}"), format!("build/{filename}"))
            .expect("Failed to copy file from 'ssl'");
    }
}
