import java.util.*;
import java.io.*;

public class Day7 {
    public static void main(String[] args) {
        int test = 1; //Integer.parseInt(args[0]);
        try {
            File file = new File("inputs/input7.txt"); 
            Scanner fileScannerVars = new Scanner(file);
            if (test == 1) {System.out.println(valueOfA(fileScannerVars, file));}
            else if (test == 2) {System.out.println(xyz(fileScannerVars));}
        } catch (FileNotFoundException ex) {
            ex.printStackTrace();
        }
    }

    public static int valueOfA(Scanner fileScannerVars, File file) { 
        ArrayList<ArrayList<String>> dictionary = new ArrayList<ArrayList<String>>(); ArrayList<String> line;
        dictionary.add(new ArrayList<String>()); dictionary.add(new ArrayList<String>()); 
        Collections.fill(dictionary.get(1), "null");
        
        
        while (fileScannerVars.hasNextLine()) {
            line = new ArrayList<String>(Arrays.asList(fileScannerVars.nextLine().split(" ")));
            for (int i = 0; i < line.size(); i++) {
                String element = line.get(i);
                if (!(element.equals("NOT") || element.equals("RSHIFT") || element.equals("LSHIFT") || 
                      element.equals("AND") || element.equals("OR") || element.equals("->"))) {
                    try {Integer.parseInt(element);}
                    catch (NumberFormatException ex) {
                        if (dictionary.get(0).indexOf(element) == -1) {
                            dictionary.get(0).add(element);
                        }
                    }
                } 
            }
        }

        

        Collections.sort(dictionary.get(0)); 
        System.out.println(dictionary.get(0).toString());
        Scanner fileScannerAsigm;
        try {
            fileScannerAsigm = new Scanner(file);
            // does initial assignments
            while (fileScannerAsigm.hasNextLine()) {
                line = new ArrayList<String>(Arrays.asList(fileScannerAsigm.nextLine().split(" ")));
                if (line.get(1).equals("->")) {
                    String var = line.get(2);
                    int dictIndex = dictionary.get(0).indexOf(var);
                    String num = line.get(0);
                    dictionary.get(1).set(dictIndex, num);
                }
            }
        } catch (FileNotFoundException ex) {}

        while (dictionary.get(1).get(0).equals("null")) {
            try {
                fileScannerAsigm = new Scanner(file);
                while (fileScannerAsigm.hasNextLine()) {
                    line = new ArrayList<String>(Arrays.asList(fileScannerAsigm.nextLine().split(" ")));
                    String var = line.get(line.size()-1);
                    if (isVarDefined(dictionary, var)) {
                        if (line.get(0).equals("NOT")) {
                            int NOTnum;
                            try {
                                NOTnum = ~Integer.parseInt(line.get(1));
                                dictionary.get(1).set(dictionary.get(0).indexOf(line.get(3)), String.valueOf(NOTnum));
                            } catch (NumberFormatException ex) {
                                if (!isVarDefined(dictionary, line.get(1))) {
                                    NOTnum = ~Integer.parseInt(varDef(dictionary, line.get(1)));
                                    dictionary.get(1).set(dictionary.get(0).indexOf(line.get(3)), String.valueOf(NOTnum));
                                }
                            }
                        } else if (line.get(1).equals("RSHIFT")) {
                            int RSHIFTnum1; int RSHIFTnum2;  
                            try {
                                RSHIFTnum1 = Integer.parseInt(line.get(0));
                            } catch (NumberFormatException ex) {
                                if (!isVarDefined(dictionary, var)) {
                                    RSHIFTnum1 = Integer.parseInt(varDef(dictionary, line.get(0)));
                            }
                            
                            
                            
                            try {
                                  
                                int RSHIFTnum2 = Integer.parseInt(line.get(2));
                                int RSHIFT1To2 = RSHIFTnum1 >> RSHIFTnum2;   
                                line.set(0, Integer.toString(RSHIFT1To2));
                                line.remove(1); line.remove(1);
                            } catch (NumberFormatException ex) {}
                        } else if (line.get(1).equals("LSHIFT")) {
                            try {
                                int LSHIFTnum1 = Integer.parseInt(line.get(0));  
                                int LSHIFTnum2 = Integer.parseInt(line.get(2));
                                int LSHIFT1To2 = LSHIFTnum1 << LSHIFTnum2;   
                                line.set(1, Integer.toString(LSHIFT1To2));
                                line.remove(1); line.remove(1);
                            } catch (NumberFormatException ex) {}
                        } else if (line.get(1).equals("AND")) {
                            try {
                                int ANDnum1 = Integer.parseInt(line.get(0));  
                                int ANDnum2 = Integer.parseInt(line.get(2));
                                int AND1To2 = ANDnum1 & ANDnum2;   
                                line.set(0, Integer.toString(AND1To2));
                                line.remove(1); line.remove(1);
                            } catch (NumberFormatException ex) {}
                        } else if (line.get(1).equals("OR")) {
                            try {
                                int ORnum1 = Integer.parseInt(line.get(0));  
                                int ORnum2 = Integer.parseInt(line.get(2));
                                int OR1To2 = ORnum1 | ORnum2;   
                                line.set(0, Integer.toString(OR1To2));
                                line.remove(1); line.remove(1);
                            } catch (NumberFormatException ex) {}
                        }
                    }
                }
            } catch (FileNotFoundException ex) {}
        } 
        return 0;
    }

    public static boolean isVarDefined(ArrayList<ArrayList<String>> dictionary, String var) {
        return !dictionary.get(1).get(dictionary.get(0).indexOf(var)).equals("null");
    }

    public static String varDef(ArrayList<ArrayList<String>> dictionary, String var) {
        return dictionary.get(1).get(dictionary.get(0).indexOf(var));
    }

    //     }
    //     try {    
    //         while (fileScanner1.hasNextLine()) {
    //             ArrayList<String> line = new ArrayList<String>(Arrays.asList(fileScanner1.nextLine().split(" ")));
    //             // does all operation except for assignment
    //             if (line.get(0).equals("NOT")) {
    //                 try {
    //                     int number = Integer.parseInt(line.get(1));
    //                     String NOTnum = Integer.toString(~number);  
    //                     line.set(1, NOTnum);
    //                     line.remove(0);
    //                 } catch (NumberFormatException ex) {}
    //             } else if (line.get(1).equals("RSHIFT")) {
    //                 try {
    //                     int RSHIFTnum1 = Integer.parseInt(line.get(0));  
    //                     int RSHIFTnum2 = Integer.parseInt(line.get(2));
    //                     int RSHIFT1To2 = RSHIFTnum1 >> RSHIFTnum2;   
    //                     line.set(0, Integer.toString(RSHIFT1To2));
    //                     line.remove(1); line.remove(1);
    //                 } catch (NumberFormatException ex) {}
    //             } else if (line.get(1).equals("LSHIFT")) {
    //                 try {
    //                     int LSHIFTnum1 = Integer.parseInt(line.get(0));  
    //                     int LSHIFTnum2 = Integer.parseInt(line.get(2));
    //                     int LSHIFT1To2 = LSHIFTnum1 << LSHIFTnum2;   
    //                     line.set(1, Integer.toString(LSHIFT1To2));
    //                     line.remove(1); line.remove(1);
    //                 } catch (NumberFormatException ex) {}
    //             } else if (line.get(1).equals("AND")) {
    //                 try {
    //                     int ANDnum1 = Integer.parseInt(line.get(0));  
    //                     int ANDnum2 = Integer.parseInt(line.get(2));
    //                     int AND1To2 = ANDnum1 & ANDnum2;   
    //                     line.set(0, Integer.toString(AND1To2));
    //                     line.remove(1); line.remove(1);
    //                 } catch (NumberFormatException ex) {}
    //             } else if (line.get(1).equals("OR")) {
    //                 try {
    //                     int ORnum1 = Integer.parseInt(line.get(0));  
    //                     int ORnum2 = Integer.parseInt(line.get(2));
    //                     int OR1To2 = ORnum1 | ORnum2;   
    //                     line.set(0, Integer.toString(OR1To2));
    //                     line.remove(1); line.remove(1);
    //                 } catch (NumberFormatException ex) {}
    //             } 

    //             String val = line.get(0);
    //             String var = line.get(2);

    //             Scanner fileScanner2 = new Scanner(file);
    //             while (fileScanner2.hasNextLine()) {
    //                 ArrayList<String> line2 = new ArrayList<String>(Arrays.asList(fileScanner2.nextLine().split(" ")));
    //                 if (line2.indexOf(var) != -1) {
    //                     line2.set(line2.indexOf(var), val);
    //                 }
    //             } 
    //             System.out.println(line.toString());

    //         }
    //         fileScanner1.close();
    //     } catch (FileNotFoundException ex) {}
    //     return a;
    // }

    public static int xyz(Scanner filScanner) {
        int a = 0;
        return a;
    }
}
