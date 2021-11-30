import java.util.*;
import java.io.*;

public class Day5 {
    public static void main(String[] args) {
        int test = Integer.parseInt(args[0]);
        File file = new File("inputs/input5.txt");
        try {
            Scanner fileScanner = new Scanner(file);
            if (test == 1) {System.out.println(niceStrings(fileScanner));}
            else if (test == 2) {System.out.println(nicerStrings(fileScanner));}
        } catch (FileNotFoundException ex) {}   
    }

    public static int niceStrings(Scanner fileScanner) {
        int count = 0;
        while (fileScanner.hasNextLine()) {
            String line = fileScanner.nextLine();

            boolean threeVowels = false;
            char[] vowels = {'a', 'e', 'i', 'o', 'u'};
            int vowelCount = 0;
            for (int i = 0; i < line.length(); i++) {
                for (int j = 0; j < vowels.length; j++) {
                    if (line.charAt(i) == vowels[j]) {vowelCount++; break;}
                }
                if (vowelCount >= 3) {threeVowels = true; break;}
            }

            boolean pairOfCharacters = false;
            for (int i = 0; i < line.length()-1; i++) {
                if (line.charAt(i) == line.charAt(i+1)) {pairOfCharacters = true; break;}
            }

            boolean doesNotContainStrings = true;
            String[] strings = {"ab", "cd", "pq", "xy"}; 
            for (int i = 0; i < strings.length; i++) {
                if (line.indexOf(strings[i]) > -1) {doesNotContainStrings = false; break;}
            }

            if (threeVowels && pairOfCharacters && doesNotContainStrings) {count++;}
        }

        return count;
    }

    public static int nicerStrings(Scanner fileScanner) {
        int count = 0;
        while (fileScanner.hasNextLine()) {
            String line = fileScanner.nextLine();

            boolean twoPair = false;
            for (int i = 0; i < line.length()-1; i++) {
                String pair = line.substring(i, i+2);
                if (line.indexOf(pair, i+2) != -1) {
                    twoPair = true;
                    break;
                } 
            }

            boolean splitPair = false;
            for (int i = 0; i < line.length()-2; i++) {
                if (line.charAt(i) == line.charAt(i+2)) {splitPair = true; break;}
            }

            if (twoPair && splitPair) {count++;}
        }

        return count;
    } 
}