#!/bin/bash

echo "Migrating data into MongoDB..."

mongoimport \
  --username="$MONGO_INITDB_ROOT_USERNAME" \
  --password="$MONGO_INITDB_ROOT_PASSWORD" \
  --authenticationDatabase=admin \
  --db=mountix_db \
  --collection=mountains \
  --file="/docker-entrypoint-initdb.d/mountix_db-mountains.json"

echo "Creating geospatial index on mountains collection..."

mongosh \
  --username="$MONGO_INITDB_ROOT_USERNAME" \
  --password="$MONGO_INITDB_ROOT_PASSWORD" \
  --authenticationDatabase=admin \
  mountix_db \
  --eval "db.mountains.createIndex({ location: '2dsphere' });"

echo "Completed migration"
