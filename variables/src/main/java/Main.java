import java.util.ArrayList;
import java.util.List;

class Main {
	public static void main(String[] args) {
		var i = 42;
		var j = i;
		j += 1;
		assert i != j;		// OK
		assert i == 42;		// OK

		var xs = new ArrayList<>(List.of(42));
		var ys = xs;
		ys.add(1);
		assert xs != ys;	// FAIL
		assert xs.equals(new ArrayList<>(List.of(42)));
							// LOL, GFY! —❤️ , Java
	}
}
