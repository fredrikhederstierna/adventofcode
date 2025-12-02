
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
           var valid : Boolean = true
           var offset : Integer = 0

           //println("TO test: " + ix)

           breakable {
               for (stride <- 1 to (slen/2)) yield {
                   if (slen % stride == 0) then
                       offset = 0
                       var mismatch : Boolean = false
                       while (offset < stride) && (! mismatch) do
                           var j : Integer = stride
                           while (j < slen) && (! mismatch) do
                               //println("testing stride " + stride + " s(" + offset + ") != s(" + (offset + j) + ")")
                               if s.charAt(offset) != s.charAt(offset + j) then
                                   //println("no match, continue")
                                   mismatch = true // failed, continue
                               j = j + stride
                           end while
                           offset = offset + 1
                       end while
                       if (! mismatch) then
                           valid = false
                           break
                   end if
               }
           }

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
  
end AocApp2


@main def Main2: Unit =
  println("AOC2: Day 2")
  val app = new AocApp2()
