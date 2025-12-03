
import java.util.StringTokenizer;
import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.util.stream.Collectors;

import scala.util.control.Breaks._

class AocApp extends App :
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

      // first digit at pos 0
      var best_first_idx = 0
      var best_first_val = Integer.parseInt(String.valueOf(token.charAt(0)))
      // find highest, cannot be last
      for (ci <- 0 to (slen - 2))
          var c : Character = token.charAt(ci)
          var v : Integer = Integer.parseInt(String.valueOf(c))
          if (v > best_first_val) then
              best_first_val = v
              best_first_idx = ci
          end if
      println("best 1st dig " + best_first_val + " at " + best_first_idx)

      // second digit at pos X+1
      var best_sec_idx = best_first_idx + 1
      var best_sec_val = Integer.parseInt(String.valueOf(token.charAt(best_first_idx + 1)))
      // find highest, cannot be last
      for (ci <- (best_first_idx + 1) to (slen - 1))
          var c : Character = token.charAt(ci)
          var v : Integer = Integer.parseInt(String.valueOf(c))
          if (v > best_sec_val) then
              best_sec_val = v
              best_sec_idx = ci
          end if
      println("best 2nd dig " + best_sec_val + " at " + best_sec_idx)

      sum = sum + 10*best_first_val + best_sec_val
  end while

  println("sum = " + sum)

  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 3")
  val app = new AocApp()
