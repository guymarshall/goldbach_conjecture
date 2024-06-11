import java.util.stream.IntStream;

public class Main {
    public static boolean isPrime(int number) {
        if (number <= 1) {
            return false;
        }
        if (number == 2) {
            return true;
        }
        if (number % 2 == 0) {
            return false;
        }

        for (int i = 3; i <= Math.sqrt(number); i += 2) {
            if (number % i == 0) {
                return false;
            }
        }

        return true;
    }

    public static boolean primesAddUpToNumber(int numberToCheck) {
        if (isPrime(numberToCheck - 2)) {
            return true;
        }
        return IntStream.iterate(3, number -> number + 2)
            .limit((numberToCheck - 3) / 2)
            .filter(Main::isPrime)
            .anyMatch(number -> isPrime(numberToCheck - number));
    }

    public static void main(String[] args) {
        long startTime = System.nanoTime();

        final int LIMIT = 1_000_000_000;

        IntStream.iterate(4, number -> number + 2)
            .limit((LIMIT - 4) / 2)
            .forEach(numberToCheck -> {
                if (numberToCheck % 1000 == 0) {
                    System.out.printf("%.4f%%%n", (numberToCheck / (double) LIMIT) * 100.0);
                }
                if (!primesAddUpToNumber(numberToCheck)) {
                    System.out.printf("No two primes add to make %d%n", numberToCheck);
                    System.exit(0);
                }
            });

        long endTime = System.nanoTime();
        System.out.printf("Time: %dms%n", (endTime - startTime) / 1000000);
    }
}