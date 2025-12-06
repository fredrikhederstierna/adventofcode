
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

  var matrix : Array[Array[Long]] = Array.ofDim(1100,1100)
  var operators : Array[Character] = Array.ofDim(1100)
  for (row <- 0 until 1100)
      operators(row) = 'x'
      for (col <- 0 until 1100)
          matrix(row)(col) = 0


  var width  : Int = 0
  var height : Int = 0

  var tokens = new StringTokenizer(input, "\n", false);
  while tokens.hasMoreTokens() do
      var token = tokens.nextToken().strip()
      var slen : Integer = token.length()
      println("token: " + token)

      var columns : Array[String] = token.split("\\s+")

      if (columns(0).charAt(0).isDigit) then
          width = 0
          for (c <- 0 until columns.size)
              println("col(" + c + ") = " + columns(c))
              val v : Long = java.lang.Long.parseLong(columns(c).strip())
              matrix(height)(width) = v
              width = width + 1

      else
          println("Operators : " + columns)
          for (c <- 0 until columns.size)
              operators(c) = columns(c).strip().charAt(0)
      end if

      /*
      val start : Long = java.lang.Long.parseLong(parts(0).strip())
      val end : Long = java.lang.Long.parseLong(parts(1).strip())
      //println("start: " + start + " end: " + end)
      var ix : Long = start
      while ix <= end do
           var s = String.valueOf(ix)
           var slen : Integer = s.length()
           var valid : Boolean = false


      width = 0
      for (c <- 0 until slen)
          matrix(height+1)(width+1) = token.charAt(c)
          matrix2(height+1)(width+1) = token.charAt(c)
          width = width + 1
      */
        
      height = height + 1
  end while

  // sub for operators line
  height = height - 1

  println("width " + width + " height " + height)
  println()

  // make an empty frame around area

  for (r <- 0 until height)
      for (c <- 0 until width)
          print(matrix(r)(c) + " ")
      println()

  println()

  println("Operators")
  for (c <- 0 until width)
      print(operators(c) + " ")

  println()


  
  var sum : Long = 0

  for (c <- 0 until width)
      var op : Character = operators(c)
      var col_sum : Long = matrix(0)(c)
      for (r <- 1 until height)
          if (op == '+')
              col_sum = col_sum + matrix(r)(c)
          if (op == '*')
              col_sum = col_sum * matrix(r)(c)
      sum = sum + col_sum

  println("Sum = " + sum)

  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 6")
  val app = new AocApp()
