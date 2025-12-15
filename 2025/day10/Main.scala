
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
  
  def getIndOn() : Int =
    this.indicator_on
  def setIndOn(ind_on : Int) : Unit =
    this.indicator_on = ind_on

  def clear() : Unit =
    for (i <- Range(0, presses.size))
        presses(i) = 999999
    
  def addButton(b : Int) : Unit =
    buttons += b

  def getBest() : Int =
    return ( presses( this.indicator_on ) )

  def toggleButton(state_bits : Int, i : Int) : Int =
    return ( state_bits ^ buttons(i) )

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

//  def isOn() : Boolean =
//    state_bits == indicator_on

end Machine



class AocApp extends App :
  println("Run...")
  println()


  var totPresses : Int = 0

  
  // stdin
  val input = getInput()
  //println(input)

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

      println("NEW MACHINE : " + i1)
      var m : Machine = new Machine(i1)

      // buttons
      var s2 : String = rr(1).strip()
      var bb : Array[String] = s2.split(raw"[\( \)]")
      for (b <- bb)
          var mask : Int = 0
          if (b.length > 0) then
              println("  b = " + b)
              var bb2 : Array[String] = b.split(raw",")
              for (b2 <- bb2)
                  println("      b2 = " + b2)
                  mask = mask | (1 << b2.toInt)
              println("        add button = " + mask)
              m.addButton( mask )



      // try press buttons....
      m.clear()
      m.traverse(0, 0) // 0 initial value, 0 presses

      var best : Int = m.getBest()
      println("Best = " + best)

      totPresses = totPresses + best
              
      // joltage requirements
      var s3 : String = rr(2).substring(0, rr(2).length - 1).strip()
      
      println("READ: s1='" + s1 + "' (" + i1 + ") ----->  s2='"  + s2 + "'  ----->  s3='" + s3 + "'")
      println()

  end while


  //println()
  //for (tile <- tiles)
  //    println("tile= " + tile.getX() + " " + tile.getY())


  println("TotPresses = " + totPresses)


  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 10")
  val app = new AocApp()
