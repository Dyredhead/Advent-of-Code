import java.util.*;
import java.io.*;

public class Day7 {
    public static void main(String[] args) {
        int test = Integer.parseInt(args[0]);
        try {
            File file = new File("inputs/input7.txt");
            Scanner fileScanner1 = new Scanner(file);
            if (test == 1) {System.out.println(valueOfA(fileScanner1, file));}
            else if (test == 2) {System.out.println(xyz(fileScanner1));}
        } catch (FileNotFoundException ex) {
            ex.printStackTrace();
        }
    }

    public static int valueOfA(Scanner fileScanner1, File file) {
        
        try {
            while (fileScanner1.hasNextLine()) {
                ArrayList<String> line1 = new ArrayList<String>(Arrays.asList(fileScanner1.nextLine().split(" ")));
                // does all operation except for assignment
                if (line1.get(0).equals("NOT")) {
                    try {
                        int number = Integer.parseInt(line1.get(1));
                        String NOTnum = Integer.toString(~number);  
                        line1.set(1, NOTnum);
                        line1.remove(0);
                    } catch (NumberFormatException ex) {}
                } else if (line1.get(1).equals("RSHIFT")) {
                    try {
                        int RSHIFTnum1 = Integer.parseInt(line1.get(0));  
                        int RSHIFTnum2 = Integer.parseInt(line1.get(2));
                        int RSHIFT1To2 = RSHIFTnum1 >> RSHIFTnum2;   
                        line1.set(0, Integer.toString(RSHIFT1To2));
                        line1.remove(1); line1.remove(1);
                    } catch (NumberFormatException ex) {}
                } else if (line1.get(1).equals("LSHIFT")) {
                    try {
                        int LSHIFTnum1 = Integer.parseInt(line1.get(0));  
                        int LSHIFTnum2 = Integer.parseInt(line1.get(2));
                        int LSHIFT1To2 = LSHIFTnum1 << LSHIFTnum2;   
                        line1.set(1, Integer.toString(LSHIFT1To2));
                        line1.remove(1); line1.remove(1);
                    } catch (NumberFormatException ex) {}
                } else if (line1.get(1).equals("AND")) {
                    try {
                        int ANDnum1 = Integer.parseInt(line1.get(0));  
                        int ANDnum2 = Integer.parseInt(line1.get(2));
                        int AND1To2 = ANDnum1 & ANDnum2;   
                        line1.set(0, Integer.toString(AND1To2));
                        line1.remove(1); line1.remove(1);
                    } catch (NumberFormatException ex) {}
                } else if (line1.get(1).equals("OR")) {
                    try {
                        int ORnum1 = Integer.parseInt(line1.get(0));  
                        int ORnum2 = Integer.parseInt(line1.get(2));
                        int OR1To2 = ORnum1 | ORnum2;   
                        line1.set(0, Integer.toString(OR1To2));
                        line1.remove(1); line1.remove(1);
                    } catch (NumberFormatException ex) {}
                } 

                String val = line1.get(0);
                String var = line1.get(2);

                Scanner fileScanner2 = new Scanner(file);
                while (fileScanner2.hasNextLine()) {
                    ArrayList<String> line2 = new ArrayList<String>(Arrays.asList(fileScanner2.nextLine().split(" ")));
                    if (line2.indexOf(var) != -1) {
                        line2.set(line2.indexOf(var), val);
                    }
                } 
                System.out.println(line1.toString());

            }
            fileScanner1.close();
        } catch (FileNotFoundException ex) {}
        return a;
    }

    public static int xyz(Scanner filScanner) {
        int a = 0;
        return a;
    }
}
