#!/bin/bash
set -x

DIST=fc40
ARCH=x86_64
RPMBUILD=rpmbuild
BUILDROOT=${HOME}/rpmbuild/
SPECFILE=packaging/FEDORA/textgraph.spec
SOURCEDIR=$(pwd)
VERSION=$(awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml)
BUILDROOT_SPECFILE="${BUILDROOT}/SPECS/textgraph.spec"

rpmdev-setuptree

echo "Making tarball"
tarwd=$(dirname "${SOURCEDIR}")
tarfile=$(basename "${SOURCEDIR}")
tar -czf "textgraph-${VERSION}.tar.gz" -C "${tarwd}" --transform "s|${tarfile}|textgraph-${VERSION}|" ${tarfile}
cp "textgraph-${VERSION}.tar.gz" "${BUILDROOT}/SOURCES/textgraph-${VERSION}.tar.gz"

echo "Copy specfile"
cp "${SPECFILE}" "${BUILDROOT_SPECFILE}"


echo "Install dependencies"
sudo dnf builddep "${BUILDROOT_SPECFILE}"

echo "Running spmbuild"
rpmbuild -ba "${BUILDROOT_SPECFILE}" \
    --define "package_version ${VERSION}" \
    --define "package_release 1.${DIST}"


