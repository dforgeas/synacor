.PHONY: all
all: vm dis ack rvm rack

vm: vm.cpp
	g++ -O2 -o vm vm.cpp -Wall -std=c++17
dis: dis.cpp
	g++ -O2 -o dis dis.cpp -Wall -std=c++17
ack: ack.cpp
	g++ -O2 -o ack ack.cpp -Wall -std=c++17
rvm: rvm.rs
	rustc --edition 2018 -O rvm.rs
rack: rack.rs
	rustc --edition 2018 -O rack.rs
