use std::path::PathBuf;

pub fn prettify(stacktrace: String) -> String {
    let project_path = "scripts";
    let absolute_project_path = PathBuf::from(&project_path)
        .canonicalize()
        .unwrap()
        .as_path()
        .to_string_lossy()
        .replace("\\\\?\\", "")
        .replace("\\", "/");
    let mut result = String::new();
    let prefix = format!("\t{}", &project_path);
    for line in stacktrace.lines() {
        let new_line = if line.starts_with(&prefix) {
            line.replace(
                &prefix,
                format!("\tfile:///{}", absolute_project_path).as_str(),
            )
            .replace("\\", "/")
        } else {
            line.to_string()
        };
        result.push_str(new_line.as_str());
        result.push_str("\n");
    }
    result
}
