#! /bin/sh

i=0
while [ "$i" -le 100 ] 
do
	cargo run --release | grep "Sum"
	i=$((i + 1))
done 
