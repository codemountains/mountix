#!/bin/bash

source ./.env

brew tap mongodb/brew
brew install mongodb-database-tools

mongoimport $DATABASE_URL \
--collection=mountains \
--db=mountix_db \
--file="./data/mountix_db-mountains.json"

brew remove mongodb-database-tools
brew untap mongodb/brew
