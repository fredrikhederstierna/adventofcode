
import java.util.StringTokenizer;
import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.util.stream.Collectors;

import scala.util.control.Breaks._

class AocApp2 extends App :
  println("Run...")

  // stdin
  val input = getInput()
  //println(input)

  var sum : Long = 0

  var tokens = new StringTokenizer(input, "\n", false);
  while tokens.hasMoreTokens() do
      var token = tokens.nextToken().strip()
      var slen : Integer = token.length()
      println("token: " + token)


      
      //--- make it general for lengths, outer while - loop
      val DIGITS : Integer = 12
      var DIGITS_LEFT : Integer = DIGITS
      var DIGITS_COUNT : Integer = 0
      var BEST_IDX : Array[Integer] = new Array[Integer](DIGITS)
      var BEST_VAL : Array[Integer] = new Array[Integer](DIGITS)
      // set all best as first 0..DIGITS index
      for (di <- 0 to (DIGITS - 1))
          BEST_IDX(di) = di
          BEST_VAL(di) = 0 //Integer.parseInt(String.valueOf(token.charAt(di)))
          //println("di=" + di + " BEST " + BEST_IDX(di) + " char " + BEST_VAL(di))

      var first_index : Integer = 0

      while (DIGITS_LEFT > 0)

          // find highest, cannot be last
          val last_index  : Integer = (slen - DIGITS_LEFT)
          for (ci <- fromTo(first_index, last_index))
              var c : Character = token.charAt(ci)
              var v : Integer = Integer.parseInt(String.valueOf(c))
              if (v > BEST_VAL(DIGITS_COUNT)) then
                  BEST_VAL(DIGITS_COUNT) = v
                  BEST_IDX(DIGITS_COUNT) = ci
              end if

          first_index = BEST_IDX(DIGITS_COUNT) + 1
          DIGITS_COUNT = DIGITS_COUNT + 1
          DIGITS_LEFT = DIGITS_LEFT - 1
      end while

      var all_val : Long = 0
      for (di <- 0 to (DIGITS - 1))
          all_val = all_val * 10
          all_val = all_val + BEST_VAL(di)
          println("BEST[" + di + "] IDX " + BEST_IDX(di) + " VAL " + BEST_VAL(di) + " all " + all_val)
      println("all val " + all_val)
      sum = sum + all_val

  end while
  println("sum = " + sum)

  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines

  def fromTo(from : Int, to : Int) : Stream[Int] = 
    if(from > to) {
      Stream.empty
    } else {
      // println("one more.") // uncomment to see when it is called
      Stream.cons(from, fromTo(from + 1, to))
    }

end AocApp2


@main def Main2: Unit =
  println("AOC2: Day 3")
  val app = new AocApp2()
