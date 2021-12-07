import java.util.*;
import java.io.*;

public class Day6 {
    public static void main(String[] args) {
        int test = Integer.parseInt(args[0]);
        //int test = 1;
        File file = new File("inputs/input6.txt");
        try {
            Scanner fileScanner = new Scanner(file);
            if (test == 1) {System.out.println(part1(fileScanner));}
            else if (test == 2) {System.out.println(part2(fileScanner));}
        } catch (FileNotFoundException ex) {}
    }

    public static int part1(Scanner fileScanner) {
        ArrayList<Integer> fishies = new ArrayList<Integer>();
        while (fileScanner.hasNextLine()) {
            String[] line = fileScanner.nextLine().split(",");
            for (int i = 0; i < line.length; i++) {
                fishies.add(Integer.parseInt(line[i]));
            }
        }

        for (int day = 1; day <= 80; day++) {
            int size = fishies.size();
            for (int f = 0; f < size; f++) {
                if (fishies.get(f) == 0) {
                    fishies.set(f, 6);
                    fishies.add(8);
                } else {
                    fishies.set(f, fishies.get(f)-1);
                }
            }
        }
        return fishies.size();
    }

    public static int part2(Scanner fileScanner) {
        int counter = 0;
        for (int i = 0; i < 100_000_000; i++) {
            counter++;
        }
        return counter;
    }
}
