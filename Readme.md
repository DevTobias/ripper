## Prepare System

```bash
sudo apt update
sudo apt upgrade
```

## Install Rust and build package

```bash
sudo apt install pkg-config
sudo apt-get install libudev-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

git clone https://github.com/DevTobias/ripper.git
cd ~/ripper/services/autoripper
cargo build --release

mkdir ~/autoripper
scp services/autoripper/config.json devtobias@192.168.178.69:~/autoripper
scp services/autoripper/frontend/.env.local devtobias@192.168.178.69:~/ripper/services/autoripper/frontend

mv -f ../../target/release/ripper ~/autoripper/

sudo apt install nodejs
sudo apt install npm

cd ~/ripper/services/autoripper/frontend/
npm install
npm run build

mkdir ~/autoripper/frontend
mv -f dist ~/autoripper/frontend/dist
cd ~/autoripper


mkdir output
mkdir profiles


sudo mv services/autoripper/deployment/autorip.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl start autorip
sudo systemctl enable autorip
```

## Install MakeMKV

```bash
sudo apt-get install build-essential pkg-config libc6-dev libssl-dev libexpat1-dev libavcodec-dev libgl1-mesa-dev qtbase5-dev zlib1g-dev
wget -c https://www.makemkv.com/download/makemkv-bin-1.17.7.tar.gz -O - | tar -xz
wget -c https://www.makemkv.com/download/makemkv-oss-1.17.7.tar.gz -O - | tar -xz

cd makemkv-oss-1.17.7
./configure
make
sudo make install

cd ../makemkv-bin-1.17.7
make
sudo make install
```

## Install Handbrake

sudo apt-get install handbrake-cli
