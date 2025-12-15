
import java.util.StringTokenizer;
import java.io.BufferedReader;
import java.io.InputStreamReader;

import java.util.stream.Collectors;

import scala.collection.mutable.ListBuffer

import scala.util.control.Breaks._

class RedTile(xin : Long, yin : Long) :

  var x = xin
  var y = yin

  // polygone stuff
  var visited : Boolean = false
  def getVisited() : Boolean =
    this.visited
  def setVisited(v : Boolean) : Unit =
    this.visited = v
  var prev : RedTile = null
  def getPrev() : RedTile =
    this.prev
  def setPrev(p : RedTile) : Unit =
    this.prev = p
  var next : RedTile = null
  def getNext() : RedTile =
    this.next
  def setNext(n : RedTile) : Unit =
    this.next = n
  
  def getX() : Long =
    this.x
  def getY() : Long =
    this.y
  def setX(xin : Long) : Unit =
    this.x = xin
  def setY(y : Long) : Unit =
    this.y = yin

  // return true if line x1,y1,x2,y2 cross line with tile.x, next.x, tile.y, next.y
  def linesCrossNext(x1 : Long, y1 : Long, x2 : Long, y2 : Long) : Boolean =
    var nextX : Long = next.getX()
    var nextY : Long = next.getY()
    var thisX : Long = this.getX()
    var thisY : Long = this.getY()

    // check not ourselves
    //if (x1 == thisX) && (y1 == thisY) then
    //    return false
    //if (x2 == thisX) && (y2 == thisY) then
    //    return false
    if (x1 == nextX) && (y1 == nextY) then
        return false
    if (x2 == nextX) && (y2 == nextY) then
        return false

    var leftX1  : Long = if (thisX <= nextX) then thisX else nextX
    var rightX1 : Long = if (thisX >= nextX) then thisX else nextX
    var topY1   : Long = if (thisY <= nextY) then thisY else nextY
    var downY1  : Long = if (thisY >= nextY) then thisY else nextY

    var leftX2  : Long = if (x1 <= x2) then x1 else x2
    var rightX2 : Long = if (x1 >= x2) then x1 else x2
    var topY2   : Long = if (y1 <= y2) then y1 else y2
    var downY2  : Long = if (y1 >= y2) then y1 else y2

    if (leftX1 >= rightX2) return false
    if (rightX1 <= leftX2) return false
    if (downY1 <= topY2)   return false
    if (topY1 >= downY2)   return false
    
    return true

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

  // override def compareTo(other: RedTile): Int =
  // compare: sortTopDownLeftRight : --- sortBy(...)
  // compare: sortLeftRightTopDown : --- sortBy(...)

  // if t1 is strictly less than t2 return true, in all other cases return false.
  def sortByXY(t1: RedTile, t2: RedTile) : Boolean =
    var res : Boolean = false
    if (t1.x < t2.x) then res = true
    else if (t1.x == t2.x) && (t1.y < t2.y) then res = true
    end if
    println("comparingXY %d,%d and %d,%d = %b".format(t1.x, t1.y, t2.x, t2.y, res))
    return res

  // if t1 is strictly less than t2 return true, in all other cases return false.
  def sortByYX(t1: RedTile, t2: RedTile) : Boolean =
    var res : Boolean = false
    if (t1.y < t2.y) then res = true
    else if (t1.y == t2.y) && (t1.x < t2.x) then res = true
    end if
    println("comparingYX %d,%d and %d,%d = %b".format(t1.x, t1.y, t2.x, t2.y, res))
    return res

end RedTile


