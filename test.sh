#!/bin/bash

curl -X POST \
	-u "root:root" \
	-H "NS: test" \
	-H "DB: test" \
	-H "Accept: application/json" \
	-d "INFO FOR TABLE test;" \
	http://localhost:8000/sql
