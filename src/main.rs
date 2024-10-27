use fs_extra::file;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use tera::Context;
use tera::Tera;

fn main() {
    let public_dir = Path::new("public");
    if !public_dir.exists() {
        fs::create_dir("public").unwrap();
    }

    if let Err(e) = rsync_static(public_dir) {
        println!("Error: {}", e);
    }

    if let Err(e) = render_pages() {
        println!("Error: {}", e);
    }
}

fn rsync_static(public_dir: &Path) -> Result<(), io::Error> {
    let output = Command::new("rsync")
        .arg("-av")
        .arg("--delete")
        .arg("content/static/")
        .arg(public_dir)
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

fn render_pages() -> Result<(), io::Error> {
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

        let dir = Path::new(&file_path);
        if !dir.parent().unwrap().exists() {
            fs::create_dir_all(dir.parent().unwrap())?;
        }

        file::write_all(file_path, &rendered).unwrap();
    }

    Ok(())
}
