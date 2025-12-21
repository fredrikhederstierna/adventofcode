
import java.util.StringTokenizer;
import java.io.BufferedReader;
import java.io.InputStreamReader;

import java.util.stream.Collectors;

import scala.collection.mutable.ListBuffer

import scala.util.control.Breaks._



class Machine(ind_on : Int) :

  var indicator_on = ind_on

  var buttons : ListBuffer[Int] = new ListBuffer[Int]()
  var presses : Array[Int] = new Array[Int](2048)

  var joltsCorrect : ListBuffer[Int] = new ListBuffer[Int]()
  var joltsHighscore : ListBuffer[Int] = new ListBuffer[Int]()
  //var joltsHighscoreSum : Int = 0
  var joltsHighscorePresses : Int = 0

  def getIndOn() : Int =
    this.indicator_on
  def setIndOn(ind_on : Int) : Unit =
    this.indicator_on = ind_on

  def clear() : Unit =
    for (i <- Range(0, presses.size))
        presses(i) = 999999
    end for
    // new test
    joltsCorrect = new ListBuffer[Int]()
    joltsHighscore = new ListBuffer[Int]()
    //joltsHighscoreSum = 9999999
    joltsHighscorePresses = 9999999

  def addButton(b : Int) : Unit =
    buttons += b

  def addJolt(j : Int) : Unit =
    joltsCorrect += j
    println("addedJoltCorrect(" + j + "): size" + joltsCorrect.size)

  def isJoltCorrect(jvec : Array[Int]) : Boolean =
    var correct : Boolean = true
    var i : Int = 0
    //println("isJoltCorrect(): size " + joltsCorrect.size)
    breakable {
    for (jj <- joltsCorrect)
        //println("isJoltCorrect(): test correct " + jj + " == test " + jvec(i))
        if (jj != jvec(i))
            correct = false
            break
        i += 1
    end for
    }
    return correct

  def isJoltOverflow(jvec : Array[Int]) : Boolean =
    var overflow : Boolean = false
    var i : Int = 0
    breakable {
    for (jj <- joltsCorrect)
        if (jvec(i) > jj)
            overflow = true
            break
        i += 1
    end for
    }
    return overflow

  /*
  def getJoltSum(jvec : Array[Int]) : Int =
    var sum : Int = 0
    for (j <- jvec)
        sum += j
    end for
    return sum
  */

  def updateJoltHighscore(jvec : Array[Int], score : Int) : Unit =
    var sum : Int = 0
    joltsHighscore = new ListBuffer[Int]()
    for (j <- jvec)
        joltsHighscore += j
        sum += j
    //joltsHighscoreSum = sum
    joltsHighscorePresses = score
    println("  New high = " + score)

  def getBest() : Int =
    return ( presses( this.indicator_on ) )

  /*
  def getBestJolt() : Int =
    // debug
    var k : Int = 0
    for (j <- joltsHighscore)
        println("getBestJoltHighscore(" + k + ") = " + j)
        k = k + 1
    return joltsHighscoreSum
  */
  def getBestJolt() : Int =
    return joltsHighscorePresses

  def toggleButton(state_bits : Int, i : Int) : Int =
    return ( state_bits ^ buttons(i) )

  def incJolt(jvec : Array[Int], button_index : Int) : Unit =
    var bits : Int = buttons( button_index )
    var xi : Int = 0
    while (bits > 0)
       if ((bits & 1) == 1) then
           jvec( xi ) += 1
       bits = bits >> 1
       xi = xi + 1
    end while

  def decJolt(jvec : Array[Int], button_index : Int) : Unit =
    var bits : Int = buttons( button_index )
    var xi : Int = 0
    while (bits > 0)
       if ((bits & 1) == 1) then
           jvec( xi ) -= 1
       bits = bits >> 1
       xi = xi + 1
    end while

  def traverse(state_bits : Int, press_count : Int) : Boolean =
    //println("    traverse state = " + state_bits + "; presses(x) = " + presses( state_bits ) + "; press_count = " + press_count)
    if (press_count < presses( state_bits )) then
        presses( state_bits ) = press_count
    else
        //println("    skip")
        return false
    var keep_search : Boolean = true
    while keep_search do
        var found_better : Boolean = false
        for (i <- Range(0, buttons.size))
            //println("        traverse2( index " + i + " )")
            var state_bits_temp : Int = toggleButton(state_bits, i)
            var better : Boolean = traverse( state_bits_temp, press_count + 1 )
            if (better) then
                found_better = true
        end for
        keep_search = found_better
    end while
    return true

  def traverseJolts(jvec : Array[Int], button_index : Int, press_count : Int) : Boolean =
    if (press_count >= joltsHighscorePresses) then
        return false

    incJolt( jvec, button_index )

    if (isJoltOverflow(jvec)) then
        decJolt( jvec, button_index )
        return false

    if (isJoltCorrect(jvec)) then
        if ((press_count + 1) < joltsHighscorePresses) then
            updateJoltHighscore(jvec, press_count + 1)
        decJolt( jvec, button_index )
        return false

    //println(jvec.mkString(" "))

    var keep_search : Boolean = true
    while keep_search do
        var found_better : Boolean = false
        for (i <- Range(button_index, buttons.size))
            var better : Boolean = traverseJolts( jvec, i, press_count + 1 )
            if (better) then
                found_better = true
        end for
        keep_search = found_better
    end while

    decJolt( jvec, button_index )
    return false

