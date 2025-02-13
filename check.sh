#cargo check 2>errors.txt
clear
cargo check 2> >(grep --color=always -A 10 -B 10 error)
# RUSTFLAGS=-Awarnings cargo check
#cat errors.txt | grep -A 10 -B 10 error 
