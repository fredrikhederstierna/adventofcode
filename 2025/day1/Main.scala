
import java.util.StringTokenizer;
import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.util.stream.Collectors;

class AocApp extends App :
  println("Run...")

  var zeros = 0
  var dial = 50

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
      dial = (dial + (sign * digits)) % 100
      if dial < 0 then dial = dial + 100
      //println("dial = " + dial)
      if dial == 0 then zeros = zeros + 1
  end while
  println("zeros = " + zeros)

  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 1")
  val app = new AocApp()
