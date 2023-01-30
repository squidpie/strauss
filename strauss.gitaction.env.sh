export STRAUSS_ROOT_DIR=$(pwd)
export STRAUSS_BUILD_DIR=${STRAUSS_ROOT_DIR}/target/debug
#export GitVersion_SemVer=$(docker run --rm -v "$(pwd):/repo" gittools/gitversion:5.6.6 /repo | \
#  python3 -c "import sys, json; print(json.load(sys.stdin)['SemVer'])")
export GitVersion_SemVer=$(docker run --rm -v "$(pwd):/repo" gittools/gitversion:5.6.6 /repo)
