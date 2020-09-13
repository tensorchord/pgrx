// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use pgx::*;
use serde::*;

pg_module_magic!();

#[pg_extern]
fn sum_array(input: Array<i32>) -> i64 {
    let mut sum = 0 as i64;

    for i in input {
        sum += i.unwrap_or(-1) as i64;
    }

    sum
}

#[pg_extern]
fn sum_vec(mut input: Vec<Option<i32>>) -> i64 {
    let mut sum = 0 as i64;

    input.push(Some(6));

    for i in input {
        sum += i.unwrap_or_default() as i64;
    }

    sum
}

#[pg_extern]
fn static_names() -> Vec<Option<&'static str>> {
    vec![Some("Brandy"), Some("Sally"), None, Some("Anchovy")]
}

#[pg_extern]
fn static_names_set() -> impl std::iter::Iterator<Item = Vec<Option<&'static str>>> {
    vec![
        vec![Some("Brandy"), Some("Sally"), None, Some("Anchovy")],
        vec![Some("Eric"), Some("David")],
        vec![Some("ZomboDB"), Some("PostgreSQL"), Some("Elasticsearch")],
    ]
    .into_iter()
}

#[pg_extern]
fn i32_array_no_nulls() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

#[pg_extern]
fn i32_array_with_nulls() -> Vec<Option<i32>> {
    vec![Some(1), None, Some(2), Some(3), None, Some(4), Some(5)]
}

#[pg_extern]
fn strip_nulls(input: Vec<Option<i32>>) -> Vec<i32> {
    input
        .into_iter()
        .filter(|i| i.is_some())
        .map(|i| i.unwrap())
        .collect()
}

#[derive(PostgresType, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SomeStruct {}

#[pg_extern]
fn return_vec_of_customtype() -> Vec<SomeStruct> {
    vec![SomeStruct {}]
}

#[cfg(test)]
pub mod pg_test {

    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}

#[cfg(any(test, feature = "pg_test"))]
pub mod tests {
    use crate::SomeStruct;
    use pgx::*;
    #[pg_test]
    fn test_vec_of_customtype() {
        let customvec =
            Spi::get_one::<Vec<SomeStruct>>("SELECT arrays.return_vec_of_customtype();")
                .expect("SQL select failed");
        assert_eq!(customvec, vec![SomeStruct {}]);
    }
}
