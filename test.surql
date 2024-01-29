define table test;
define field some on test type option<string>;

define table rel;
define field in on rel type record<test>
define field out on rel type record<test>


return {
  let $first = (create test content {
    some: "First thing"
  });

  let $second = (create test content {
    some: "Second thing"
  });

  create rel content {
    in: $first[0].id,
    out: $second[0].id,
  };
};

return {
  let $first = (create test content {
    some: "First thing"
  });
  return $first.id[0];
};
