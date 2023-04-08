#!/bin/bash
set -efuo pipefail

SCRIPTPATH="$(readlink -f "$0")"
SCRIPT="$(basename "$SCRIPTPATH")"
REPODIR="$(dirname "$(dirname "$SCRIPTPATH")")"
WORKDIR=''

function main
{
  [ $# = 1 ] || fail "Wrong number of arguments. Expected: $SCRIPT <new version>"
  local newver="$1"

  init-workdir
  vrun check-clean-git
  vrun modify-cargo-version "$newver"

  echo 'Self test:'
  vrun cargo run

  vrun git add "$REPODIR"/Cargo.{toml,lock}
  vrun git commit -m "New Release: $newver"
  vrun git tag "v${newver}"
  vrun git log -1
}

function init-workdir
{
  WORKDIR="$(mktemp --tmpdir --directory "${SCRIPT}.working.XXX")"
  trap "rm -rf $WORKDIR" EXIT
}

function check-clean-git
{
  local statfile="$WORKDIR/git-status"
  git status --porcelain | tee "$statfile"
  [ "$(cat "$statfile" | wc -l)" = 0 ] || fail 'git repository is not a clean checkout.'
}

function modify-cargo-version
{
  local newver="$1"

  sed -i \
    "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"$/version = \"$newver\"/" \
    "$REPODIR/Cargo.toml"

  git diff --color=always | cat
}

function vrun
{
  echo -e "\n--- Running: $*"
  "$@"
}

function fail
{
  echo "Failure: $*"
  exit 1
}

main "$@"
