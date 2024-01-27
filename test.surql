define table test
define field data on test type array<string> permissions full;
define table test2;
define field some on test2 type array<array<string>>
define field optional on test2 type option<string>;
define field optional_array on test2 type array<option<string>>;
define field record on test type record<test2>;
define field duration on test type duration;

create test content {
  data: [
    "Hello",
    "World",
    "World",
  ],
  record: r"test2:thing",
  duration: 1w2d3h4m5s,
}
let $some_null = 123;
let $optional_array = [null, "Hello"]

create test2 content {
  some: [
    ["Hello", "World"],
    ["Hello", "World"],
    ["Hello", "World"],
    ["Hello", "World"],
  ],
  optional: "thing",
  optional_array: [
    null,
    "Hello",
    null,
    "World",
  ],
}

let $test = 123
let $test2 = $test
let $my_arr = [1, 2, 3, 4, null]
return $my_arr

let $my_obj = {
  test: "Hello",
  test2: "World",
  more: {
    test: "Hello", test2: "World", 
  } 
}

let $my_array = [1, 2, 3, 4, 5]

create test2 content {
  some: [
    ["Hello", "World"],
    ["Hello", "World"],
    ["Hello", "World"],
    ["Hello", "World"],
  ]
}

let $test = d"2020-04-30T00:00:00.000Z"
let $record = r"test:thing"


let $options = ["one", "two", "three", null]
return $options

let $date_test = ($test == "")

let $duration = 1w2d3h4m5s

let $duration_test = ($duration == 1w2d3h4m5s);

select data, {
  test: data
} from test
let $new = $record


return 10 / 3;
