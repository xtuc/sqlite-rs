#[derive(PartialEq)]
enum State {
    Normal,
    AtSeperator,
    InBegin,
}

pub fn split_statements(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut state = State::Normal;

    let mut buffer = "".to_owned();

    for b in input.chars() {
        // Cleanup a bit and remove unnecessary whitespace after the seperator.
        if state == State::AtSeperator {
            if b == ' ' || b == '\n' {
                continue;
            }

            state = State::Normal;
            buffer.push(b);

            continue;
        }

        if state == State::Normal {
            buffer.push(b);

            // Check if we didn't encourter a keywords
            if b == ' ' {
                if buffer.ends_with("BEGIN ") {
                    state = State::InBegin;
                }
            }

            if b == ';' {
                state = State::AtSeperator;
                out.push(buffer.clone());
                buffer.clear();
            }

            continue;
        }

        if state == State::InBegin {
            buffer.push(b);

            if b == ';' && buffer.ends_with("END;") {
                state = State::AtSeperator;
                out.push(buffer.clone());
                buffer.clear();
            }

            continue;
        }
    }

    // Flush what the buffer contained before the EOF
    if !buffer.is_empty() {
        out.push(buffer);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_split_statements() {
        let stmts = split_statements("SELECT 1; SELECT 2");
        assert_eq!(stmts, vec!["SELECT 1;", "SELECT 2"]);
    }

    #[test]
    fn it_keep_statement_with_begin() {
        let stmts =
            split_statements("CREATE TRIGGER trigger AFTER INSERT ON t BEGIN SELECT 1; END;");
        assert_eq!(
            stmts,
            vec!["CREATE TRIGGER trigger AFTER INSERT ON t BEGIN SELECT 1; END;"]
        );

        let stmts = split_statements(
            "CREATE TRIGGER trigger AFTER INSERT ON t BEGIN SELECT 1; END; SELECT 1",
        );
        assert_eq!(
            stmts,
            vec![
                "CREATE TRIGGER trigger AFTER INSERT ON t BEGIN SELECT 1; END;",
                "SELECT 1"
            ]
        );
    }

    #[test]
    fn it_split_statements_multiline() {
        let stmts = split_statements(
            r#"SELECT 1;
                           SELECT 2;
        "#,
        );
        assert_eq!(stmts, vec!["SELECT 1;", "SELECT 2;"]);
    }
}
