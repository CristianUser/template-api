use serde_json;
use handlebars::Handlebars;

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
