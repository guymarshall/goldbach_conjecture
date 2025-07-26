import java.util.stream.IntStream;

public class Primes {
    private static boolean isPrime(int number) {
        if (number == 2 || number == 3) {
            return true;
        }

        if (number <= 1 || number % 2 == 0 || number % 3 == 0) {
            return false;
        }

        for (int i = 5; i * i <= number; i += 6) {
            if (number % i == 0 || number % (i + 2) == 0) {
                return false;
            }
        }

        return true;
    }

    public static boolean primesAddUpToNumber(int numberToCheck) {
        if (isPrime(numberToCheck - 2)) {
            return true;
        }

        return IntStream.iterate(3, i -> i + 2)
                .limit((numberToCheck - 3) / 2)
                .filter(Primes::isPrime)
                .anyMatch(i -> isPrime(numberToCheck - i));
    }
}
