#!/bin/bash
## This file will constructs the DB URL and other required configs ##
# Read the password from the secret file
POSTGRES_PASSWORD=$(cat /run/secrets/db-password)

# Construct the full DATABASE_URL using the environment variables and the password
export DATABASE_URL="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST:$POSTGRES_PORT/$POSTGRES_DB"

# Print the constructed DATABASE_URL (for debugging purposes)
echo "DATABASE_URL: $DATABASE_URL"

# Now execute the original command to start your application
exec "$@"




