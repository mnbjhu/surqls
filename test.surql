define table test;
define field data on test type object permissions full;
define field data.first on test type string permissions full;
define field data.second on test type string permissions full;

select data, data.first as first
from test
where data.second skip first;
