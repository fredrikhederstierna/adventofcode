
import java.util.StringTokenizer;

import java.io.BufferedReader;
import java.io.InputStreamReader;

import java.util.stream.Collectors;

import scala.collection.mutable.ListBuffer
import scala.collection.mutable.Map

import scala.util.control.Breaks._

/*

0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

 4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2


  Long[64] ? Boolean[64][64] ?

  antal positioner (w - 2) * (h - 2)

  skapa Paket{ vector[8] } alla rotationer + flip  (0..7)

  Exempel
  49x47: 54 57 45 65 59 74

  (49-2)*(47-2) = 2115

  
*/



class Present(name : String) :


end Present


class Region() :

      
end Region


class AocApp extends App :
  println("Run...")
  println()

  
  // stdin
  val input = getInput()
  //println(input)

  var region : Region = new Region()

  var readPresents : Boolean = true

  var tokens = new StringTokenizer(input, "\n", false);
  while tokens.hasMoreTokens() do
  
      if (readPresents) then
          // read 6 packets
          for (n <- Range(0, 6))
              // read index
              var pres_index = tokens.nextToken().strip()
              println("present[index " + pres_index + "] n = " + n)
              // present mask
              var present_mask : Array[Array[Int]] = Array(Array(1, 2, 3), Array(4, 5, 6), Array(7, 8, 9))
              // read 3 rows
              for (r <- Range(0, 3))
                  var row_str = tokens.nextToken().strip()
                  //println("row " + r + " = " + row_str)
                  // handle 3 cols
                  for (c <- Range(0, 3))
                      var ch : Character = row_str( c )
                      if (ch == '#') then
                          present_mask(r)(c) = 1
                      else
                          present_mask(r)(c) = 0
                      end if
                      //println("r " + r + ", c " + c + " = " + ch )
                  end for
              end for

              for (r <- Range(0, 3))
                  for (c <- Range(0, 3))
                      print(" " + present_mask(r)(c))
                  end for
                  println()
              end for

          end for
          readPresents = false

      var token = tokens.nextToken().strip()
      var slen : Integer = token.length()
      println("token: " + token)
      
  end while

  println("Start packing...")
  println()

  println("xyz = ")

  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 12")
  val app = new AocApp()
