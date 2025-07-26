import java.util.stream.IntStream;

public class Main {
    public static void main(String[] args) {
        final int LIMIT = 1_000_000_000;

        IntStream.iterate(4, i -> i + 2)
                .limit((LIMIT - 4L) / 2)
                .forEach(numberToCheck -> {
                    if (numberToCheck % 10000 == 0) {
                        System.out.printf("\r%.4f%%", (numberToCheck * 100.0) / LIMIT);
                    }

                    if (!Primes.primesAddUpToNumber(numberToCheck)) {
                        System.out.println("\nNo two primes add to make " + numberToCheck);
                        System.exit(0);
                    }
                });

        System.out.println();
    }
}