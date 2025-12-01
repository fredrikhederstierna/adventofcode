
import java.util.StringTokenizer;
import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.util.stream.Collectors;

class AocApp extends App :
  println("Run...")

  var full_turns = 0
  var zeros = 0
  var dial = 50
  var clicks = 0

  //val input = "L68 L30 R48 L5 R60 L55 L1 L99 R14 L82";
  // stdin
  val input = getInput()
  //println(input)

  var tokens = new StringTokenizer(input, " \r\n\t", false);
  while tokens.hasMoreTokens() do
      var token = tokens.nextToken()
      //println(token)
      var sign = 0
      if token.charAt(0) == 'L' then
        sign = -1
      else
        sign = 1
      end if
      var digits : Integer = Integer.parseInt(token.substring(1).trim())
      //println("i=" + digits)
      // add full turns
      var abs_digits : Integer = Math.abs(digits)
      if abs_digits > 100 then
          full_turns = full_turns + (abs_digits / 100)
          digits = digits % 100
      if digits > 0 then
          var dial_new = (dial + (sign * digits))
          //println("dial_new = " + dial_new)
          if (dial_new < 0) && (dial > 0) then
              clicks = clicks + 1
              //println("neg click")
          if (dial_new > 100) then
              clicks = clicks + 1
              //println("pos click")
          dial = (dial_new % 100)
          if dial < 0 then dial = dial + 100
          //println("dial = " + dial)
      if dial == 0 then zeros = zeros + 1
  end while
  println("zeros = " + zeros)
  println("full_turns = " + full_turns)
  println("clicks = " + clicks)
  println("zeros + full_turns + clicks = " + (zeros + full_turns + clicks))

  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 1")
  val app = new AocApp()
