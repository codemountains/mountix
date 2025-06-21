#!/bin/bash

source ./.env

brew tap mongodb/brew
brew install mongodb-database-tools

echo "Migrating data into MongoDB..."

mongoimport $DATABASE_URL \
  --collection=mountains \
  --db=mountix_db \
  --file="./data/mountix_db-mountains.json"

echo "Creating geospatial index on mountains collection..."

mongosh \
  --username="$MONGO_INITDB_ROOT_USERNAME" \
  --password="$MONGO_INITDB_ROOT_PASSWORD" \
  --authenticationDatabase=admin \
  mountix_db \
  --eval "db.mountains.createIndex({ location: '2dsphere' });"

echo "Completed migration"

brew remove mongodb-database-tools
brew untap mongodb/brew
