
import java.util.StringTokenizer;
import java.io.BufferedReader;
import java.io.InputStreamReader;

import java.util.stream.Collectors;

import scala.collection.mutable.ListBuffer

import scala.util.control.Breaks._

class RedTile(xin : Long, yin : Long) :

  var x = xin
  var y = yin

  def getX() : Long =
    this.x
  def getY() : Long =
    this.y
  def setX(xin : Long) : Unit =
    this.x = xin
  def setY(y : Long) : Unit =
    this.y = yin

  def getArea(tile2 : RedTile) : Long =
    var area : Long = (Math.abs(tile2.getX() - this.x) + 1) * (Math.abs(tile2.getY() - this.y) + 1)
    return area

  def insideArea(tile2 : RedTile, testTile : RedTile) : Boolean =
    var leftX  : Long = if (this.x < tile2.getX()) then this.x else tile2.getX()
    var rightX : Long = if (this.x > tile2.getX()) then this.x else tile2.getX()
    var topY   : Long = if (this.y < tile2.getY()) then this.y else tile2.getY()
    var downY  : Long = if (this.y > tile2.getY()) then this.y else tile2.getY()
    var inside : Boolean = (testTile.getX() > leftX) && (testTile.getX() < rightX) && (testTile.getY() > topY) && (testTile.getY() < downY)
    return inside

end RedTile

class AocApp extends App :
  println("Run...")

  var tiles : ListBuffer[RedTile] = new ListBuffer[RedTile]()

  // stdin
  val input = getInput()
  //println(input)

  var tokens = new StringTokenizer(input, "\n", false);
  while tokens.hasMoreTokens() do
      var token = tokens.nextToken().strip()
      var slen : Integer = token.length()
      println("token: " + token)

      var rr : Array[String] = token.split(',')
      println("Tile: " + rr(0) + "===>"  + rr(1))

      var newTile : RedTile = new RedTile(rr(0).toLong, rr(1).toLong)
      tiles += newTile

  end while

  println()
  for (tile <- tiles)
      println("tile= " + tile.getX() + " " + tile.getY())

  var i : Int = 0
  var max : Long = 0;
  while (i < tiles.size) do
      var j : Int = i + 1
      while (j < tiles.size) do
          var area : Long = tiles(i).getArea( tiles(j) )

          // check that no other point is inside area
          var inside : Boolean = false

          /*
          breakable {
              for (tile <- tiles)
                  if (tile != tiles(i) && tile != tiles(j)) then
                      if (tiles(i).insideArea( tiles(j), tile ) ) then
                          inside = true
                          break
          }
          */
          
          if ((area > max) && (! inside)) then
              max = area
              println("new max " + max + " from " + tiles(i).getX() + "," + tiles(i).getY() + " - " + tiles(j).getX() + "," + tiles(j).getY() + ".")
          j = j + 1
      i = i + 1
  end while

  println("Max area = " + max)

  println("END.")


  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 9")
  val app = new AocApp()
