#!/bin/bash

source ./.env

brew tap mongodb/brew
brew install mongodb-database-tools

mongoimport $DATABASE_URL \
--collection=mountains_sh_v2 \
--db=mountix_db \
--file="./data/mountix_db-mountains.json"
