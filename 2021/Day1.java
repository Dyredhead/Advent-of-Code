import java.util.*;
import java.io.*;

public class Day1 {
    public static void main(String[] args) {
        int test = Integer.parseInt(args[0]);
        File file = new File("inputs/input1.txt");
        try {
            Scanner fileScanner = new Scanner(file);
            if (test == 1) {System.out.println(numberOfIncreases(fileScanner));}
            else if (test == 2) {System.out.println(numberOfTripletIncreases(fileScanner));}
        } catch (FileNotFoundException ex) {}
    }
    
    public static int numberOfIncreases(Scanner fileScanner) {
        ArrayList<Integer> depths = new ArrayList<Integer>();
        while (fileScanner.hasNextInt()) {
            depths.add(fileScanner.nextInt());
        }
        int increases = 0;
        for (int i = 0; i < depths.size()-1; i++) {
            int depth1 = depths.get(i);
            int depth2 = depths.get(i+1);
            if (depth2 > depth1) {increases++;}
        }
        fileScanner.close();
        return increases;
    }

    public static int numberOfTripletIncreases(Scanner fileScanner) {
        ArrayList<Integer> depths = new ArrayList<Integer>();
        while (fileScanner.hasNextInt()) {
            depths.add(fileScanner.nextInt());
        }
        int tripletIncreases = 0;
        for (int i = 0; i < depths.size()-3; i++) { 
            int depth1 = depths.get(i);
            int depth2 = depths.get(i+1);
            int depth3 = depths.get(i+2);
            int depth4 = depths.get(i+3);

            int window1 = depth1 + depth2 + depth3;
            int window2 = depth2 + depth3 + depth4;
            if (window2 > window1) {tripletIncreases++;}
        }
        fileScanner.close();
        return tripletIncreases;
    }
}