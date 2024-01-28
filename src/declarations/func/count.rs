// r#"## `count`
//
// The count function counts the number of times that the function is called. This is useful for returning the total number of rows in a SELECT statement with a `GROUP BY` clause.
//
// ### API Definition
// ```
// count() -> 1
// ```
// If a value is given as the first argument, then this function checks whether a given value is truthy. This is useful for returning the total number of rows, which match a certain condition, in a [`SELECT`](/docs/surrealql/statements/select) statement, with a GROUP BY clause.
//
// ### API Definition
// ```
// count(any) -> number
// ```
//
// If an array is given, this function counts the number of items in the array which are truthy. If, instead, you want to count the total number of items in the given array, then use the [`array::len()`](/docs/surrealql/functions/array#len) function.
//
// ### API Definition
// ```
// count(array) -> number
// ```
// The following example shows this function, and its output, when used in a [`RETURN`](/docs/surrealql/statements/return) statement:
//
// ```
// RETURN count();
//
// 1
// ```
// ```
// RETURN count(true);
//
// 1
// ```
// ```
// RETURN count(10 > 15);
//
// 0
// ```
// ```
// RETURN count([ 1, 2, 3, null, 0, false, (15 > 10), rand::uuid() ]);
//
// 5
// ```
// The following examples show this function being used in a [`SELECT`](/docs/surrealql/statements/select) statement with a GROUP clause:
//
// ```
// SELECT count() FROM [{ age: 33 }, { age: 45 }, { age: 39 }] GROUP ALL;
//
// 3
// ```
// ```
// SELECT count(age > 35) FROM [{ age: 33 }, { age: 45 }, { age: 39 }] GROUP ALL;
//
// 2
// ```
// An advanced example of the count function can be seen below:
//
// ```
// SELECT
// 	country,
// 	count(age > 30) AS total
// FROM [
// 	{ age: 33, country: 'GBR' },
// 	{ age: 45, country: 'GBR' },
// 	{ age: 39, country: 'USA' },
// 	{ age: 29, country: 'GBR' },
// 	{ age: 43, country: 'USA' }
// ]
// GROUP BY country;
// ```
// ```json
// [
// 	{
// 		country: 'GBR',
// 		total: 2
// 	},
// 	{
// 		country: 'USA',
// 		total: 2
// 	}
// ]
// ```"#.to_string(),
//

use std::collections::HashMap;

use crate::declarations::{
    functions::{Function, GenericType},
    type_::Type,
};

pub fn get_count_functions() -> HashMap<String, Function> {
    let mut map: HashMap<String, Function> = HashMap::new();

    map.insert(
        "count".to_string(),
        Function {
            args: vec![],
            return_type: GenericType::Named(Type::Int),
            doc: Some(r#"
            ## Count
            The count function counts the number of times that the function is called. This is useful for returning the total number of rows in a SELECT statement with a `GROUP BY` clause.

            ### Examples
            ```surqls
            SELECT
                count() AS total
            FROM [
                { age: 33 },
                { age: 45 },
                { age: 39 }
            ]
            GROUP ALL;
            ```
            ```json
            [
                {
                    total: 3
                }
            ]
            ```
            "#.to_string()),
        },
    );

    map
}
