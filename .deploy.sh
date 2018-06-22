#!/bin/bash
if [[ "$TRAVIS_OS_NAME" == "linux" && "$TRAVIS_PULL_REQUEST" = "false" && "$TRAVIS_BRANCH" == "master" ]]; then
  cp CNAME public
  mv public/en/* public && rm -R public/en
  git clone https://github.com/davisp/ghp-import.git &&
  ./ghp-import/ghp_import.py -n -p -f -m "Documentation upload" -b master -r https://"$GH_TOKEN"@github.com/actix/actix.github.io.git public &&
  echo "Uploaded documentation"
fi
