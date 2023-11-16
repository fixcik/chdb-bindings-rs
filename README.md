# Rust bindings for libchdb

See: https://github.com/metrico/libchdb

Required to install `libchdb`:

On ubuntu:
```bash
wget -qO - https://metrico.github.io/metrico.gpg | apt-key add - 
echo "deb [arch=x86_64] https://metrico.github.io/deb stable main" | tee /etc/apt/sources.list.d/metrico.list
apt update && apt install -y libchdb
```