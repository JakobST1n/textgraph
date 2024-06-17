#!/bin/bash
set -x

tmpdir=$(mktemp -d)

dir="${tmpdir}/textgraph"

mkdir -p ${dir}
rm -r ${dir}
mkdir -p ${dir}
cargo build --release

bindir=${dir}/usr/local/bin
mkdir -p ${bindir}

mandir=${dir}/usr/share/man/man1
mkdir -p ${mandir}

cp -r packaging/DEBIAN ${dir}/DEBIAN
cp target/release/textgraph ${bindir}/textgraph
cp textgraph.1 ${mandir}/textgraph.1

cd ${tmpdir}
dpkg-deb --build textgraph
