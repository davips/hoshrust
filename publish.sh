sudo systemctl start docker.socket
docker run --rm -v $(pwd):/io konstin2/maturin build --release; twine upload target/wheels/*
sudo systemctl stop docker.socket
