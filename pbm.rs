use std::fs;

#[derive(Debug)]
struct Metadata<'b> {
    format: &'b str,
    rows: usize,
    cols: usize,
    data: String,
}

impl<'b> Metadata<'b> {
    fn new(data: String) -> Self {
        let trimmed_data = move|data: String| -> String {
            let mut trimmed = String::new();
            let mut data_lines = data.lines();

            while let Some(line) = data_lines.next() {
                trimmed.push_str(line.trim());
                trimmed.push_str("\n");
            }

            return trimmed;
        };

        Self {
            format: "P1",
            rows: img_size(&data).1,
            cols: img_size(&data).0,
            data: trimmed_data(data),
        }
    }
    fn write_file(&mut self, file_name: &str) {
        let resolution = format!("{} {}", self.rows, self.cols);
        let mut bytes = String::from(self.format);

        bytes.push_str(&format!("\n{}", resolution));
        bytes.push_str(&format!("\n{}", self.data));

        let _ = fs::write(file_name, bytes);
    }
}

fn img_size(data: &str) -> (usize, usize) {
    let mut lines = data.lines();
    let mut col_count = 0;
    let row_count = lines.clone().count();

    if let Some(line) = lines.next() {
        for c in line.trim().chars() {
            if !c.is_whitespace() {
                col_count += 1;
            }
        }
    }

    (row_count, col_count) 
}

fn main() {
    let data = r#"0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
                  0 1 0 0 1 0 1 1 1 1 0 1 0 0 0 0 1 0 0 0 0 1 1 1 1 0 0 0 0 1 0 0 0 1 0 1 1 1 1 0 1 1 1 1 0 1 0 0 0 0 1 1 1 0 0
                  0 1 0 0 1 0 1 0 0 0 0 1 0 0 0 0 1 0 0 0 0 1 0 0 1 0 0 0 0 1 0 0 0 1 0 1 0 0 1 0 1 0 0 1 0 1 0 0 0 0 1 0 0 1 0
                  0 1 1 1 1 0 1 1 1 1 0 1 0 0 0 0 1 0 0 0 0 1 0 0 1 0 0 0 0 1 0 1 0 1 0 1 0 0 1 0 1 1 1 1 0 1 0 0 0 0 1 0 0 1 0
                  0 1 0 0 1 0 1 0 0 0 0 1 0 0 0 0 1 0 0 0 0 1 0 0 1 0 0 0 0 1 1 0 1 1 0 1 0 0 1 0 1 0 1 0 0 1 0 0 0 0 1 0 0 1 0
                  0 1 0 0 1 0 1 1 1 1 0 1 1 1 1 0 1 1 1 1 0 1 1 1 1 0 0 0 0 1 0 0 0 1 0 1 1 1 1 0 1 0 0 1 0 1 1 1 1 0 1 1 1 0 0
                  0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"#;

    let mut pbm = Metadata::new(data.to_owned());

    pbm.write_file("portable_map.pbm");
}
