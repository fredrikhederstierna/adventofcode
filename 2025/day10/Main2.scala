
import java.util.StringTokenizer;
import java.io.BufferedReader;
import java.io.InputStreamReader;

import java.util.stream.Collectors;

import scala.collection.mutable.ListBuffer

import scala.util.control.Breaks._



// SimplexSolver
import org.apache.commons.math3.optim.linear._
import org.apache.commons.math3.optim.nonlinear.scalar.GoalType
import java.util.ArrayList


// Linear system
import org.apache.commons.math3.linear._
//import org.apache.commons.math3.linear.RealMatrix
//import org.apache.commons.lang3.ArrayUtils


class Machine(ind_on : Int) :

  var indicator_on = ind_on

  var buttons : ListBuffer[Int] = new ListBuffer[Int]()
  var buttonsList : ListBuffer[Array[String]] = new ListBuffer[Array[String]]()
  var presses : Array[Int] = new Array[Int](2048)


  var joltsCorrect : ListBuffer[Int] = new ListBuffer[Int]()
  var joltsHighscore : ListBuffer[Int] = new ListBuffer[Int]()
  //var joltsHighscoreSum : Int = 0
  var joltsHighscorePresses : Int = 0

  def getIndOn() : Int =
    this.indicator_on
  def setIndOn(ind_on : Int) : Unit =
    this.indicator_on = ind_on


  // return true if A is a true subset of B
  def isTrueSubSet(A : Array[String], B : Array[String]) : Boolean =
    var subset : Boolean = true
    if (A.size >= B.size) then
      return false
    breakable {
    for (a <- A)
      var contains : Boolean = false
      breakable {
      for (b <- B)
        if (a == b) then
          contains = true
          break
        end if
      end for
      }
      // if element 'a' existed in B, so is still true subset
      // if not, then not true subset
      if (! contains) then
        subset = false
        break
      end if
    end for
    }
    return subset

  def swapButtonOrder(i : Int, j : Int) : Unit =
    var T1 : Array[String] = buttonsList(i)
    var T2 : Array[String] = buttonsList(j)
    buttonsList(i) = T2
    buttonsList(j) = T1

  def printButtons() : Unit =
    var n : Int = buttonsList.size
    for (i <- Range(0, n))
      println("debug bi " + i + " = " + buttonsList(i))
      for (j <- buttonsList(i))
        println("      s = " + j)
      end for
    end for

  // sort buttons to be in order of true subsets
  def sortButtons() : Unit =
    var n : Int = buttonsList.size

    // print unsorted
    //printButtons()

    var swap_done : Boolean = true
    while swap_done do
      swap_done = false
      for (bi <- Range(0, n))
        for (bi2 <- Range(0, n))
          var subset : Boolean = isTrueSubSet(buttonsList(bi), buttonsList(bi2))
          //println("   isTrueSubset(" + bi + "," + bi2 + ") = " + subset)
          if (subset) then
            if (bi2 > bi)
              //println("SWAP")
              swapButtonOrder(bi, bi2)
              swap_done = true
        end for
      end for
    end while

    //printButtons()

  def linSystem() : Unit =
    println("System")

    val matSys : RealMatrix = MatrixUtils.createRealMatrix(3, 3)
    val r : Int = 1
    val c : Int = 2
    val varVal: Double = 8.8
    matSys.setEntry(r, c, varVal)

    val myArray : Array[Double] = Array(3.0, 20.0, 89.0) // 3 rows
    val matEqu : RealVector = MatrixUtils.createRealVector( myArray )

    val solver : DecompositionSolver = new LUDecomposition( matSys ).getSolver()
    val solution : RealVector = solver.solve( matEqu )

    println("Solution : " + solution)




  // jolt J { 10, 20, 30 }
  // buttons B [ 0 2 ] , [ 1 3 ], [ 0 3 ]
  //
  //
  //  P * B = J
  //
  //      indicator affected
  //  b   [ 1 0 1 1 ]T  [ p1 ]
  //  u   [ 1 0 0 1 ]   [ p2 ]   [ 101 ]
  //  t   [ 0 0 1 1 ]   [ p3 ]   [  23 ]
  //  t   [ 0 1 1 0 ] * [ p4 ] = [ 164 ]
  //  o   [ 1 1 1 1 ]   [ p5 ]   [  69 ]
  //  n   [ 1 1 0 0 ]   [ p6 ]
  //  s   [ 0 0 0 1 ]   [ p7 ]
  //
  //  (p0 * 0|1) + (p1 * 0|1) + ...    = 10
  //  (p0 * 0|1) + (p1 * 0|1) + ...    = 20
  //  (p0 * 0|1) + (p1 * 0|1) + ...    = 30
  //
  // minimize sum of presses of buttons:  p0 + p1 + p2 + ... pn
  //
  //
  // Under-decided linear equation system optimization problem.
  // Use ILP - Interger Linear Programming solver library.
  //
  // >> B = [0 0 0 1; 0 1 0 1; 0 0 1 0; 0 0 1 1; 1 0 1 0; 1 1 0 0]
  //
  // B =
  //   0   0   0   1
  //   0   1   0   1
  //   0   0   1   0
  //   0   0   1   1
  //   1   0   1   0
  //   1   1   0   0
  //
  // >> P = [1 3 0 3 1 2]
  // P =
  //   1   3   0   3   1   2
  //
  // >> S = P * B
  //
  // S =
  //   3   5   4   7
  //
  // >> sum(P)
  //   ans = 10
  //

  def simplexSolve(n_indicators : Int) : Int =
    println("Simples: n_indicators = " + n_indicators)

    val n_buttons : Int = buttons.size



    // JOLTS
    var J : Array[Double] = new Array[Double](n_indicators)
    for (jix <- Range(0, n_indicators))
        J(jix) = joltsCorrect(jix)

    //var ji : Int = 0
    //for (j <- J)
    //    println("  J[" + ji + "] = " + j)
    //    ji = ji + 1



   // OBJ func
    var F : Array[Double] = new Array[Double](n_buttons)
    for (bi <- Range(0, n_buttons))
        F(bi) = 1
    //var fi : Int = 0
    //for (f <- F)
    //    println("  F[" + fi + "] = " + f)
    //    fi = fi + 1
    val F_obj = new LinearObjectiveFunction(F, 0)
    //
    // CONSTRAINTS
    val Constraints = new ArrayList[LinearConstraint]()
    //Constraints.add(new LinearConstraint( Array(2.0, 8.0), Relationship.EQ, 13.0 ))



    // setup equations for each indicator
    for (ii <- Range(0, n_indicators))

        // run through all buttons
        var A : Array[Double] = new Array[Double](n_buttons)
        for (bi <- Range(0, n_buttons))

            // check buttons[bi]
            val ba : Array[String] = buttonsList(bi)
            //println("Test button[" + bi + "] ba = " + ba)
            // is indicator affected by this button?
            var found : Boolean = false
            breakable {
              for (b <- ba)
                  //println("    b " + b + " ii " + ii)
                  if (b.toInt == ii) then
                      found = true
                      break
            }
            if (found) then
                //println("  button[" + bi + "] #CONTAINS# indicator " + ii)
                A(bi) = 1
            else
                //println("  button[" + bi + "] ***NOT*** contained indicator " + ii)
                A(bi) = 0

        //var ai : Int = 0
        //for (a <- A)
        //    println("  A[" + ai + "] = " + a)
        //    ai = ai + 1

        // ADD CONSTRAINT
        //Constraints.add(new LinearConstraint( Array(2.0, 8.0), Relationship.EQ, 13.0 ))
        Constraints.add(new LinearConstraint( A, Relationship.EQ, J(ii) ))



    // Maximize 3x + 5y subject to:
    //   2x + 8y <= 13
    //   5x - 1y <= 11
    //val f_obj = new LinearObjectiveFunction(Array(3.0, 5.0), 0)
    //val constraints = new ArrayList[LinearConstraint]()
    //constraints.add(new LinearConstraint(Array(2.0, 8.0),  Relationship.LEQ, 13.0))
    //constraints.add(new LinearConstraint(Array(5.0, -1.0), Relationship.LEQ, 11.0))
    //constraints.add(new LinearConstraint(Array(2.0, 8.0),  Relationship.EQ, 13.0))
    //constraints.add(new LinearConstraint(Array(5.0, -1.0), Relationship.EQ, 11.0))

    val solver = new SimplexSolver()
    val solution = solver.optimize(
      //f_obj,
      //new LinearConstraintSet(constraints),
      //GoalType.MAXIMIZE,
      F_obj,
      new LinearConstraintSet(Constraints),
      GoalType.MINIMIZE,
      new NonNegativeConstraint(true)
     )

    //println(s"Optimal value: ${solution.getValue}")
    //println(s"x: ${solution.getPoint()(0)}, y: ${solution.getPoint()(1)}")

    var absDiff : Double = Math.abs(solution.getValue - solution.getValue.toInt);
    if (absDiff > 0.001) then
        println("     ===> WARNING. Non integer solution: " + solution.getValue + " DIFF = " + absDiff)
    // Get the optimal point (variable values)
    var point : Array[Double] = solution.getPoint()
    for (k <- Range(0, point.size))
        println("    k[" + k + "] " + point(k))
    //println(s"    x: ${solution.getPoint()(0)}, y: ${solution.getPoint()(1)}")

    return (solution.getValue + 0.001).toInt


  def clear() : Unit =
    for (i <- Range(0, presses.size))
        presses(i) = 999999
    end for
    // simplex
    //buttonsList = new ListBuffer[Array[String]]()
    // new test
    joltsCorrect = new ListBuffer[Int]()
    joltsHighscore = new ListBuffer[Int]()
    //joltsHighscoreSum = 9999999
    joltsHighscorePresses = 9999999

  def addButton(b : Int) : Unit =
    buttons += b
  def addButtonArray(ba : Array[String]) : Unit =
    buttonsList += ba

  def addJolt(j : Int) : Unit =
    joltsCorrect += j
    //println("addedJoltCorrect(" + j + "): size" + joltsCorrect.size)

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

  inline def updateJoltHighscore(jvec : Array[Int], score : Int) : Unit =
    //var sum : Int = 0
    joltsHighscore = new ListBuffer[Int]()
    for (j <- jvec)
        joltsHighscore += j
        //sum += j
    //joltsHighscoreSum = sum
    joltsHighscorePresses = score
    println("  New high = " + score + " button presses")

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

  def incJolt(jvec : Array[Int], button_index : Int) : Boolean =
    var overflow : Boolean = false
    var bits : Int = buttons( button_index )
    var xi : Int = 0
    while (bits > 0)
       if ((bits & 1) == 1) then
           jvec( xi ) += 1
           if (jvec( xi ) > joltsCorrect( xi ))
               overflow = true
       bits = bits >> 1
       xi = xi + 1
    end while
    return overflow

  inline def decJolt(jvec : Array[Int], button_index : Int) : Unit =
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

  // return true if we should skip iterate on this path
  def traverseJolts(jvec : Array[Int], button_index : Int, press_count : Int) : Boolean =
    // we already found equal or better solution, skip
    if (press_count >= joltsHighscorePresses) then
        return true

    // press button to increase jolt
    var ovf : Boolean = incJolt( jvec, button_index )
    // did new button press exceed max required presses?
    //if (isJoltOverflow(jvec)) then
    if (ovf) then
        decJolt( jvec, button_index )
        return false

    // check for new highscore
    if (isJoltCorrect(jvec)) then
        if ((press_count + 1) < joltsHighscorePresses) then
            updateJoltHighscore(jvec, press_count + 1)
        decJolt( jvec, button_index )
        // we cannot find lower on this path
        return true

    //println(jvec.mkString(" "))

    //var keep_search : Boolean = true
    //while keep_search do
    //var dead_end : Boolean = false
    breakable {
        var skip : Boolean = false
        for (i <- Range(button_index, buttons.size))
            skip = traverseJolts( jvec, i, press_count + 1 )
            if (skip) then
                break
        end for
    }
    //keep_search = dead_end
    //end while

    decJolt( jvec, button_index )
    return false

