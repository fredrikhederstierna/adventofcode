
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

  var matrix : Array[Array[Char]] = Array.ofDim(4000,4000)
  var operators : Array[Char] = Array.ofDim(4000)
  for (row <- 0 until 4000)
      operators(row) = '?'
      for (col <- 0 until 4000)
          matrix(row)(col) = ' '


  var width  : Int = 0
  var height : Int = 0

  var maxOperators : Int = 0
  
  var tokens = new StringTokenizer(input, "\n", false);
  while tokens.hasMoreTokens() do
      var token = tokens.nextToken()
      var slen : Integer = token.length()
      println("token: " + token + " len: " + slen)

      var columns : Array[String] = token.strip().split("\\s+")
      var rowChars : Array[Char] = token.toCharArray

      if (columns(0).charAt(0).isDigit) then
          width = 0
          for (c <- 0 until rowChars.size)
              println("col(" + c + ") = " + rowChars(c))
              matrix(height)(width) = rowChars(c)
              width = width + 1

      else
          println("Operators : " + token)
          maxOperators = columns.size
          for (c <- 0 until columns.size)
              operators(c) = columns(c).strip().charAt(0)
      end if

      height = height + 1
  end while

  // sub for operators line
  height = height - 1

  println("width " + width + " height " + height)
  println()

  // make an empty frame around area

  println("Matrix")
  for (r <- 0 until height)
      for (c <- 0 until width)
          print(matrix(r)(c) + ".")
      println()

  println()

  println("Operators")
  for (c <- 0 until width)
      print(operators(c) + " ")

  println()


  var total : Long = 0
  var evalW : Integer = width

  var terms : Array[Long] = Array.ofDim(4000)
  var ti : Integer = 0

  var opIndex : Int = maxOperators - 1

  while (evalW > 0) do
      evalW = evalW - 1


      var colSum : Long = 0
      for (ri <- 0 until height)
          //println("matrix(" + ri + ")(" + evalW + ") = " + matrix(ri)(evalW))
          if (matrix(ri)(evalW).isDigit)
              colSum = colSum * 10
              colSum = colSum + (matrix(ri)(evalW).toInt - '0')

      // special case last one
      if (evalW == 0) then
          println("Last Colsum(ti " + ti + ") = " + colSum)
          terms(ti) = colSum
          ti = ti + 1
      
      if (evalW == 0) || (colSum == 0) then
          println(" === summarize! ti " + ti)
          var op : Char = operators(opIndex)
          println(" === operator! " + op)



          var sum : Long = 0
          sum = terms(0)
          for (i <- 1 until ti)
              if (op == '+')
                  sum = sum + terms(i)
              if (op == '*')
                  sum = sum * terms(i)

          println("Sum = " + sum)

          opIndex = opIndex - 1
          
          total = total + sum

          
          ti = 0
      else
          println("Store Colsum(ti " + ti + ") = " + colSum)
          terms(ti) = colSum
          ti = ti + 1
      end if

      
      if (evalW == 0) then
          println("Last one")
          evalW = evalW - 1 // exit
          
  end while

  println("Total = " + total)

  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 6")
  val app = new AocApp()
