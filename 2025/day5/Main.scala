
import java.util.StringTokenizer;
import java.io.BufferedReader;
import java.io.InputStreamReader;

import java.util.stream.Collectors;
//import java.util.Vector

//import java.util.List
//import java.util.ArrayList

//import scala.collection.immutable._

//import scala.collection.convert.ImplicitConversions.seq AsJavaList
//import scala.collection.convert.ImplicitConversionsToJava.seq AsJavaList
//import scala.collection.convert.ImplicitConversions.collection asJava
//import scala.collection.convert.ImplicitConversionsToJava.collection asJava

//import scala.collection.JavaConversions._

//import scala.collection.immutable.Vector
//import scala.collection.immutable.List
import scala.collection.mutable.ListBuffer

import scala.util.control.Breaks._

class NumberRange(istart : Long, iend : Long) :

  var start = istart
  var end = iend

  def getStart() : Long =
    this.start
  def getEnd() : Long =
    this.end
  def setStart(s : Long) : Unit =
    this.start = s
  def setEnd(e : Long) : Unit =
    this.end = e

end NumberRange

class AocApp extends App :
  println("Run...")

  //var ranges : Vector[NumberRange] = new Vector[NumberRange]()
  //var ingred : Vector[Long] = new Vector[Long]()
  var ranges : ListBuffer[NumberRange] = new ListBuffer[NumberRange]()
  var ingred : ListBuffer[Long] = new ListBuffer[Long]()

  // stdin
  val input = getInput()
  //println(input)

  var tokens = new StringTokenizer(input, "\n", false);
  while tokens.hasMoreTokens() do
      var token = tokens.nextToken().strip()
      var slen : Integer = token.length()
      println("token: " + token)

      if (token.indexOf('-') == -1)
          var ing : Long = java.lang.Long.parseLong(token)
          println("* found no minus : " + ing)
          //ingred.add(ing)
          ingred += ing
      else
          var rr : Array[String] = token.split('-')
          println("Range: " + rr(0) + "===>"  + rr(1))
          var newRange : NumberRange = new NumberRange(rr(0).toLong, rr(1).toLong)
          //ranges.add(newRange)
          ranges += newRange

  end while


  // convert java Vector to Array
  //val ingArray : Array[Object] = java.util.Arrays.asList( ingred ).toArray
  //val ingArray : java.util.List[Object] = java.util.Arrays.asList( ingred )
  //ingArray.foreach(println)
  //
  //val rangeArray : Array[Object] = java.util.Arrays.asList( ranges ).toArray
  //rangeArray.foreach(println)

  //println("size ingred=" + ingred.size)
  //println("size array ing=" + ingArray.size)
  //println("size array range=" + rangeArray.size)

//  for (ing <- ingArray)
//      println("ING=" + ing)
//      for (rg <- rangeArray)
//          println("RANGE=" + rg)

  var sum : Int = 0

  for (ing <- ingred)
      var fresh : Boolean = false
      println("ING=" + ing)
      breakable {
          for (rg <- ranges)
              //println("RANGE=" + rg.getStart() + " - " + rg.getEnd())
              if (ing >= rg.getStart()) && (ing <= rg.getEnd()) then
                  fresh = true
                  break
              end if
      }
      println("fresh = " + fresh)
      if fresh then
          sum = sum + 1

  println("sum = " + sum)

  // merge ranges

  println()
  for (rg <- ranges)
      println("UNMERGED_RANGE=" + rg.getStart() + " - " + rg.getEnd())
  println()

  var merged : Boolean = true

  while merged do

      breakable {
          for (ri1 <- 0 until ranges.size)
              var s1 = ranges(ri1).getStart()
              var e1 = ranges(ri1).getEnd()
              println("RANGE1(" + ri1 + ")=" + s1 + " - " + e1)
              for (ri2 <- (ri1 + 1) until ranges.size)
                  var s2 = ranges(ri2).getStart()
                  var e2 = ranges(ri2).getEnd()
                  println("  RANGE2(" + ri2 + ")=" + s2 + " - " + e2)
                  // check if merge
                  if s1 <= s2 && e1 >= e2 then
                      ranges.remove(ri2)
                      merged = true
                      break
                  if s2 <= s1 && e2 >= e1 then
                      ranges.remove(ri1)
                      merged = true
                      break
                  if s1 <= s2 && e1 >= s2 then
                      ranges(ri1).setEnd(e2)
                      ranges.remove(ri2)
                      merged = true
                      break
                  if s1 <= e2 && e1 >= e2 then
                      ranges(ri1).setStart(s2)
                      ranges.remove(ri2)
                      merged = true
                      break
        
          merged = false
      }
  
  end while

  var sum2 : Long = 0

  println()
  for (rg <- ranges)
      println("MERGED_RANGE=" + rg.getStart() + " - " + rg.getEnd())
      var elements : Long = rg.getEnd() - rg.getStart() + 1
      //var addy : Long = ((elements * (rg.getEnd() + rg.getStart())) / 2)
      //println(" ADD " + addy)
      //sum2 = sum2 + addy
      println(" ADD " + elements)
      sum2 = sum2 + elements

  println("SUM2=" + sum2)

  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 5")
  val app = new AocApp()
