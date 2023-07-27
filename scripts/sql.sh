#!/usr/bin/env bash
. ../.env

curl -s -X POST \
  -u "$DB_USER:$DB_PASS" \
  -H "NS: $DB_NS" \
  -H "DB: $DB_NAME" \
  -H "Accept: application/json" \
  -d "$1" \
  http://localhost:9999/sql | jq
