import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class Day2 {
    public static void main(String[] args) {
        int test = Integer.parseInt(args[0]);
        File file = new File("2015/inputs/input2.txt");
        try {
            Scanner fileScanner = new Scanner(file);
            if (test == 1) {System.out.println(part1(fileScanner));}
            else if (test == 2) {System.out.println(part2(fileScanner));}
        } catch (FileNotFoundException ex) {}
    }

    public static int part1(Scanner fileScanner) {
        int total = 0;
        while (fileScanner.hasNextLine()) {
            String[] lineStr = fileScanner.nextLine().split("x");
            int l = Integer.parseInt(lineStr[0]);
            int w = Integer.parseInt(lineStr[1]);
            int h = Integer.parseInt(lineStr[2]);
            int SA = 2*l*w + 2*w*h + 2*h*l;
            int LEP = Math.min(Math.min(l*w, w*h), h*l);
            total += SA + LEP;
        }
        fileScanner.close();
        return total;
    }

    public static int part2(Scanner fileScanner) {
        int total = 0;
        while (fileScanner.hasNextLine()) {
            String[] lineStr = fileScanner.nextLine().split("x");
            int l = Integer.parseInt(lineStr[0]);
            int w = Integer.parseInt(lineStr[1]);
            int h = Integer.parseInt(lineStr[2]);
            int LWperimiter = 2*l + 2*w;
            int WHperimiter = 2*w + 2*h;
            int HLperimiter = 2*h + 2*l;
            int SP = Math.min(Math.min(LWperimiter, WHperimiter), HLperimiter);
            int V = l*w*h;
            total += SP + V;
        }
        fileScanner.close();
        return total;
    }
}
