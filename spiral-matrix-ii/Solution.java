class Solution {
    public int[][] generateMatrix(int n) {
        // prelude: allocate the matrix
        int[][] matrix = new int[n][];
        for (int i = 0; i < n; i++) {
            matrix[i] = new int[n];
        }
        
        int leftCol = 0;
        int rightCol = n - 1;
        int topRow = 0;
        int botRow = n - 1;
        
        int counter = 1;
        for (int i = 0; i < n / 2; i++) {
            //top row
            for (int col = leftCol; col <= rightCol; col++) {
                matrix[topRow][col] = counter++;
            }
            topRow++;
            
            //right column
            for (int row = topRow; row <= botRow; row++) {
                matrix[row][rightCol] = counter++;
            }
            rightCol--;
            
            //bottom row
            for (int col = rightCol; col >= leftCol; col--) {
                matrix[botRow][col] = counter++;
            }
            botRow--;
            
            //left column
            for (int row = botRow; row >= topRow; row--) {
                matrix[row][leftCol] = counter++;
            }
            leftCol++;
        }
        
        // if n is odd do the middle
        if (n % 2 == 1) {
            matrix[leftCol][topRow] = counter;
        }
        
        return matrix;
    }
}
