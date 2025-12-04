
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

  var matrix : Array[Array[Character]] = Array.ofDim(200,200)
  var matrix2 : Array[Array[Character]] = Array.ofDim(200,200)
  for (row <- 0 until 200)
      for (col <- 0 until 200)
          matrix(row)(col) = '.'
          matrix2(row)(col) = '.'

  var width  : Int = 0
  var height : Int = 0

  var tokens = new StringTokenizer(input, "\n", false);
  while tokens.hasMoreTokens() do
      var token = tokens.nextToken().strip()
      var slen : Integer = token.length()
      println("token: " + token)

      width = 0
      for (c <- 0 until slen)
          matrix(height+1)(width+1) = token.charAt(c)
          matrix2(height+1)(width+1) = token.charAt(c)
          width = width + 1
          
      height = height + 1
  end while

  println("width " + width + " height " + height)
  println()

  var count : Int = 0

  // make an empty frame around area

  for (r <- 1 until height+1)
      for (c <- 1 until width+1)
          print(matrix(r)(c))
          var empty : Int = 0
          if (matrix(r-1)(c-1) == '.') then empty = empty + 1
          if (matrix(r-1)(c)   == '.') then empty = empty + 1
          if (matrix(r-1)(c+1) == '.') then empty = empty + 1

          if (matrix(r)(c-1)   == '.') then empty = empty + 1
          if (matrix(r)(c+1)   == '.') then empty = empty + 1
          
          if (matrix(r+1)(c-1) == '.') then empty = empty + 1
          if (matrix(r+1)(c)   == '.') then empty = empty + 1
          if (matrix(r+1)(c+1) == '.') then empty = empty + 1

          var rolls = 8 - empty
          if (matrix(r)(c) == '@' && (rolls < 4)) then
              matrix2(r)(c) = 'x'
              count = count + 1
      println()

  println()

  for (r <- 1 until height+1)
      for (c <- 1 until width+1)
          print(matrix2(r)(c))
      println()
      
      // 2226 too high
  println("Count = " + count)
  
  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 4")
  val app = new AocApp()
