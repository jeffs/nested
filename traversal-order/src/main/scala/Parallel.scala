object Parallel {

  def main(args: Array[String]): Unit = {

    val d = Math.ulp(1.0) / 2 // Half epsilon.

// A nice, orderly sequence of numbers.
    val numbers = List.fill(10000)(d) :+ 1.0

// Summing the series in order gives a consistent result.
    val sums = 1 to 1000 map (_ => numbers.sum)
    println(sums.distinct) // Vector(1.0000000000011102)

// ...But summing in parallel produces two different results.
    val parSums = 1 to 1000 map (_ => numbers.par.sum)
    println(parSums.distinct)
    // Vector(1.0000000000011102, 1.00000000000111)

  }

}
