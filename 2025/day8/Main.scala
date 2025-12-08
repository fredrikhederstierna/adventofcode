
import java.io.BufferedReader;
import java.io.InputStreamReader;

import java.util.StringTokenizer;
import java.util.stream.Collectors;

import scala.util.control.Breaks._

class JuncBox(xin : Int, yin : Int, zin : Int, xout : Int, yout : Int, zout : Int, nboxes : Int) :

  var myCircuit : Set[JuncBox] = Set()
  var myConnected : Set[JuncBox] = Set()

  var nBoxes : Int = nboxes

  var xIn : Int = xin
  var yIn : Int = yin
  var zIn : Int = zin 

  var xOut : Int = xout
  var yOut : Int = yout
  var zOut : Int = zout

  def getNboxes() : Int =
    this.nBoxes
  def setNboxes(boxes : Int) : Unit =
    this.nBoxes = boxes

  def getXin() : Int =
    this.xIn
  def getYin() : Int =
    this.yIn
  def getZin() : Int =
    this.zIn

  def getXout() : Int =
    this.xOut
  def getYout() : Int =
    this.yOut
  def getZout() : Int =
    this.zOut

  def compare(that: JuncBox): Int = (this.nBoxes) compare (that.nBoxes)

  // override
  def toName() : String =
      return "n: " + nBoxes + "; in: " + xIn + "," + yIn + "," + zIn + "; out: " + xOut + "," + yOut + "," + zOut + ""

  def minDistance(jbox : JuncBox) : Double =
      var d_min : Double = 0.0

      val d_in_in   : Double = Math.sqrt( Math.pow(this.getXin() -jbox.getXin(),2) + Math.pow(this.getYin()  -jbox.getYin(),2)  + Math.pow(this.getZin() -jbox.getZin(),2) )
      val d_in_out  : Double = Math.sqrt( Math.pow(this.getXin() -jbox.getXout(),2) + Math.pow(this.getYin() -jbox.getYout(),2) + Math.pow(this.getZin() -jbox.getZout(),2) )
      val d_out_in  : Double = Math.sqrt( Math.pow(this.getXout()-jbox.getXin(),2) + Math.pow(this.getYout() -jbox.getYin(),2)  + Math.pow(this.getZout()-jbox.getZin(),2) )
      val d_out_out : Double = Math.sqrt( Math.pow(this.getXout()-jbox.getXout(),2) + Math.pow(this.getYout()-jbox.getYout(),2) + Math.pow(this.getZout()-jbox.getZout(),2) )

      // return smallest...
      if (d_in_in < d_in_out) && (d_in_in < d_out_in) && (d_in_in < d_out_out) then
          d_min = d_in_in
      else if (d_in_out < d_out_in) && (d_in_out < d_out_out) then
          d_min = d_in_out
      else if (d_out_in < d_out_out) then
          d_min = d_out_in
      else
          d_min = d_out_out

      return d_min

  /*
  def inSameCircuit(jbox : JuncBox) : Boolean =
      //println("  Testing jbox: " + jbox + " (this " + this + ")")
      if (jbox == this)
          return true
      var sameCirc : Boolean = myCircuit.contains(jbox)
      //println("  Testing jbox: NOT SAME OBJ : same = " + sameCirc)
      return sameCirc
  */

  def isConnected(jbox : JuncBox) : Boolean =
      if (jbox == this)
          return true
      var directConnected : Boolean = myConnected.contains(jbox)
      return directConnected

  def setCircuit(circuit : Set[JuncBox]) : Unit =
      this.myCircuit = circuit
    
  def getCircuit() : Set[JuncBox] =
      // always add myself
      var fullCircuit = this.myCircuit + this
      return fullCircuit

  def connectLocal(jbox : JuncBox) : Unit =
      // always add myself
      this.myConnected = myConnected + this
      // add new boxes
      this.myConnected = myConnected + jbox
      
  def connectCircuit(jbox : JuncBox) : Unit =
      // always add myself
      this.myCircuit = myCircuit + this
      // add new boxes
      this.myCircuit = myCircuit ++ jbox.getCircuit()
      this.nBoxes = myCircuit.size
      for (jb <- this.myCircuit)
          jb.setNboxes(this.nBoxes)
          jb.setCircuit(this.myCircuit)

  /*
  def mergeWires(jbox : JuncBox) : Unit =

      val d_in_in   : Double = Math.sqrt( Math.pow(this.getXin() -jbox.getXin(),2) + Math.pow(this.getYin()  -jbox.getYin(),2)  + Math.pow(this.getZin() -jbox.getZin(),2) )
      val d_in_out  : Double = Math.sqrt( Math.pow(this.getXin() -jbox.getXout(),2) + Math.pow(this.getYin() -jbox.getYout(),2) + Math.pow(this.getZin() -jbox.getZout(),2) )
      val d_out_in  : Double = Math.sqrt( Math.pow(this.getXout()-jbox.getXin(),2) + Math.pow(this.getYout() -jbox.getYin(),2)  + Math.pow(this.getZout()-jbox.getZin(),2) )
      val d_out_out : Double = Math.sqrt( Math.pow(this.getXout()-jbox.getXout(),2) + Math.pow(this.getYout()-jbox.getYout(),2) + Math.pow(this.getZout()-jbox.getZout(),2) )

      // return smallest...
      if (d_in_in < d_in_out) && (d_in_in < d_out_in) && (d_in_in < d_out_out) then
          this.xIn = jbox.xOut
          this.yIn = jbox.yOut
          this.zIn = jbox.zOut
      else if (d_in_out < d_out_in) && (d_in_out < d_out_out) then
          this.xIn = jbox.xIn
          this.yIn = jbox.yIn
          this.zIn = jbox.zIn
      else if (d_out_in < d_out_out) then
          this.xOut = jbox.xOut
          this.yOut = jbox.yOut
          this.zOut = jbox.zOut
      else
          this.xOut = jbox.xIn
          this.yOut = jbox.yIn
          this.zOut = jbox.zIn

      this.nBoxes = this.nBoxes + jbox.nBoxes
  */

