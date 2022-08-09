object Main {
  def main(args: Array[String]): Unit = {
    val d = Math.ulp(1.0) / 2
    val numbers = List.fill(10000)(d) :+ 1.0
    val sums = 1 to 1000 map (_ => numbers.sum)
    val parSums = 1 to 1000 map (_ => numbers.par.sum)
    println(sums.distinct) // Vector(1.0000000000011102)
    println(parSums.distinct) // Vector(1.0000000000011102, 1.00000000000111)
  }
}
