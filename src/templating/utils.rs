use serde_json;
use handlebars::Handlebars;

pub fn load_templates_from_directory(directory: &str) -> Handlebars {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true); // Enable strict mode for better error handling

    let template_files = std::fs::read_dir(directory).unwrap_or_else(|_| {
        panic!("Failed to read directory {}", directory)
    });
    for entry in template_files {
        let entry = entry.unwrap_or_else(|_| {
            panic!("Failed to read entry in directory {}", directory)
        });
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let template_id = file_name.trim_end_matches(".hbs");
            let template_content = std::fs::read_to_string(path.to_owned()).unwrap_or_else(|_| {
                panic!("Failed to read template file {}", file_name)
            });
            handlebars.register_template_string(template_id, template_content).unwrap_or_else(|_| {
                panic!("Failed to register template {}", template_id)
            });
        }
    }

    handlebars
}


pub fn render_template_from_dir(template_id: &str, context: &serde_json::Value) -> String {
  let directory = "templates"; // Adjust this to your templates directory
  let template_path = format!("{}/{}.hbs", directory, template_id);

  let srcdir = std::path::PathBuf::from(template_path.to_owned());
  let usable_path = srcdir.to_str().unwrap_or_else(|| {
      panic!("Failed to convert path to string")
  });
  let file_content = std::fs::read_to_string(usable_path).unwrap_or_else(|_| {
      panic!("Failed to read template file {}", template_id)
  });
  let reg = Handlebars::new();

  reg.render_template(&file_content, context)
      .unwrap_or_else(|e| {
          println!("Error: {}", e.reason());
          panic!("Failed to render template {}", template_id)
      })
}
