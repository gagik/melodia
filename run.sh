cargo build
export $(cat .env | xargs)
sudo ./target/debug/melodia