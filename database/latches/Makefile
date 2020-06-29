INCLUDE=-I ./include
CFLAGS=-std=c++14 -Wall -g
EXEC=bin/main
INCLUDE_DIR=./include

src=src
build=build
bin=bin
test=test

objects := $(patsubst $(src)/%.cpp, $(build)/%.o, $(wildcard $(src)/*/*.cpp)) ${build}/main.o

all: build ${EXEC}
.PHONY: all

build:
	mkdir -p ${bin}/
	mkdir -p ${bin}/test
	mkdir -p ${build}/
	mkdir -p ${build}/latch/
	mkdir -p ${build}/test/
.PHONY: build

spin_latch_test: build ${bin}/test/spin_latch_test
.PHONY: spin_latch_test

${EXEC}: ${objects}
	g++ ${CFLAGS} ${objects} -o $@ ${INCLUDE}

# build module
${build}/main.o: ${src}/main.cpp
	g++ ${INCLUDE} -c $< ${CFLAGS} -o $@

${build}/latch/spin_latch.o: ${src}/latch/spin_latch.cpp ${INCLUDE_DIR}/latch/spin_latch.h
	g++ ${INCLUDE} -c $< ${CFLAGS} -o $@

# build test
${bin}/test/spin_latch_test: ${build}/latch/spin_latch.o ${build}/test/spin_latch_test.o
	g++ ${CFLAGS} $^ -o ${bin}/test/spin_latch_test ${INCLUDE}

${build}/test/spin_latch_test.o: ${test}/spin_latch_test.cpp
	g++ ${INCLUDE} -c $< ${CFLAGS} -o $@

clean:
	rm -rf bin/
	rm -rf build/

run:
	./${EXEC}