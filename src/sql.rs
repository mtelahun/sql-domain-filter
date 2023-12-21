use std::any::Any;

/// An object that wraps SQL code with its parameters, like::
///```
/// # use sql_domain_filter::sql::Sql;
///    let query = Sql::new("UPDATE TABLE ? SET bar = ? WHERE", Some(&mut [&"foo", &"baz"]));
///```
/// The query string fragment is a ``?``-format string. Escaped
/// characters (like ``"??"``) are not supported. The parameters
/// will be merged into the query string using the `?` formatting operator.
///
/// The purpose of this object is to prevent SQL injection attacks and
/// make SQL queries safer.
#[derive(Clone)]
pub struct Sql<'a> {
    code: String,
    params: Vec<&'a dyn Any>,
}

impl<'a> Sql<'a> {
    /// Create a new instance of [Sql].
    ///
    /// The [fragment] argument is a ``?``-formatted string containing the SQL query.
    ///
    /// # returns
    /// A new [Sql] object.
    pub fn new(fragment: &'a str, params: Option<&mut [&'a dyn Any]>) -> Sql<'a> {
        let params = match params {
            Some(args) => args.to_vec(),
            None => Vec::new(),
        };

        Self {
            code: fragment.to_owned(),
            params,
        }
    }

    pub fn append(mut self, sql: &'a Sql) -> Sql<'a> {
        let mut params = sql.params().to_vec();
        self.code += &sql.code;
        self.params.append(&mut params);

        self
    }

    pub fn query(&self) -> String {
        self.code.clone()
    }

    pub fn params(&self) -> Vec<&'a dyn Any> {
        self.params.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let sql = Sql::new("", None);

        assert!(
            sql.query().is_empty(),
            "Sql object created with empty fragment is itself empty"
        );
        assert_eq!(
            "",
            sql.query(),
            "Sql object created with empty fragment returns empty string"
        );
        assert_eq!(0, sql.params().len(), "Empty Sql object has no parameters")
    }

    #[test]
    fn naked_string() {
        let sql = Sql::new("1 = 1", None);

        assert_eq!(
            "1 = 1",
            sql.query(),
            "Sql object created with a string fragment returns the fragment"
        );
        assert_eq!(
            0,
            sql.params().len(),
            "Sql object with a text fragment has no parameters"
        )
    }

    #[test]
    fn string_with_one_parameter() {
        let sql = Sql::new("CREATE TABLE %s()", Some(&mut [&"foo"]));

        assert_eq!(
            "CREATE TABLE %s()",
            sql.query(),
            "Sql object statement() returns the fragment part"
        );
        assert_eq!(
            *sql.params().pop().unwrap().downcast_ref::<&str>().unwrap(),
            "foo",
            "The first parameter returned contains 'foo'"
        );
    }

    #[test]
    fn string_and_integer_parameter() {
        let one = 1;
        let sql = Sql::new("UPDATE TABLE %s SET one=%s", Some(&mut [&"foo", &one]));

        assert_eq!(
            "UPDATE TABLE %s SET one=%s",
            sql.query(),
            "Sql object statement() returns the fragment part"
        );
        let params = sql.params();
        assert_eq!(
            *params[0].downcast_ref::<&str>().unwrap(),
            "foo",
            "The first parameter is &str"
        );
        assert_eq!(
            *params[1].downcast_ref::<i32>().unwrap(),
            1,
            "The second parameter is an int"
        );
    }
}
