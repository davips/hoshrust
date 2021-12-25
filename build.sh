cargo build --release && rm hoshrust/*.so && cd hoshrust && ln -s ../target/release/libhoshrust.so hoshrust.so && cd -
tree -I 'incremental|deps|build|target|oprofile_data|dist|venv'
