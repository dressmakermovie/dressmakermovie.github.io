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
    expose_root_files();

    println!("Compiled for production.");
    Ok(())
}

/// Copy all files from `/assets/root/*` to `/build/*`
fn expose_root_files() {
    for file in std::fs::read_dir("assets/root")
        .expect("Failed to read directory 'assets/root'")
        .flatten()
    {
        let filename = file.file_name();
        let filename = filename.to_string_lossy().to_string();

        dircpy::copy_dir(format!("assets/root/{filename}"), format!("build/{filename}"))
            .expect("Failed to copy file from 'assets/root'");
    }
}
