import java.util.*;
import java.io.*;

public class Day6 {
    public static void main(String[] args) {
        int test = Integer.parseInt(args[0]);
        try {
            File file = new File("2015/inputs/input6.txt");
            Scanner fileScanner = new Scanner(file);
            if (test == 1) {System.out.println(part1(fileScanner));}
            else if (test == 2) {System.out.println(part2(fileScanner));}
        } catch (FileNotFoundException ex) {}
    }

    public static int part1(Scanner fileScanner) { 
        boolean[][] grid = new boolean[1000][1000];
        for (int i = 0; i < grid.length; i++) {Arrays.fill(grid[i], false);} 

        int count = 0;
        while (fileScanner.hasNextLine()) {
            String[] line = fileScanner.nextLine().split(" ");
            
            int offset = 0;
            String function = line[0];
            if (line[1].equals("on") || line[1].equals("off")) {
                offset = 1;
                function = line[0] + " " + line[1];
            }

            String[] startPos = line[1 + offset].split(",");
            int startXPos = Integer.parseInt(startPos[0]);
            int startYPos = Integer.parseInt(startPos[1]);
            String[] endPos = line[3 + offset].split(",");
            int endXPos = Integer.parseInt(endPos[0]);
            int endYPos = Integer.parseInt(endPos[1]);

            for (int i = startYPos; i <= endYPos; i++) {
                for (int j = startXPos; j <= endXPos; j++) {
                    if (function.equals("turn on")) {grid[i][j] = true;} 
                    else if (function.equals("toggle")) {grid[i][j] = !grid[i][j];} 
                    else if (function.equals("turn off")) {grid[i][j] = false;}
                }
            }

        }

        for (int i = 0; i < grid.length; i++) {
            for (int j = 0; j < grid[0].length; j++) {
                if (grid[i][j]) {count++;}
            }
        } 
        return count;
    }

    public static int part2(Scanner fileScanner) {
        int[][] grid = new int[1000][1000];
        for (int i = 0; i < grid.length; i++) {Arrays.fill(grid[i], 0);} 

        int brightness = 0;
        while (fileScanner.hasNextLine()) {
            String[] line = fileScanner.nextLine().split(" ");
            
            int offset = 0;
            String function = line[0];
            if (line[1].equals("on") || line[1].equals("off")) {
                offset = 1;
                function = line[0] + " " + line[1];
            }

            String[] startPos = line[1 + offset].split(",");
            int startXPos = Integer.parseInt(startPos[0]);
            int startYPos = Integer.parseInt(startPos[1]);
            String[] endPos = line[3 + offset].split(",");
            int endXPos = Integer.parseInt(endPos[0]);
            int endYPos = Integer.parseInt(endPos[1]);

            for (int i = startYPos; i <= endYPos; i++) {
                for (int j = startXPos; j <= endXPos; j++) {
                    if (function.equals("turn on")) {grid[i][j] += 1;} 
                    else if (function.equals("toggle")) {grid[i][j] += 2;} 
                    else if (function.equals("turn off")) {if (grid[i][j] > 0) {grid[i][j] -= 1;}}
                }
            }

        }

        for (int i = 0; i < grid.length; i++) {
            for (int j = 0; j < grid[0].length; j++) {
                brightness += grid[i][j];
            }
        } 
        return brightness;
    }
}
