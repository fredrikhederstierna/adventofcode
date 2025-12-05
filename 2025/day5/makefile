#
# To build: make compile
# To run:   make run
# To clean: make clean
#

compile:
	mkdir -p ./build
	scala compile -d ./build Main.scala

run:
	scala run -classpath ./build Main.scala

clean:
	rm -rf ./build/
	rm -rf ./.bsp/
	rm -rf ./.scala-build/

test:
	scala run -classpath ./build Main.scala < test.in

input:
	scala run -classpath ./build Main.scala < input.txt
