touch src/ffi.rs && \
rm src/ffi.rs && \
git restore src/* && \
cargo extern-fn -vvv \
        -i board.rs \
        -i build.rs \
        -i fen.rs \
        -i magics.rs \
        -i movelist.rs \
        -i position.rs \
        -i san.rs \
        -i setup.rs \
        -i uci.rs \
        -i util.rs \
        -i variant.rs \
        -i zobrist.rs && \
python3 t.py && \
cargo check