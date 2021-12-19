import java.util.*;
import java.io.*;

public class Day4 {
    public static void main(String[] args) {
        //int test = Integer.parseInt(args[0]);
        int test = 2;
        File file = new File("2021/inputs/input4.txt");
        try {
            Scanner fileScanner = new Scanner(file);
            if (test == 1) {System.out.println(part1(fileScanner));}
            else if (test == 2) {System.out.println(part2(fileScanner));}
        } catch (FileNotFoundException ex) {}
    }

    public static int part1(Scanner fileScanner) {
        // get the inputs
        String[] markersStr = fileScanner.nextLine().split(",");
        int[] markers = new int[markersStr.length];
        for (int i = 0; i < markersStr.length; i++) {markers[i] = Integer.parseInt(markersStr[i]);}
        
        
        // get the boards as an ArrayList 
        int bb = 0;

        ArrayList<ArrayList<ArrayList<Integer>>> boardsArrayList = new ArrayList<ArrayList<ArrayList<Integer>>>(); 
        while (fileScanner.hasNextInt()) {
            boardsArrayList.add(new ArrayList<ArrayList<Integer>>());
            for (int l = 0; l < 5; l++) {
                boardsArrayList.get(bb).add(new ArrayList<Integer>());
                for (int c = 0; c < 5; c++) {
                    boardsArrayList.get(bb).get(l).add(fileScanner.nextInt());
                }
            }
            bb++;
        }

        // get the boards as an Array 
        int[][][] boards = new int[boardsArrayList.size()][boardsArrayList.get(0).size()][boardsArrayList.get(0).get(0).size()]; 
        for (int b = 0; b < boardsArrayList.size(); b++) {
            for (int l = 0; l < boardsArrayList.get(b).size(); l++) {
                for (int c = 0; c < boardsArrayList.get(b).get(l).size(); c++) {
                    boards[b][l][c] = boardsArrayList.get(b).get(l).get(c);
                }
            }
        }
        
        // play bingo
        int marker = -1;
        int winningBoard = -1; 

        for (int m = 0; m < markers.length; m++) {
            marker = markers[m];
            placeMarker(boards, marker);

            boolean win = false;
            for (int b = 0; b < boards.length; b++) {
                win = checkForWin(boards[b]);
                if (win) {winningBoard = b; break;}
            }
            if (win) {break;}
        }
        return finalScore(boards[winningBoard], marker);
    }



    public static int part2(Scanner fileScanner) {
        // get the inputs
        String[] markersStr = fileScanner.nextLine().split(",");
        int[] markers = new int[markersStr.length];
        for (int i = 0; i < markersStr.length; i++) {markers[i] = Integer.parseInt(markersStr[i]);}
        
        
        // get the boards as an ArrayList 
        int bb = 0;

        ArrayList<ArrayList<ArrayList<Integer>>> boardsArrayList = new ArrayList<ArrayList<ArrayList<Integer>>>(); 
        while (fileScanner.hasNextInt()) {
            boardsArrayList.add(new ArrayList<ArrayList<Integer>>());
            for (int l = 0; l < 5; l++) {
                boardsArrayList.get(bb).add(new ArrayList<Integer>());
                for (int c = 0; c < 5; c++) {
                    boardsArrayList.get(bb).get(l).add(fileScanner.nextInt());
                }
            }
            bb++;
        }

        // get the boards as an Array 
        int[][][] boards = new int[boardsArrayList.size()][boardsArrayList.get(0).size()][boardsArrayList.get(0).get(0).size()]; 
        for (int b = 0; b < boardsArrayList.size(); b++) {
            for (int l = 0; l < boardsArrayList.get(b).size(); l++) {
                for (int c = 0; c < boardsArrayList.get(b).get(l).size(); c++) {
                    boards[b][l][c] = boardsArrayList.get(b).get(l).get(c);
                }
            }
        }
        
        // play bingo
        ArrayList<Integer> boardsWon = new ArrayList<Integer>(); // to not check the same board twice
        ArrayList<Integer> markersWon = new ArrayList<Integer>();
        int[][] winningBoardCopy = new int[5][5]; // to check for win later on becuase it will be curropted by other placed markers

        for (int m = 0; m < markers.length; m++) {
            int marker = markers[m];
            placeMarker(boards, marker);

            for (int b = 0; b < boards.length; b++) {
                if (boardsWon.indexOf(b) == -1) {
                    boolean win = checkForWin(boards[b]);
                    if (win) {boardsWon.add(b); markersWon.add(marker); winningBoardCopy = Arrays.copyOf(boards[b], boards[b].length);}
                }
            }
        }
        return finalScore(winningBoardCopy, markersWon.get(markersWon.size()-1));
    }


    public static void placeMarker(int[][][] boards, int marker) {
        for (int b = 0; b < boards.length; b++) {
            for (int l = 0; l < 5; l++) {
                for (int c = 0; c < 5; c++) {
                    if (boards[b][l][c] == marker) {boards[b][l][c] = -1;}
                }
            }
        }
    }

    public static boolean checkForWin(int[][] board) {
        for (int l = 0; l < 5; l++) {
            boolean horizontalBingo = true;
            for (int c = 0; c < 5; c++) {
                if (board[l][c] >= 0) {horizontalBingo = false; break;} 
            }
            if (horizontalBingo) {return true;}
        }

        for (int c = 0; c < 5; c++) {
            boolean verticalBingo = true;      
            for (int l = 0; l < 5; l++) {
                if (board[l][c] >= 0) {verticalBingo = false; break;} 
            }
            if (verticalBingo) {return true;}
        }

        return false;
    }

    public static int finalScore(int[][] board, int marker) {
        int score = 0;
        for (int l = 0; l < 5; l++) {
            for (int c = 0; c < 5; c++) {
                if (board[l][c] >= 0) {score += board[l][c];}
            }
        }
        score *= marker;
        return score;
    }
}
