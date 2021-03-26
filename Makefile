.PHONY: all
all: vm dis ack rvm rack rvault rruins

vm: vm.cpp
	g++ -O2 -o vm vm.cpp -Wall -std=c++17
dis: dis.cpp
	g++ -O2 -o dis dis.cpp -Wall -std=c++17
ack: ack.cpp
	g++ -O2 -o ack ack.cpp -Wall -std=c++17 -Wl,--stack,$$(( 1 << 24 ))
RUST=rustc
RUST_FLAGS=--edition 2018 -O
rvm: rvm.rs
	$(RUST) $(RUST_FLAGS) $<
rack: rack.rs
	$(RUST) $(RUST_FLAGS) $<
rvault: rvault.rs
	$(RUST) $(RUST_FLAGS) $<
rruins: rruins.rs
	$(RUST) $(RUST_FLAGS) $<
