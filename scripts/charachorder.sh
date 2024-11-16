#!/usr/bin/env bash
set -euo pipefail
BIN="${1:-optimize_sa}"
echo $BIN
cargo build --release --bin $BIN

LAYOUT="jylgonrehtcmak,sq.ud;fvi'bxzwp"

CMD="./target/release/$BIN -l config/keyboard/charachorder.yml"

if [[ "$BIN" == "optimize_sa" || "$BIN" == "optimize_genetic" || "$BIN" == "evaluate" ]]; then
  CMD+=" -e config/evaluation/charachorder.yml -n ngrams/oxey_english"
fi

if [[ "$BIN" == "optimize_sa" || "$BIN" == "optimize_genetic" ]]; then
  CMD+=" -s"
fi

CMD+=" $LAYOUT"
export RUST_LOG=INFO
$CMD
