cargo +nightly build --release --features "divrem" && rm hosh/*.so && cd hosh && ln -s ../target/release/libhosh.so hosh.so && cd -
tree -I 'incremental|deps|build|target|oprofile_data|dist|venv'
