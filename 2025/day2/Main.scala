
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

  //var x : Long = java.lang.Long.parseLong("6666660113")

  var tokens = new StringTokenizer(input, ",", false);
  while tokens.hasMoreTokens() do
      var token = tokens.nextToken()
      //println(token)
      var parts : Array[String] = token.split('-')
      val start : Long = java.lang.Long.parseLong(parts(0).strip())
      val end : Long = java.lang.Long.parseLong(parts(1).strip())
      //println("start: " + start + " end: " + end)
      var ix : Long = start
      while ix <= end do
           var s = String.valueOf(ix)
           var slen : Integer = s.length()
           var valid : Boolean = false
           if (slen % 2 == 0) then
                //println("I=" + ix)
                breakable { for (pos <- 0 to (slen/2 - 1))
                    //println("C=" + s.charAt(pos))
                    if s.charAt(pos) != s.charAt(slen/2 + pos) then
                           valid = true
                           break
                }
           else
              valid = true
           if valid == false then
                println(s)
                sum = sum + ix
           ix = ix + 1
      end while
  end while
  println("sum = " + sum)

  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 2")
  val app = new AocApp()
