#!/bin/sh

# This script will update the documation and publish it to Github pages.
# Requires ghp-import (see https://github.com/davisp/ghp-import)

cargo doc || exit 1

ghp-import -n target/doc || exit 1

git push --force origin gh-pages || exit 1
