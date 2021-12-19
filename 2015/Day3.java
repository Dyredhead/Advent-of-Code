import java.util.*;
import java.io.*;

public class Day3 {
    public static void main(String[] args) {
        int test = Integer.parseInt(args[0]);
        File file = new File("2015/inputs/input3.txt");
        try {
            Scanner fileScanner = new Scanner(file);
            if (test == 1) {System.out.println(part1(fileScanner));} 
            else if (test == 2) {System.out.println(part2(fileScanner));}  
        } catch (FileNotFoundException ex) {}
    }

    public static int part1(Scanner fileScanner) {
        int count = 1; 
        String directions = fileScanner.nextLine();

        int countUP = 0, countDOWN = 0, countRIGHT = 0, countLEFT = 0;
        for (int i = 0; i < directions.length(); i++) {
            if (directions.charAt(i) == '^') {countUP++;} 
            else if (directions.charAt(i) == 'v') {countDOWN++;}
            else if (directions.charAt(i) == '>') {countRIGHT++;}
            else if (directions.charAt(i) == '<') {countLEFT++;}
        }

        boolean[][] houses = new boolean[countUP + countDOWN + 1][countRIGHT + countLEFT + 1];
        for (int i = 0; i < countUP + countDOWN + 1; i++) {Arrays.fill(houses[i], false);}

        int[] location = {countUP, countLEFT};
        houses[location[0]][location[1]] = true;

        for (int i = 0; i < directions.length(); i++) {
            if (directions.charAt(i) == '^') {location[0] -= 1;} 
            else if (directions.charAt(i) == 'v') {location[0] += 1;} 
            else if (directions.charAt(i) == '>') {location[1] += 1;} 
            else if (directions.charAt(i) == '<') {location[1] -= 1;}

            if (houses[location[0]][location[1]] == false) {
                houses[location[0]][location[1]] = true;
                count++;
            }
        }

        fileScanner.close();
        return count;
    }

    public static int part2(Scanner fileScanner) {
        int count = 1;
        String directions = fileScanner.nextLine();

        int countUP = 0, countDOWN = 0, countRIGHT = 0, countLEFT = 0;
        for (int i = 0; i < directions.length(); i++) {
            if (directions.charAt(i) == '^') {countUP++;} 
            else if (directions.charAt(i) == 'v') {countDOWN++;}
            else if (directions.charAt(i) == '>') {countRIGHT++;}
            else if (directions.charAt(i) == '<') {countLEFT++;}
        }

        boolean[][] houses = new boolean[countUP + countDOWN + 1][countRIGHT + countLEFT + 1];
        for (int i = 0; i < countUP + countDOWN + 1; i++) {Arrays.fill(houses[i], false);}

        int[] location = {countUP, countLEFT};
        int[] santaLocation = location.clone();
        int[] roboLocation = location.clone();
        houses[location[0]][location[1]] = true;
        
        for (int i = 0; i < directions.length(); i++) {    
            if (i%2 == 0) {location = santaLocation;} 
            else if (i%2 == 1){location = roboLocation;}

            if (directions.charAt(i) == '^') {location[0] -= 1;} 
            else if (directions.charAt(i) == 'v') {location[0] += 1;} 
            else if (directions.charAt(i) == '>') {location[1] += 1;} 
            else if (directions.charAt(i) == '<') {location[1] -= 1;}

            if (houses[location[0]][location[1]] == false) {
                houses[location[0]][location[1]] = true;
                count++;
            }
        }
        fileScanner.close();
        return count;
    }
}