all: cohle main

main: main.rs
	rustc main.rs -L bin/.

cohle: src/cohle.rs
	rustc src/cohle.rs --out-dir=bin

clean:
	rm bin/* main
