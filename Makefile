.PHONY: all
all: vm dis ack rvm rack rvault

vm: vm.cpp
	g++ -O2 -o vm vm.cpp -Wall -std=c++17
dis: dis.cpp
	g++ -O2 -o dis dis.cpp -Wall -std=c++17
ack: ack.cpp
	g++ -O2 -o ack ack.cpp -Wall -std=c++17
RUST=rustc
RUST_FLAGS=--edition 2018 -O
rvm: rvm.rs
	$(RUST) $(RUST_FLAGS) rvm.rs
rack: rack.rs
	$(RUST) $(RUST_FLAGS) rack.rs
rvault: rvault.rs
	$(RUST) $(RUST_FLAGS) rvault.rs
