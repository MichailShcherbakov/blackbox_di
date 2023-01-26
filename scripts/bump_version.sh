#! /usr/bin/env bash

set -e

#
# Bumps the version number to ${1} (x.x.x).
#

if [ -z "${1}" ] ; then
  echo "Usage: $0 <new-version>"
  echo "Example: $0 0.1.1"
  exit 1
fi  

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

PROJECT_ROOT=$(realpath "${SCRIPT_DIR}/../") || exit $?
LIB_ROOT=$(realpath "${PROJECT_ROOT}/packages/lib") || exit $?

CURRENT_VERSION=$(git grep -h "^version" "${LIB_ROOT}" | head -n 1 | cut -d '"' -f2)
NEW_VERSION="${1}"

function dupm_version() {
  find "${PROJECT_ROOT}" -type f -name "*.toml" -exec sed -i "s/${CURRENT_VERSION}/${NEW_VERSION}/g" {} \;
} 

function check_versions_match() {
  local -a PROJECT_PACKAGES=$(echo "("; find "${PROJECT_ROOT}" -type f -name "*.toml" -printf '"%p" '; echo ")")
  
  local last_version=""

  for dir in "${PROJECT_PACKAGES[@]}"; do
    local cargo_toml="${dir}"

    if ! [ -f "${cargo_toml}" ]; then
      echo "Cargo configuration file '${cargo_toml}' does not exist."
      exit 1
    fi

    local version=$(grep version "${cargo_toml}" | head -n 1 | cut -d' ' -f3)
    
    if [ -z "${last_version}" ]; then
      last_version="${version}"
    elif ! [ "${version}" = "${last_version}" ]; then
      echo "Versions differ in '${cargo_toml}'. ${version} != ${last_version}"
      exit 1
    fi
  done
}

echo ":::: BUMP VERSION '${CURRENT_VERSION}' to '${NEW_VERSION}'..."

dupm_version
check_versions_match