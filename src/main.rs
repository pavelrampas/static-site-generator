use fs_extra::dir;
use std::path::Path;
use tera::Context;
use tera::Tera;

fn main() {
    let public_dir = Path::new("public");
    if !public_dir.exists() {
        std::fs::create_dir("public").unwrap();
    }

    if let Err(e) = copy_static(public_dir) {
        println!("Error: {}", e);
    }

    if let Err(e) = render_pages() {
        println!("Error: {}", e);
    }
}

fn copy_static(public_dir: &Path) -> Result<(), std::io::Error> {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;

    dir::copy("content/static", public_dir, &options).unwrap();

    Ok(())
}

fn render_pages() -> Result<(), std::io::Error> {
    let tera = Tera::new("content/**/*.html").unwrap();

    // Render all pages.
    let templates: Vec<_> = tera.get_template_names().collect();
    for template in templates {
        if !template.contains("pages/") {
            continue;
        }

        println!("Rendering {}", template);

        let rendered = tera.render(template, &Context::new()).unwrap();

        let mut file_path = template.replace("pages/", "public/");
        if !template.contains("index.html") {
            file_path = file_path.replace(".html", "/index.html");
        }

        let dir = std::path::Path::new(&file_path);
        if !dir.parent().unwrap().exists() {
            std::fs::create_dir_all(dir.parent().unwrap())?;
        }

        fs_extra::file::write_all(file_path, &rendered).unwrap();
    }

    Ok(())
}
