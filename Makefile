RUSTC=rustc

./out/% : ./problems/%.rs
	rustc $< --out-dir=./out -L util

./out/%.stdout : ./out/%
	$< > $@

% : ./out/%.stdout
	diff $< ./solutions/$@.sln

