import java.util.*;
import java.io.*;
import java.security.*;

public class Day4 {
    public static void main(String[] args) {
        int test = Integer.parseInt(args[0]);
        File file = new File("inputs/input4.txt");
        try {
            Scanner fileScanner = new Scanner(file);
            if (test == 1) {System.out.println(part1(fileScanner));}
            else if (test == 2) {System.out.println(part2(fileScanner));}
        } catch (FileNotFoundException ex) {}
    }
    
    public static int part1(Scanner fileScanner) {
        String secretKey = fileScanner.nextLine();

        int hashNumber = 1;
        while (true) {
            String hash = secretKey + hashNumber;
            String hexOutput = MD5ByteArrayToHexString(hash);
            if (hexOutput.substring(0,5).equals("00000")) {return hashNumber;} 
            else {hashNumber++;}
        }
    }

    public static int part2(Scanner fileScanner) {
        String secretKey = fileScanner.nextLine();
        
        int hashNumber = 1;
        while (true) {
            String hash = secretKey + hashNumber;
            String hexOutput = MD5ByteArrayToHexString(hash);
            if (hexOutput.substring(0,6).equals("000000")) {return hashNumber;} 
            else {hashNumber++;}
        }
    }

    public static String MD5ByteArrayToHexString(String hash) {
        String hexOutput = "";
        try {
            byte[] bytesOfMessage = hash.getBytes();
            MessageDigest md = MessageDigest.getInstance("MD5");
            byte[] theMD5digest = md.digest(bytesOfMessage);
            for (int i = 0; i < theMD5digest.length; i++) {
                hexOutput += String.format("%02X", theMD5digest[i]); 
            }
        } catch (NoSuchAlgorithmException ex) {}

        return hexOutput;
    }
}