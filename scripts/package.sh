#!/bin/bash
SemVer=$(docker run --rm -v "$(pwd):/repo" gittools/gitversion:5.6.6 /repo | \
  python3 -c "import sys, json; print(json.load(sys.stdin)['SemVer'])")

RelName="strauss-${SemVer}"
RelPath="packaging/${RelName}"

mkdir -p ${RelPath}

# Copy required system files
cp system/strauss-compose.yml ${RelPath}/

tar czf ${RelName}.tar.gz -C packaging ${RelName}
mv ${RelName}.tar.gz ${RelPath}/
