#[cfg(test)]
use pretty_assertions::assert_eq as pretty_eq;

#[derive(PartialEq)]
enum State {
    Normal,
    AtSeperator,
    InBegin,
    InStringSQ, // '
    InStringDQ, // "
    InStringBT, // `
    InIdent,
    InComment,
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

        if state == State::InComment {
            if b == '\n' {
                state = State::Normal;
                continue;
            }
        }

        if state == State::InIdent {
            buffer.push(b);

            if b == ']' {
                state = State::Normal;
            }

            continue;
        }

        if state == State::InStringSQ {
            buffer.push(b);

            if b == '\'' {
                state = State::Normal;
            }

            continue;
        }

        if state == State::InStringDQ {
            buffer.push(b);

            if b == '"' {
                state = State::Normal;
            }

            continue;
        }

        if state == State::InStringBT {
            buffer.push(b);

            if b == '`' {
                state = State::Normal;
            }

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
            if b == '\n' {
                if buffer.ends_with("BEGIN\n") {
                    state = State::InBegin;
                }
            }

            if b == '\'' {
                state = State::InStringSQ;
            }
            if b == '"' {
                state = State::InStringDQ;
            }
            if b == '`' {
                state = State::InStringBT;
            }

            if b == '[' {
                state = State::InIdent;
            }

            if b == ';' {
                state = State::AtSeperator;
                out.push(buffer.clone());
                buffer.clear();
            }

            if b == '-' {
                if buffer.ends_with("--") {
                    buffer.pop();
                    buffer.pop();
                    state = State::InComment;
                }
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
        pretty_eq!(stmts, vec!["SELECT 1;", "SELECT 2"]);
    }

    #[test]
    fn it_keep_statement_with_begin() {
        let stmts =
            split_statements("CREATE TRIGGER trigger AFTER INSERT ON t BEGIN SELECT 1; END;");
        pretty_eq!(
            stmts,
            vec!["CREATE TRIGGER trigger AFTER INSERT ON t BEGIN SELECT 1; END;"]
        );

        let stmts = split_statements(
            "CREATE TRIGGER trigger AFTER INSERT ON t BEGIN SELECT 1; END; SELECT 1",
        );
        pretty_eq!(
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
        pretty_eq!(stmts, vec!["SELECT 1;", "SELECT 2;"]);

        let stmts = split_statements(
            r#"CREATE TRIGGER trigger AFTER INSERT ON t
BEGIN
    SELECT 1;
END;
        "#,
        );
        pretty_eq!(
            stmts,
            vec!["CREATE TRIGGER trigger AFTER INSERT ON t\nBEGIN\n    SELECT 1;\nEND;"]
        );
    }

    #[test]
    fn it_against_sql_split() {
        // Taken from https://github.com/jvasile/sql_split/blob/main/src/lib.rs

        pretty_eq!(
            split_statements("CREATE TABLE foo (bar text)"),
            vec!["CREATE TABLE foo (bar text)"],
            "Trailing semi-colon is optional"
        );
        pretty_eq!(
            split_statements("CREATE TABLE foo (bar text);"),
            vec!["CREATE TABLE foo (bar text);"],
            "We preserve the semi-colons"
        );
        pretty_eq!(
            split_statements("CREATE TABLE foo (bar text); INSERT into foo (bar) VALUES ('hi')"),
            vec![
                "CREATE TABLE foo (bar text);",
                "INSERT into foo (bar) VALUES ('hi')"
            ]
        );
        pretty_eq!(
            split_statements("invalid sql; but we don't care because we don't really parse it;"),
            vec![
                "invalid sql;",
                "but we don't care because we don't really parse it;"
            ]
        );
        pretty_eq!(
            split_statements("INSERT INTO foo (bar) VALUES ('semicolon in string: ;')"),
            vec!["INSERT INTO foo (bar) VALUES ('semicolon in string: ;')"]
        );
        pretty_eq!(
            split_statements(
                "INSERT INTO foo (bar) VALUES (\"semicolon in double-quoted string: ;\")"
            ),
            vec!["INSERT INTO foo (bar) VALUES (\"semicolon in double-quoted string: ;\")"]
        );
        pretty_eq!(
            split_statements("INSERT INTO foo (bar) VALUES (`semicolon in backtick string: ;`)"),
            vec!["INSERT INTO foo (bar) VALUES (`semicolon in backtick string: ;`)"]
        );
        pretty_eq!(
            split_statements(
                "INSERT INTO foo (bar) VALUES ('interior quote and semicolon in string: ;''')"
            ),
            vec!["INSERT INTO foo (bar) VALUES ('interior quote and semicolon in string: ;''')"]
        );
        pretty_eq!(split_statements("INSERT INTO foo (bar) VALUES (\"interior quote and semicolon in double-quoted string: ;\"\"\")"), vec!["INSERT INTO foo (bar) VALUES (\"interior quote and semicolon in double-quoted string: ;\"\"\")"]);
        pretty_eq!(split_statements("INSERT INTO foo (bar) VALUES (`interior quote and semicolon in backtick string: ;```)"), vec!["INSERT INTO foo (bar) VALUES (`interior quote and semicolon in backtick string: ;```)"]);
        pretty_eq!(
            split_statements("INSERT INTO foo (bar) VALUES (`semicolon after interior quote ``;`)"),
            vec!["INSERT INTO foo (bar) VALUES (`semicolon after interior quote ``;`)"]
        );
        pretty_eq!(
            split_statements(
                "CREATE TABLE [foo;bar] (bar: text); INSERT into foo (bar) VALUES ('hi')"
            ),
            vec![
                "CREATE TABLE [foo;bar] (bar: text);",
                "INSERT into foo (bar) VALUES ('hi')"
            ]
        ); // brackets are ok for identifiers in sqlite
    }

    #[test]
    fn it_recognizes_strings() {
        pretty_eq!(
            split_statements("INSERT INTO projects VALUES(';;;');"),
            vec!["INSERT INTO projects VALUES(';;;');",]
        );
        pretty_eq!(
            split_statements("INSERT INTO projects VALUES('`;');"),
            vec!["INSERT INTO projects VALUES('`;');",]
        );
        pretty_eq!(
            split_statements("INSERT INTO projects VALUES('\"');"),
            vec!["INSERT INTO projects VALUES('\"');",]
        );
        pretty_eq!(
            split_statements("INSERT INTO projects VALUES(\"'\");"),
            vec!["INSERT INTO projects VALUES(\"'\");",]
        );
    }

    #[test]
    fn it_closing_bracket() {
        pretty_eq!(
            split_statements(
                "UPDATE test SET str = 'update]'; UPDATE test SET str = 'second update';"
            ),
            vec![
                "UPDATE test SET str = 'update]';",
                "UPDATE test SET str = 'second update';"
            ]
        );
    }

    #[test]
    fn test_split_comments() {
        // Taken from https://github.com/jvasile/sql_split/blob/main/src/lib.rs
        pretty_eq!(
            split_statements("SELECT * FROM foo; -- trailing comments are fine"),
            vec!["SELECT * FROM foo;"]
        );
        pretty_eq!(
            split_statements("SELECT * FROM foo -- trailing comments are fine"),
            vec!["SELECT * FROM foo "],
            "Fail trailing -- comment w/ no semicolon"
        );
        pretty_eq!(
            split_statements("SELECT * FROM foo; -- trailing comments are fine\nSELECT 1;"),
            vec!["SELECT * FROM foo;", "SELECT 1;",]
        );
        pretty_eq!(split_statements("SELECT 1; --test"), vec!["SELECT 1;",]);
        pretty_eq!(split_statements("SELECT 1;\n --test"), vec!["SELECT 1;",]);
        pretty_eq!(split_statements("--test\nSELECT 1;"), vec!["SELECT 1;",]);
        pretty_eq!(
            split_statements("SELECT * FROM foo; -- trailing ; comments ; are ; fine"),
            vec!["SELECT * FROM foo;"],
            "trailing -- comment w/ multiple semicolons"
        );
        pretty_eq!(
            split_statements(
                "CREATE TABLE foo (\nbar text -- describe bar\nbaz int -- how many baz\n);"
            ),
            vec!["CREATE TABLE foo (\nbar text baz int );"],
            "multiline statement with --comments interspersed"
        );
        pretty_eq!(
            split_statements("SELECT * FROM foo  WHERE blah blah blah"),
            vec!["SELECT * FROM foo  WHERE blah blah blah"],
            "block comment mid-statement"
        );
        assert!(split_statements("-- Start with a comment;SELECT * FROM foo;").is_empty());
        pretty_eq!(
            split_statements("-- Start with a comment\nSELECT * FROM foo;"),
            vec!["\nSELECT * FROM foo;"],
            "-- comment didn't know where to stop"
        );
    }
}
