import java.util.*;
import java.io.*;

public class Day8 {
    public static void main(String[] args) {
        int test = Integer.parseInt(args[0]);
        try {
            File file = new File("inputs/input8.txt");
            Scanner fileScanner = new Scanner(file);
            if (test == 1) {System.out.println(part1(fileScanner));}
            else if (test == 2) {System.out.println(part2(fileScanner));}
        } catch (FileNotFoundException ex) {}
    }
    
    public static int part1(Scanner fileScanner) {
        int literal = 0;
        int memory = 0;
        while (fileScanner.hasNextLine()) {
            String line = fileScanner.nextLine();
            literal += line.length();
            for (int i = 0; i < line.length(); i++) { 
                boolean isEscapeSequence = line.charAt(i) == '\\';
                if (line.charAt(i) == '\"') {i++;} 
                else if (isEscapeSequence) {
                    memory++;
                    char escapeSequence = line.charAt(i+1);
                    if (escapeSequence == '\"' || escapeSequence == '\\') {i++; } 
                    else if (escapeSequence == 'x') {i += 3;}
                } else { memory++;}
            }
        }
        return literal - memory;
    }

    public static int part2(Scanner fileScanner) {
        return 0;
    }
}