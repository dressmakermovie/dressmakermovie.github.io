use unreact::prelude::*;

// Where the site is hosted
const URL: &str = "https://darccyy.github.io/unreact-template";

fn main() -> Result<(), Error> {
    let mut app = Unreact::new(Config::default(), is_dev(), URL)?;

    app
        // Index page
        .index("homepage", object! { secret: "Hello!" })?
        // 404 page
        .not_found("404", object! {})?
        // Complete app
        .run()?;

    // Only prints if NOT in dev mode
    println!("Compiled for production.");
    Ok(())
}
