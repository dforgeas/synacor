.PHONY: all
all: vm dis rvm

vm: vm.cpp
	g++ -O2 -o vm vm.cpp -Wall
dis: dis.cpp
	g++ -O2 -o dis dis.cpp -Wall
rvm: rvm.rs
	rustc --edition 2018 -O rvm.rs