end JuncBox


class AocApp extends App :

  var jboxArray : Vector[JuncBox] = Vector()

  println("Run...")

  // stdin
  val input = getInput()
  //println(input)


  var tokens = new StringTokenizer(input, "\n", false);
  while tokens.hasMoreTokens() do
      var token = tokens.nextToken().strip()
      var slen : Integer = token.length()
      println("token: " + token)

      var xyz : Array[String] = token.split(",")
      var x = xyz(0).toInt
      var y = xyz(1).toInt
      var z = xyz(2).toInt
      //println("  x = " + x + "; y = " + y + "; z = " + z)
      jboxArray = jboxArray :+ JuncBox(x,y,z,x,y,z,1)
  end while

  println()
  println("jboxArray.size = " + jboxArray.size)

  var count : Int = 0
  for (c <- jboxArray)
      print("JuncBox[" + count + "] = { obj = " + c)
      print("\tn = " + c.getNboxes() + ",")
      print("\tin = (" + c.getXin() + "," + c.getYin() + "," + c.getZin() + "),")
      print("\tout = (" + c.getXout() + "," + c.getYout() + "," + c.getZout() + ")")
      println("\t}")
      count += 1
  println()


  val n_iter_max = 1000
  var n_iter = 1
  while (n_iter <= n_iter_max)
      print("Iter[" + n_iter + "]: ")

      // 1. find closest wire-pair
      // 2. merge
      var min_dist : Double = 999_999_999
      var bestBox1 : JuncBox = null
      var bestBox2 : JuncBox = null
      for (bi1 <- 0 until jboxArray.size)
          val box1 = jboxArray(bi1)
          for (bi2 <- (bi1 + 1) until jboxArray.size)
              val box2 = jboxArray(bi2)

              //println("  COMPARE bi1 " + bi1 + " bi2 " + bi2 + " box1 " + box1 + " box2 " + box2)
              //if (! box1.inSameCircuit(box2))
              //if ((! box1.isConnected(box2)) && (! box2.isConnected(box1)))
              if (! box1.isConnected(box2))
                  val dist_test = box1.minDistance(box2)
                  //println("    DIST=" + dist_test + " MIN=" + min_dist)
                  if dist_test < min_dist then
                      bestBox1 = box1
                      bestBox2 = box2
                      min_dist = dist_test

 
      //println("  Nbr jboxArray BEFORE = " + jboxArray.size)
      //XX println(" Min = " + min_dist + "\t Best = " + bestBox1 + "(" + bestBox1.toName() + ") - " + bestBox2 + "(" + bestBox2.toName() + ")")
      // merge 1 -> 2, update box-count
      if (bestBox1 != null && bestBox2 != null) then
          //println("  MERGE")
          //bestBox1.mergeWires( bestBox2 )
          bestBox1.connectLocal( bestBox2 )
          bestBox2.connectLocal( bestBox1 )
          bestBox1.connectCircuit( bestBox2 )
          bestBox2.connectCircuit( bestBox1 )
          // remove merged item, filter out
          //jboxArray = jboxArray.filter(_ != bestBox2)
      //println("  Nbr jboxArray AFTER = " + jboxArray.size)

      n_iter += 1
  end while


  // After making the 10 shortest connections, there are 11 circuits:
  //   1 circuit which contains 5 junction boxes
  //   1 circuit which contains 4 junction boxes
  //   2 circuits which contain 2 junction boxes
  //   7 circuits which contain 1 junction box
  // Multiplying together the sizes of the 3 largest circuits (5, 4, and one of the circuits of size 2) produces 40.


  /*
  println()
  var bi : Int = 0
  for (w <- jboxArray)
      var n_box : Int = w.getNboxes()
      var n_cir : Int = w.getCircuit().size
      println("w[" + bi + "]: " + n_box + " boxes == set.size " + n_cir + " SHOULD BE SAME")
      bi += 1
  */


  /*
  println()
  bi = 0
  jboxArray = jboxArray.sortBy(_.nBoxes).reverse
  for (w <- jboxArray)
      var n_box : Int = w.getNboxes()
      var n_cir : Int = w.getCircuit().size
      println("w[" + bi + "]: " + n_box + " boxes == set.size " + n_cir + " SHOULD BE SAME2")
      bi += 1
  //jboxArray.foreach(println)
  */


  var mySets : Set[JuncBox] = Set()
  var mySizes : Vector[Int] = Vector()
  for (t <- jboxArray)
      var tset : Set[JuncBox] = t.getCircuit()
      if (mySets.intersect(tset) != tset) then
          mySets = mySets ++ tset
          mySizes = mySizes :+ tset.size

  mySizes = mySizes.sorted.reverse
          
  println()
  print("sizex = ")
  for (sx <- mySizes)
      print(sx + " ")
  println()
  
  
  //var prod_box : Int = 0
  //prod_box = jboxArray(0).getNboxes() * jboxArray(1).getNboxes() * jboxArray(2).getNboxes()
  //println()
  //println("Prod-Box = " + prod_box)


  // 4275 -- too low
  var prod_box : Int = 0
  prod_box = mySizes(0) * mySizes(1) * mySizes(2)
  println()
  println("Prod-Box = " + prod_box)
  
  
  println("END.")


  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines


end AocApp


@main def Main: Unit =
  println("AOC: Day 8")
  val app = new AocApp()
