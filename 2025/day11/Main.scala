
import java.util.StringTokenizer;
import java.io.BufferedReader;
import java.io.InputStreamReader;

import java.util.stream.Collectors;

import scala.collection.mutable.ListBuffer

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

  var edges : ListBuffer[Node] = new ListBuffer[Node]()

  def addEdge(node : Node) : Unit =
      edges.append( node )

  def getName() : String =
      return this.nodeName

  def setVisited(visited : Boolean) : Unit =
      this.visited = visited

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

  var youNode : Node = g.getNodeByName( "you" )
  var outNode : Node = g.getNodeByName( "out" )

  var n : Int = youNode.walk( outNode )

  println("Walks: " + n)


  def getInput() : String =
    var reader = new BufferedReader(new InputStreamReader(System.in))
    var lines = reader.lines().collect(Collectors.joining("\n"))
    return lines
  
end AocApp


@main def Main: Unit =
  println("AOC: Day 11")
  val app = new AocApp()
