#!/usr/bin/env bash

set -euo pipefail
set -o xtrace

cd -P -- "$(dirname -- "${BASH_SOURCE[0]}")"


gen_dir=$(mktemp -d -t 'kubernetes-client-generate-XXXXXX')

pushd $gen_dir >/dev/null
echo "Downloading generator to $gen_dir..."
git init >/dev/null
git remote add origin https://github.com/kubernetes-client/gen.git
git fetch --progress --depth=1 origin ea9a2062d504bbc56022ef8cc312a79cae7bcbc2
git checkout -b generate FETCH_HEAD

# NOTE:
# we use 5.3.1 because later versions introduced
# some terrible naming convention where dots are replaced
# with the word "Period".
# see
# https://github.com/OpenAPITools/openapi-generator/issues/14706
# https://github.com/OpenAPITools/openapi-generator/issues/15254

# setting the client version major and minor to the kubernetes version
# major and minor, and keeping the patch version separate in case
# we need to make a change how we generate the client.

cat > settings.sh <<EOF
OPENAPI_GENERATOR_COMMIT=v5.3.1
KUBERNETES_BRANCH=v1.31.3
CLIENT_VERSION=1.31.0
PACKAGE_NAME=kubernetes
USERNAME=kubernetes
EOF
popd >/dev/null

"${gen_dir}/openapi/rust.sh" $(pwd) "${gen_dir}/settings.sh"

rm -rf "${gen_dir}"

# Remove unnecessary files generated by openapi-generator.
rm -rf .travis.yml swagger.json swagger.json.unprocessed git_push.sh

# Do not format generated code.
cat > rustfmt.toml <<EOF
ignore = ["/"]
EOF

cargo build

sed -i '' 's/No description provided.*/Generated by .\/regen.bash/' README.md
