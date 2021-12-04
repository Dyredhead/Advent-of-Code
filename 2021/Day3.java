import java.util.*;
import java.io.*;

public class Day3 {
    public static void main(String[] args) {
        int test = Integer.parseInt(args[0]);
        File file = new File("inputs/input3.txt");
        try {
            Scanner fileScanner = new Scanner(file);
            if (test == 1) {System.out.println(part1(fileScanner));}
            else if (test == 2) {System.out.println(part2(fileScanner));}
        } catch (FileNotFoundException ex) {}
    }

    public static int part1(Scanner fileScanner) {
        ArrayList<String> energyReport = new ArrayList<String>();
        while (fileScanner.hasNextLine()) {
            String line = fileScanner.nextLine();
            energyReport.add(line);
        }
        String gammaBinary = ""; String epsilonBinary = "";
        for (int i = 0; i < energyReport.get(0).length(); i++) {
            gammaBinary += bitCounterInColoumn(energyReport, i, "gamma");
            epsilonBinary += bitCounterInColoumn(energyReport, i, "epsilon");
        }
        int gamma = Integer.parseInt(gammaBinary, 2); int epsilon = Integer.parseInt(epsilonBinary, 2);
        int powerConsumption = gamma * epsilon;
        return powerConsumption;
    }

    public static int part2(Scanner fileScanner) {
        ArrayList<String> report = new ArrayList<String>();
        while (fileScanner.hasNextLine()) {
            String line = fileScanner.nextLine();
            report.add(line);
        }

        ArrayList<String> O2report = new ArrayList<String>(report);
        String O2RatingBinary = "";
        for (int i = 0; i < report.get(0).length(); i++) {
            char bit = bitCounterInColoumn(O2report, i, "O2");
            for (int j = O2report.size()-1; j >= 0 ; j--) {
                String line = O2report.get(j);
                if (line.charAt(i) != bit && O2report.size() != 1) {O2report.remove(j);}
                if (O2report.size() == 1) {O2RatingBinary = line; break;}
            }
            if (!O2RatingBinary.equals("")) {break;}
        }

        ArrayList<String> CO2report = new ArrayList<String>(report);
        String CO2RatingBinary = "";
        for (int i = 0; i < report.get(0).length(); i++) {
            char bit = bitCounterInColoumn(CO2report, i, "CO2");
            for (int j = CO2report.size()-1; j >= 0 ; j--) {
                String line = CO2report.get(j);
                if (line.charAt(i) != bit && O2report.size() != 1) {O2report.remove(j);}
                if (O2report.size() == 1) {CO2RatingBinary = line; break;}
            }
            if (!CO2RatingBinary.equals("")) {break;}
        }

        int O2Rating = Integer.parseInt(O2RatingBinary, 2); int CO2Rating = Integer.parseInt(CO2RatingBinary, 2);
        int lifeSupportRating = O2Rating * CO2Rating;
        return lifeSupportRating;

    }

    public static char bitCounterInColoumn(ArrayList<String> report, int column, String powerTest) {
        int countZero = 0; int countOne = 0;
        for (int i = 0; i < report.size(); i++) {
            if (report.get(i).charAt(column) == '0') {countZero++;}
            else if (report.get(i).charAt(column) == '1') {countOne++;}
        }
        if (powerTest.equals("gamma") || powerTest.equals("O2")) {
            if (countZero > countOne) {return '0';}
            else if (countOne > countZero) {return '1';}
        } else if (powerTest.equals("epsilon") || powerTest.equals("CO2")) {
            if (countZero < countOne) {return '0';}
            else if (countOne < countZero) {return '1';}
        }
        return '2';
    }




}
