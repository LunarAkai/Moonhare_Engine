#! /usr/bin/bash

git add .
git commit -m "$@"
git push -u origin main
git push -u github main

hash=$(git rev-parse --short HEAD)

echo "commit $hash pushed to origin and github"
echo "commit message: $@"