end Machine


// IDEA: handle one value at time, first setup #0, then on currect { ... } tick index forward. can skip more branches?


class AocApp extends App :
  println("Run...")
  println()


  var totPresses : Int = 0
  var totPressesJolt : Int = 0
  var totPressesSimplex : Int = 0

  
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
              // simplex
              m.addButtonArray( bb2 )
              for (b2 <- bb2)
                  //println("      b2 = " + b2)
                  mask = mask | (1 << b2.toInt)
              //println("        add button = " + mask)
              m.addButton( mask )





      ///////////////
      val runPart1 = false
      if (runPart1) then
        // try press buttons....
        m.clear()
        m.traverse(0, 0) // 0 initial value, 0 presses

        var best : Int = m.getBest()
        println("Best = " + best)

        totPresses = totPresses + best
      end if


      


      
      ///////////////
      val runPart2 : Boolean = true
      if (runPart2) then
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

        // sort buttons ------------------------
        println("Sort buttons...")
        m.sortButtons()
        println("Buttons sorted.")

        // try press buttons with jolt....

        // create empty init vector with 0 presses
        var jvec : Array[Int] = new Array[Int]( s1.length )
        var k : Int = 0
        for (j <- jvec)
            jvec(k) = 0
            k += 1
        end for

        var res : Boolean = false
        res = m.traverseJolts(jvec, 0, 0) // 0 initial value, 0 index, 0 presses

        var bestJolt : Int = m.getBestJolt()
        println("BestJolt = " + bestJolt)

        totPressesJolt = totPressesJolt + bestJolt
      end if





      /////////////////////////////
      // simple solve
      val runSimplex : Boolean = false
      if (runSimplex) then
        m.clear()

        // joltage requirements
        var s3 : String = rr(2).substring(0, rr(2).length - 1).strip()
        var jj : Array[String] = s3.split(raw",")
        for (j <- jj)
            if (j.length > 0) then
                //println("    addSimplexJolt = " + j)
                m.addJolt( j.toInt )
      
        totPressesSimplex = totPressesSimplex + m.simplexSolve(s1.length)
      end if




      ///////////////////////////
      //m.linSystem()


      
      
      //println("READ: s1='" + s1 + "' (" + i1 + ") ----->  s2='"  + s2 + "'  ----->  s3='" + s3 + "'")
      //println()

  end while


  //println()
  //for (tile <- tiles)
  //    println("tile= " + tile.getX() + " " + tile.getY())


  println("TotPresses = " + totPresses)
  println("TotPressesJolt = " + totPressesJolt)
  println("TotPressesSimplex = " + totPressesSimplex)


  // 16725 - too low.
  // 16739 - too low.


  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp

@main def Main: Unit =
  println("AOC: Day 10")
  val app = new AocApp()
