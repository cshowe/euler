RUSTC=rustc

libutil : ./util/prime.rs
	rustc --out-dir=./out --crate-type=lib --crate-name=util $<

./out/% : ./problems/%.rs libutil
	rustc $< --out-dir=./out -L ./out

./out/%.stdout : ./out/%
	$< > $@

% : ./out/%.stdout
	diff $< ./solutions/$@.sln

all : p1 p2 p3 p6 p7 p9 p10
