object Main {
  def main(args: Array[String]) = {
    lazy val x = {
      println("x initializer");
      1;
    }
    val y = 2;
    println(y);
    println(x);
  }
}
