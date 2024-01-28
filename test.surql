define table test2;
define field obj on test2 type object;
define field obj.data on test2 type option<array<string>>;
define field float_test on test2 type float;
define field number_test on test2 type number;

let $test = "test";
let $numbers = [1, 10];
return $numbers;

create test2 content {
  obj: {
    data: [$test],
  },
  float_test: 0f,
  number_test: 1dec,
};
