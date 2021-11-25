cd backend
cargo run --release &
cd ../frontend/app
npm run serve && kill $!
