.PHONY: all
all: vm dis ack rvm rack rvault rruins
CXX=/home/david/devel/musl-cross-make/output/bin/m68k-linux-musl-g++
vm: vm.cpp
	$(CXX) -O2 -o vm vm.cpp -Wall -std=c++17
dis: dis.cpp
	$(CXX) -O2 -o dis dis.cpp -Wall -std=c++17
ack: ack.cpp
	$(CXX) -O2 -o ack ack.cpp -Wall -std=c++17 -Wl,--stack,$$(( 1 << 23 ))
RUST=rustc
RUST_FLAGS=--edition 2018 -O -C lto=yes
rvm: rvm.rs
	$(RUST) $(RUST_FLAGS) $<
rack: rack.rs
	$(RUST) $(RUST_FLAGS) $<
rvault: rvault.rs
	$(RUST) $(RUST_FLAGS) $<
rruins: rruins.rs
	$(RUST) $(RUST_FLAGS) $<
rteleport: rteleport.rs
	$(RUST) $(RUST_FLAGS) $<
