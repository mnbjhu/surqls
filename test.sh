#!/bin/bash

curl -X POST \
	-u "root:root" \
	-H "NS: test" \
	-H "DB: test" \
	-H "Accept: application/json" \
	-d "create test content { data: { first: \"first\", second: \"second\", } };" \
	http://localhost:8000/sql