class AocApp extends App :
  println("Run...")

  var tiles : ListBuffer[RedTile] = new ListBuffer[RedTile]()
  var tilesSorted : ListBuffer[RedTile] = new ListBuffer[RedTile]()

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
      println("vector.tiles= " + tile.getX() + " " + tile.getY())

  tilesSorted = tiles.sortWith(tiles(0).sortByYX)
  
  println()
  for (tile <- tilesSorted)
      println("vector.tilesSorted= " + tile.getX() + " " + tile.getY() + " prev " + tile.getPrev() + " next " + tile.getNext())

  println()
  var indexMinX : Int = 0
  var index : Int = 0
  while (index < tilesSorted.size)
      if (tilesSorted(index).getX() < tilesSorted(indexMinX).getX()) then
          indexMinX = index
      index = index + 1
  println("Index minX: " + indexMinX + " x,y " + tilesSorted(indexMinX).getX() + "," + tilesSorted(indexMinX).getY())

  println()
  println("Create outer polygone, convex hull...")
  // start from first tile
  var curTile : RedTile = tilesSorted( indexMinX )
  curTile.visited = true
  var tilesVisited : Int = 1
  var lastVisited : Int = 0
  //var tileIndex : Int = 0
  var walking : Boolean = true
  while (curTile.getNext() == null) && walking do
      // print visited
      println(" visted = " + tilesVisited)

      if (lastVisited == tilesVisited)
          walking = false
          println("EXIT")
      else
         lastVisited = tilesVisited

      // find next
      breakable {
          for (tile <- tilesSorted)
              // if tile got no input, and not ourselves
              if ((tile.getPrev() == null) && (tile != curTile)) then
                  if (tile.getY() == curTile.getY()) && (tile.getX() < curTile.getX()) then
                      // move left
                      println("left")
                      println("  (x,y) ---> (x,y): " + curTile.getX() + "," + curTile.getY() + " ---> " + tile.getX() + "," + tile.getY())
                      tile.setPrev(curTile)
                      curTile.setNext(tile)
                      tile.setVisited(true)
                      tilesVisited = tilesVisited + 1
                      curTile = tile
                      break
                  else if (tile.getX() == curTile.getX()) && (tile.getY() < curTile.getY()) then
                      // move up
                      println("up")
                      tile.setPrev(curTile)
                      curTile.setNext(tile)
                      println("  (x,y) ---> (x,y): " + curTile.getX() + "," + curTile.getY() + " ---> " + tile.getX() + "," + tile.getY())
                      tile.setVisited(true)
                      tilesVisited = tilesVisited + 1
                      curTile = tile
                      break
                  else if (tile.getY() == curTile.getY()) && (tile.getX() > curTile.getX()) then
                      // move right
                      println("right")
                      println("  (x,y) ---> (x,y): " + curTile.getX() + "," + curTile.getY() + " ---> " + tile.getX() + "," + tile.getY())
                      tile.setPrev(curTile)
                      curTile.setNext(tile)
                      tile.setVisited(true)
                      tilesVisited = tilesVisited + 1
                      curTile = tile
                      break
                  else if (tile.getX() == curTile.getX()) && (tile.getY() > curTile.getY()) then
                      // move down
                      println("down")
                      println("  (x,y) ---> (x,y): " + curTile.getX() + "," + curTile.getY() + " ---> " + tile.getX() + "," + tile.getY())
                      tile.setPrev(curTile)
                      curTile.setNext(tile)
                      tile.setVisited(true)
                      tilesVisited = tilesVisited + 1
                      curTile = tile
                      break
                  //else
                  //    println("No tile found ?! try next")
              end if
      }
      
      //walking = false
  end while

  println()
  for (tile <- tilesSorted)
      println("vector.tilesSorted= " + tile.getX() + " " + tile.getY() + " prev " + tile.getPrev() + " next " + tile.getNext())

  println()
  println("Evaluate all possible squares...")

  var i : Int = 0
  var max : Long = 0;
  while (i < tiles.size) do
      var j : Int = i + 1
      while (j < tiles.size) do
          var area : Long = tiles(i).getArea( tiles(j) )

          var x1 : Long = tiles(i).getX()
          var y1 : Long = tiles(i).getY()
          var x2 : Long = tiles(j).getX()
          var y2 : Long = tiles(j).getY()

          // check that square is allowed and not disqualified
          var disq : Boolean = false

          // SOLUTION
          // have 2 sorted Vector copies of Red tiles: TopDownLeftRight, LeftRightTopDown
          // walk through all ray-casting, 0 --> # (black) # ---> # green -----> end (black) --- next row, etc
          // check if any (black) inside --> disqualify

          // v2: Create (unique?) polygone, start minXY, then try go left, up, right, down. create convex hull.
          // then all lines in polygone may not cross lines of rectangle. - test in inner loop, traversing all squares, set disq if crossing.


          breakable {
              for (tile <- tiles)
                  if (tile != tiles(i)) && (tile != tiles(j)) then
                      if (tile.linesCrossNext( x1, y1, x2, y2 ) ) then
                          println("DISQ " + i + "," + j)
                          disq = true
                          break
          }

          
          if ((area > max) && (! disq)) then
              max = area
              println("new max " + max + " from " + x1 + "," + y1 + " - " + x2 + "," + y2 + ".")
          j = j + 1
      i = i + 1
  end while

  // 1518119876 too low.
  // 4748769124 too high.
  // 1525991432 - correct
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
