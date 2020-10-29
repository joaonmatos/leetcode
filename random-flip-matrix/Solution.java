class Solution {
    private final HashSet<Integer> used = new HashSet<>();

    private final int rows;
    private final int cols;
    private final int max;
    private final Random rand = new Random();

    public Solution(int n_rows, int n_cols) {
        this.rows = n_rows;
        this.cols = n_cols;
        max = rows * cols;
    }
    
    public int[] flip() {
        int i = rand.nextInt(max);
        while (used.contains(i))
            i = rand.nextInt(max);
        used.add(i);
        int[] out = { i / cols, i % cols };
        return out;
    }
    
    public void reset() {
        used.clear();
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * Solution obj = new Solution(n_rows, n_cols);
 * int[] param_1 = obj.flip();
 * obj.reset();
 */
