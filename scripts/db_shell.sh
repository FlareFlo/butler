#!/usr/bin/env bash
if [[ -z "$DB_CONTAINER" ]]; then
    echo "Must provide $DB_CONTAINER in environment" 1>&2
    exit 1
fi
podman exec -it $DB_CONTAINER psql -U botuser botdb