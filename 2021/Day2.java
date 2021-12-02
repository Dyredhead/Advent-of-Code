import java.util.*;
import java.io.*;

public class Day2 {
    public static void main(String[] args) {
        int test = Integer.parseInt(args[0]);
        File file = new File("inputs/input2.txt");
        try {
            Scanner fileScanner = new Scanner(file);
            if (test == 1) {System.out.println(part1(fileScanner));}
            else if (test == 2) {System.out.println(part2(fileScanner));}
        } catch (FileNotFoundException ex) {}
    }

    public static int part1(Scanner fileScanner) {
        int horizontalPosition = 0;
        int verticalPosition = 0;

        while (fileScanner.hasNextLine()) {
            String[] line = fileScanner.nextLine().split(" ");
            String direction = line[0];
            int distance = Integer.parseInt(line[1]);
            if (direction.equals("forward")) {horizontalPosition += distance;}
            else if (direction.equals("up")) {verticalPosition -= distance;}
            else if (direction.equals("down")) {verticalPosition += distance;}
        }
        return horizontalPosition * verticalPosition;
    }

    public static int part2(Scanner fileScanner) {
        int horizontalPosition = 0;
        int aim = 0;
        int verticalPosition = 0;

        while (fileScanner.hasNextLine()) {
            String[] line = fileScanner.nextLine().split(" ");
            String direction = line[0];
            int distance = Integer.parseInt(line[1]);
            if (direction.equals("forward")) {horizontalPosition += distance; verticalPosition += distance * aim;}
            else if (direction.equals("up")) {aim -= distance;}
            else if (direction.equals("down")) {aim += distance;}
        }
        return horizontalPosition * verticalPosition;
    }
}
