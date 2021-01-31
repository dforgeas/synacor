vm: vm.cpp
	g++ -O3 -mcpu=native -o vm vm.cpp -Wall
dis: dis.cpp
	g++ -O3 -mcpu=native -o dis dis.cpp -Wall
