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
    query: String,
    fragment: String,
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
            query: String::new(),
            fragment: fragment.to_owned(),
            params,
        }
    }

    /// Add another [Sql] object to the end of this object.
    /// 
    /// The string fragment of the ``sql`` object is concatenated to this one
    /// and the parameters from the ``sql`` are also added to the end of the
    /// parameter list of this object.
    /// 
    /// # Returns
    /// The current instance of [Sql] with the added parts.
    pub fn append(mut self, sql: Sql<'a>) -> Sql<'a> {
        let mut params = sql.params().to_vec();
        self.fragment.push(' ');
        self.fragment += &sql.fragment;
        self.params.append(&mut params);

        self
    }

    /// The string fragment.
    /// 
    /// # Retruns
    /// The string fragment stored in the [Sql] object.
    pub fn query(&self) -> String {
        self.fragment.clone()
    }

    /// The list of parameters.
    /// 
    /// # Returns
    /// A vector containing the parameters to be inserted into the query.
    pub fn params(&self) -> Vec<&'a dyn Any> {
        self.params.clone()
    }

    /// The final formatted query
    /// 
    /// # Returns
    /// A string containing the formatted query to be passed into a query engine.
    pub fn formatted(&self) -> String {
        self.query.clone()
    }

    /// Finalize the query.
    /// 
    /// The query parameters in the query string will be numbered. Each ``?`` in
    /// the query string will be changed into a ``$`` followed by a number denoting
    /// the position of the parameter to be substituted (beginning with 1, not 0).
    pub fn finalize(mut self) -> Sql<'a> {
        let mut idx = 1;
        let mut query = String::new();
        for ch in self.fragment.chars() {
            if ch == '?' {
                query += &format!("${idx}");
                idx += 1;
            } else {
                query.push(ch);
            }
        }
        self.query = query;

        self
    }

    pub fn identifier(arg: & str) -> String {
        format!(r#""{arg}""#)
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
        let name = Sql::identifier("foo");
        let sql = Sql::new("UPDATE TABLE", None)
            .append(Sql::new(&name, None))
            .append(Sql::new("SET name=?, one=?", Some(&mut [&name, &one])));

        assert_eq!(
            "UPDATE TABLE \"foo\" SET name=?, one=?",
            sql.query(),
            "Sql object statement() returns the fragment part"
        );
        let params = sql.params();
        assert_eq!(
            *params[0].downcast_ref::<String>().unwrap(),
            r#""foo""#,
            "The first parameter is &str"
        );
        assert_eq!(
            *params[1].downcast_ref::<i32>().unwrap(),
            1,
            "The second parameter is an int"
        );
        assert_eq!(
            "UPDATE TABLE \"foo\" SET name=$1, one=$2",
            sql.finalize().formatted(),
            "Sql object statement() returns the fragment part"
        );
    }
}
