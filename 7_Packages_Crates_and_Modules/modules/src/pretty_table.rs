
pub fn get_pretty_table(title: Option<&str>, headers: &[String], items: &[Vec<String>]) -> String {
    let mut output = String::new();
    for row in items{
        if headers.len() != row.len() {
            return output
        }
    }

    let mut sizes: Vec<usize> = Vec::new();
    for i in 0..headers.len() {
        let mut max_width = headers[i].len();
        for row in items {
            if row[i].len() > max_width {
                max_width = row[i].len();
            }
        }
        sizes.push(max_width+4);
    }
    let total_size = sizes.iter().sum();

    match title {
        None => output.push_str(&format!("#{:=>width$}#", "", width = total_size)),
        Some(n) => output.push_str(&format!("#{:=^width$}#", n, width=total_size)),
    }
    output.push('\n');

    output.push('|');
    for (i, n) in headers.iter().enumerate() {
        output.push_str(&format!("{:-<width$}", n, width=sizes[i]))
    }
    output.push_str("|\n");

    for row in items {
        output.push('|');
        for (i, item) in row.iter().enumerate() {
            output.push_str(&format!("{:<width$}", item, width=sizes[i]))
        }
        output.push_str("|\n");
    }
    output.push_str(&format!("#{:=>width$}#", "", width = total_size));

    output
}