end Machine



class AocApp extends App :
  println("Run...")
  println()


  var totPresses : Int = 0
  var totPressesJolt : Int = 0

  
  // stdin
  val input = getInput()
  //println(input)

  var machine_count : Int = 0

  var tokens = new StringTokenizer(input, "\n", false);
  while tokens.hasMoreTokens() do
      var token = tokens.nextToken().strip()
      var slen : Integer = token.length()
      println("token: " + token)

      var rr : Array[String] = token.split(raw"[\]\{]")

      // indicator correct ON state
      var s1 : String = rr(0).substring(1).strip().replace('.','0').replace('#','1')
      s1 = s1.reverse
      var i1 : Int = Integer.parseInt(s1, 2)

      println("NEW MACHINE[" + machine_count + "] : " + i1)
      var m : Machine = new Machine(i1)
      machine_count = machine_count + 1

      // buttons
      var s2 : String = rr(1).strip()
      var bb : Array[String] = s2.split(raw"[\( \)]")
      for (b <- bb)
          var mask : Int = 0
          if (b.length > 0) then
              //println("  b = " + b)
              var bb2 : Array[String] = b.split(raw",")
              for (b2 <- bb2)
                  //println("      b2 = " + b2)
                  mask = mask | (1 << b2.toInt)
              //println("        add button = " + mask)
              m.addButton( mask )



      // try press buttons....
      m.clear()
      m.traverse(0, 0) // 0 initial value, 0 presses

      var best : Int = m.getBest()
      println("Best = " + best)

      totPresses = totPresses + best


      

      
      println()
      println("Now add jolts: contacts " + s1)

      m.clear()
      
      // joltage requirements
      var s3 : String = rr(2).substring(0, rr(2).length - 1).strip()
      var jj : Array[String] = s3.split(raw",")
      for (j <- jj)
          if (j.length > 0) then
              println("    addJolt = " + j)
              m.addJolt( j.toInt )

      println("Added all jolts")
      println()

      // try press buttons with jolt....
      // create init vector
      var jvec : Array[Int] = new Array[Int]( s1.length )
      var k : Int = 0
      for (j <- jvec)
          jvec(k) = 0
          k += 1
      end for
      m.traverseJolts(jvec, 0, 0) // 0 initial value, 0 index, 0 presses

      var bestJolt : Int = m.getBestJolt()
      println("BestJolt = " + bestJolt)

      totPressesJolt = totPressesJolt + bestJolt

      

      
      
      //println("READ: s1='" + s1 + "' (" + i1 + ") ----->  s2='"  + s2 + "'  ----->  s3='" + s3 + "'")
      //println()

  end while


  //println()
  //for (tile <- tiles)
  //    println("tile= " + tile.getX() + " " + tile.getY())


  println("TotPresses = " + totPresses)
  println("TotPressesJolt = " + totPressesJolt)


  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 10")
  val app = new AocApp()
