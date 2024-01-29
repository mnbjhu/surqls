define table todo;
define field title on todo type string;
define field description on todo type string;
define field tasks on todo type array<string>;

let $created = (create todo content { 
    title: "test", 
    description: "test",
    tasks: ["test1", "test2"],
  });
return $created.title;

let $nested_array_of_obj = [
  [
    { test: "data1"},
    { test: "data2"}
  ],
  [
    { test: "data3"},
    { test: "data4"}
  ]
];

let $test_thing = $nested_array_of_obj.test;

define table item;
define field name on item type string;

define table person;
define field name on person type any;

define table purchase;
define field in on purchase type record<person>;
define field out on purchase type record<item>;

let $person = (create person content { name: "test" });

let $some = (select title, "Hello" as my_string from todo);

return $some[0]