use std::collections::HashMap;

use sql::Sql;

pub mod sql;

/// Domain operators.
const NOT_OPERATOR: char = '!';
const OR_OPERATOR: char = '|';
const AND_OPERATOR: char = '&';
const DOMAIN_OPERATORS: [char; 3] = [NOT_OPERATOR, OR_OPERATOR, AND_OPERATOR];

/// Term operators.
const TERM_OPERATORS: [&str; 19] = [
    "=",
    "!=",
    "<=",
    "<",
    ">",
    ">=",
    "=?",
    "=like",
    "=ilike",
    "like",
    "not like",
    "ilike",
    "not ilike",
    "in",
    "not in",
    "child_of",
    "parent_of",
    "any",
    "not any",
];

const NEGATIVE_TERM_OPERATORS: [&str; 4] = ["!=", "not like", "not ilike", "not in"];

pub fn sql_operators() -> HashMap<&'static str, Sql<'static>> {
    let map  = HashMap::from([
        ("=", Sql::new("=", None)),
        ("!=", Sql::new("!=", None)),
        ("<=", Sql::new("<=", None)),
        ("<", Sql::new("<", None)),
        (">", Sql::new(">", None)),
        (">=", Sql::new(">=", None)),
        ("in", Sql::new("IN", None)),
        ("not in", Sql::new("NOT IN", None)),
        ("=like", Sql::new("LIKE", None)),
        ("=ilike", Sql::new("ILIKE", None)),
        ("like", Sql::new("LIKE", None)),
        ("ilike", Sql::new("ILIKE", None)),
        ("not like", Sql::new("NOT LIKE", None)),
        ("not ilike", Sql::new("NOT ILIKE", None)),
    ]);

    map
}

pub enum Domain {}
