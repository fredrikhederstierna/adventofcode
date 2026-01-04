
import java.util.StringTokenizer;
import java.io.BufferedReader;
import java.io.InputStreamReader;

import java.util.stream.Collectors;

import scala.collection.mutable.ListBuffer
import scala.collection.mutable.Map

import scala.util.control.Breaks._


// rack ---> reactor

//
// devices and their outputs
//
//  'you' ---> 'out'   ----every path. N paths
//
//  DAG - Directed Asynclic? Graph
//    ------>
// NAME: OUTPUTS
//
// aaa: you hhh
// you: bbb ccc
// bbb: ddd eee
// ccc: ddd eee fff
// ddd: ggg
// eee: out
// fff: out
// ggg: out
// hhh: ccc fff iii
// iii: out
//


class Node(name : String) :

  var nodeName = name

  var visited : Boolean = false
  
  val memo : Map[Node, Long] = Map()   // store dyn prog... key,value

  var edges : ListBuffer[Node] = new ListBuffer[Node]()

  
  def addEdge(node : Node) : Unit =
      edges.append( node )

  def getName() : String =
      return this.nodeName

  def setVisited(visited : Boolean) : Unit =
      this.visited = visited

  def clearDynamic() : Unit =
      // clear memo map
      this.memo.clear()

  def walk(dest : Node) : Int =

      // if reached dest node
      if (this == dest)
          return 1

      setVisited(true)

      var n_walks : Int = 0

      for (edgeNode <- this.edges)
          if (edgeNode.visited == false) then
              var n : Int = edgeNode.walk( dest )
              n_walks = n_walks + n
          end if
      end for

      setVisited(false)

      return n_walks

  def walk2(dest : Node, dacNode : Node, fftNode : Node, dacVisited : Boolean, fftVisited : Boolean) : Long =

      //println("walk2: " + this.nodeName + " dac " + dac + " fft " + fft)
      
      // if reached dest node, do not walk further from the end point
      if (this == dest) then
          //if ((dacVisited == true) && (fftVisited == true)) then
          //    return 1
          //else
          //    return 0
          // TEST time for dyn prog.
          return 1

        
      setVisited(true)

      
      var n_walks : Long = 0
      var n : Long = 0

      for (edgeNode <- this.edges)
          if (edgeNode.visited == false) then

              // check dynamic prog table
              val containsKey = memo.contains( edgeNode )
              if (containsKey) then
                  n = memo.getOrElse( edgeNode, 0 )
                  //println("  get " + this.name + " -> " + edgeNode.getName() + " = " + n)
              else
                  n = edgeNode.walk2( dest, dacNode, fftNode, dacVisited || (this == dacNode), fftVisited || (this == fftNode) )
                  // add to dynamic programming hashmap
                  memo.put( edgeNode, n )
                  //println("  put " + this.name + " -> " + edgeNode.getName() + " = " + n)
              
              n_walks = n_walks + n

          end if
      end for

      setVisited(false)

      return n_walks

end Node


class Graph() :

  var graph : ListBuffer[Node] = new ListBuffer[Node]()

  def getNodeByName(name : String) : Node =
      var foundNode : Node = null
      breakable {
      for (node <- graph)
          if (node.getName() == name) then
              foundNode = node
              break
      }
      return foundNode

  def addNode(node : Node) : Unit =
      graph.append( node )

  def addEdge(nodeSource : Node, nodeDest : Node) : Unit =
      breakable {
      for (node <- graph)
          if (node == nodeSource) then
              node.addEdge(nodeDest)
              break
      }

  def clearDynamic() : Unit =
      for (node <- graph)
          node.clearDynamic()
      
end Graph


class AocApp extends App :
  println("Run...")
  println()

  
  // stdin
  val input = getInput()
  //println(input)

  var g : Graph = new Graph()

  var tokens = new StringTokenizer(input, "\n", false);
  while tokens.hasMoreTokens() do
      var token = tokens.nextToken().strip()
      var slen : Integer = token.length()
      println("token: " + token)

      var rr : Array[String] = token.split(':')

      val name : String = rr(0).strip()
      val edges : Array[String] = rr(1).strip().split(' ')


      // Add source node
      if (g.getNodeByName( name ) == null) then
          // add source node
          var n : Node = new Node( name )
          g.addNode( n )
          println("  SOURCE addNode(" + name + ")")
      end if
      
      // Add dest nodes
      for (e_name <- edges)
          if (g.getNodeByName( e_name ) == null) then
              // add dest node
              var e_n : Node = new Node( e_name )
              g.addNode( e_n )
              println("  DEST : addNode(" + e_name + ")")
          end if

      // Add edges
      for (e_name <- edges)
          // add dest node
          var source : Node = g.getNodeByName( name )
          var dest   : Node = g.getNodeByName( e_name )
          g.addEdge( source, dest )
          println("  EDGE : addEdge(" + name + " ---> " + e_name + ")")
          
      println("READ: NAME = " + rr(0) + " ----->  EDGES = " + rr(1))
      println()
      
  end while

  println("Start walking...")
  println()

  var youNode : Node = g.getNodeByName( "you" )
  var outNode : Node = g.getNodeByName( "out" )

  // part1
  /*
  var n : Int = youNode.walk( outNode )
  println("Walks: " + n)
  */

  // part2
  var svrNode : Node = g.getNodeByName( "svr" )
  var fftNode : Node = g.getNodeByName( "fft" )
  var dacNode : Node = g.getNodeByName( "dac" )
  /*
  var n2 : Long = svrNode.walk2( outNode, dacNode, fftNode, false, false )
  println("Walks2: " + n2)
  */
  //
  var n_svr_fft : Long = svrNode.walk2( fftNode, dacNode, fftNode, false, false )
  println("Walks_svr_fft: " + n_svr_fft)
  g.clearDynamic()
  //
  var n_svr_dac : Long = svrNode.walk2( dacNode, dacNode, fftNode, false, false)
  println("Walks_svr_dac: " + n_svr_dac)
  g.clearDynamic()
  //
  var n_fft_out : Long = fftNode.walk2( outNode, dacNode, fftNode, false, false )
  println("Walks_fft_out: " + n_fft_out)
  g.clearDynamic()
  //
  var n_dac_out : Long = dacNode.walk2( outNode, dacNode, fftNode, false, false )
  println("Walks_dac_out: " + n_dac_out)
  g.clearDynamic()
  //  
  var n_fft_dac : Long = fftNode.walk2( dacNode, dacNode, fftNode, false, false )
  println("Walks_fft_dac: " + n_fft_dac)
  g.clearDynamic()
  //  
  var n_dac_fft : Long = dacNode.walk2( fftNode, dacNode, fftNode, false, false )
  println("Walks_dac_fft: " + n_dac_fft)
  g.clearDynamic()
  //  
  var n_svr_out : Long = svrNode.walk2( outNode, dacNode, fftNode, false, false )
  println("Walks_svr_out: " + n_svr_out)
  g.clearDynamic()

  println("sum2 = " + (n_svr_fft * n_fft_dac * n_dac_out))

  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 11")
  val app = new AocApp()
