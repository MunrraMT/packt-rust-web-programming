#!/bin/bash
sudo docker compose up -d
until pg_isready -h localhost -p 5432 -U username
do
  echo "waiting for postgres"
  sleep 1;
done
echo "docker is now running"
sudo docker compose down