#
# To build: make compile
# To run:   make run
# To clean: make clean
#

#  "-Xprint:postInlining",
#  "-Xmax-inlines:100000"

compile:
	mkdir -p ./build
	scala compile -d ./build Main.scala

compile2:
	mkdir -p ./build
	cp *.jar ./build
	scala compile -cp commons-math3-3.6.1.jar -d ./build Main2.scala -Xprint:postInlining -Xmax-inlines:100000

run:
	scala run -classpath ./build Main.scala

run2:
	scala run -classpath ./build:commons-math3-3.6.1.jar Main2.scala --main-class Main

clean:
	rm -rf ./build/
	rm -rf ./.bsp/
	rm -rf ./.scala-build/

test:
	scala run -classpath ./build Main.scala < test.in

test2:
	scala run -classpath ./build:commons-math3-3.6.1.jar Main2.scala --main-class Main < test.in

input:
	scala run -classpath ./build Main.scala < input.txt

input2:
	scala run -classpath ./build:commons-math3-3.6.1.jar Main2.scala --main-class Main < input.txt
