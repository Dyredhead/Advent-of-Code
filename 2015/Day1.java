import java.util.*;
import java.io.*;

public class Day1 {
    public static void main(String args[]) {
        int test = Integer.parseInt(args[0]);
        File file = new File("inputs/input1.txt");
        try {
            Scanner fileScanner = new Scanner(file);
            if (test == 1) {System.out.println(whatFloor(fileScanner));}
            else if (test == 2) {System.out.println(basement(fileScanner));}
        } catch (FileNotFoundException ex) {}
    }

    public static int whatFloor(Scanner fileScanner) {
        String directions = fileScanner.nextLine();
        int floor = 0;
        for (int i = 0; i < directions.length(); i++) {
            if (directions.charAt(i) == '(') {
                floor++;
            } else if (directions.charAt(i) == ')') {
                floor--;
            }
        }
        fileScanner.close();
        return floor;
    }

    public static int basement(Scanner fileScanner) {
        String directions = fileScanner.nextLine();
        int floor = 0;
        int position = 0;
        while (floor != -1) {
            if (directions.charAt(position) == '(') {floor++;} 
            else if (directions.charAt(position) == ')') {floor--;}

            if (floor == -1) {break;}
            else {position++;}
        }
        fileScanner.close();
        return position+1;
    }
}
