
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
  var paths : Array[Array[Long]] = Array.ofDim(200,200)
  for (row <- 0 until 200)
      for (col <- 0 until 200)
          matrix(row)(col) = '.'
          paths(row)(col) = 0

  var width  : Int = 0
  var height : Int = 0

  var tokens = new StringTokenizer(input, "\n", false);
  while tokens.hasMoreTokens() do
      var token = tokens.nextToken().strip()
      var slen : Integer = token.length()
      println("token: " + token)

      width = 0
      for (c <- 0 until slen)
          matrix(height)(width)  = token.charAt(c)
          width = width + 1
      height = height + 1
  end while

  println("width " + width + " height " + height)
  println()

  
  // create the first 'S' beam
  for (c <- 0 until width)
      var op : Character = matrix(0)(c)
      if (op == 'S')
              matrix(1)(c) = '|'
              paths(0)(c) = 1
              paths(1)(c) = 1

  
  var splits : Long = 0

  for (r <- 2 until height by 2)
      for (c <- 0 until width)
          var op0 : Character = matrix(r-1)(c)
          var op1 : Character = matrix(r)(c)
          // check empty space
          if (op0 == '|') && (op1 == '.') then
              // just keep moving beam down
              paths(r)(c)    = paths(r-1)(c)
              paths(r+1)(c)  = paths(r-1)(c)
              matrix(r)(c)   = '|'
              matrix(r+1)(c) = '|'
      for (c <- 0 until width)
          var op0 : Character = matrix(r-1)(c)
          var op1 : Character = matrix(r)(c)
          // check splitter
          if (op0 == '|') && (op1 == '^') then
              if (matrix(r+1)(c-1) == '|' && matrix(r+1)(c+1) == '|') then
                  // both used paths
                  println("used both?")
                  paths(r)(c-1)   = paths(r)(c-1)   + paths(r-1)(c)
                  paths(r+1)(c-1) = paths(r+1)(c-1) + paths(r-1)(c)
                  paths(r)(c+1)   = paths(r)(c+1)   + paths(r-1)(c)
                  paths(r+1)(c+1) = paths(r+1)(c+1) + paths(r-1)(c)
              else if (matrix(r+1)(c-1) == '|') then
                  // one used path
                  println("used left")
                  paths(r)(c-1)   = paths(r)(c-1)   + paths(r-1)(c)
                  paths(r+1)(c-1) = paths(r+1)(c-1) + paths(r-1)(c)
                  paths(r)(c+1)   = paths(r-1)(c)
                  paths(r+1)(c+1) = paths(r-1)(c)
              else if (matrix(r+1)(c+1) == '|') then
                  // one used path
                  println("used right")
                  paths(r)(c-1)   = paths(r-1)(c)
                  paths(r+1)(c-1) = paths(r-1)(c)
                  paths(r)(c+1)   = paths(r)(c+1)   + paths(r-1)(c)
                  paths(r+1)(c+1) = paths(r+1)(c+1) + paths(r-1)(c)
              else if (matrix(r+1)(c-1) == '.' && matrix(r+1)(c+1) == '.') then
                  // used paths
                  println("unused")
                  paths(r)(c-1)   = paths(r-1)(c) // + 1
                  paths(r+1)(c-1) = paths(r-1)(c) // + 1
                  paths(r)(c+1)   = paths(r-1)(c) // + 1
                  paths(r+1)(c+1) = paths(r-1)(c) // + 1
              else
                  printf("WHAT?")
              end if
              // draw splitted 2 beams left/right
              matrix(r)(c-1)   = '|'
              matrix(r)(c+1)   = '|'
              matrix(r+1)(c-1) = '|'
              matrix(r+1)(c+1) = '|'
              splits = splits + 1
          end if
      println("Splits(" + r + ") = " + splits)

  // 1633 is correct part-1
  println("Splits = " + splits)

  for (r <- 0 until height)
      for (c <- 0 until width)
          print(matrix(r)(c))
      println()


  println("PATHS")
  for (r <- 0 until height)
      for (c <- 0 until width)
          print(paths(r)(c) + " ")
      println()

  var sum : Long = 0
  for (c <- 0 until width)
      print(paths(height - 1)(c) + " ")
      sum = sum + paths(height - 1)(c)

  println()
  println("Sum of paths = " + sum)
      
  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 7")
  val app = new AocApp()
