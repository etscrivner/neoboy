.PHONY: all clean doc doc-show test

all: doc test
	@echo "  BUILD"
	@cargo build

clean:
	rm -rf *~
	rm -rf **/*~ target

doc:
	@echo "  DOCS"
	@cargo doc

doc-show: doc
	@echo "  DOC SHOW"
	@xdg-open ./target/doc/gameboy/index.html

test:
	@echo "  TEST"
	@cargo test
