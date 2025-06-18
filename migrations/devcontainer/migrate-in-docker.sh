#!/bin/bash

echo "Migrating data into MongoDB..."

mongoimport \
  --username="$MONGO_INITDB_ROOT_USERNAME" \
  --password="$MONGO_INITDB_ROOT_PASSWORD" \
  --authenticationDatabase=admin \
  --db=mountix_db \
  --collection=mountains \
  --file="/docker-entrypoint-initdb.d/mountix_db-mountains.json"

echo "Completed migration"
