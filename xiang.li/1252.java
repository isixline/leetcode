class Solution {
    public int oddCells(int n, int m, int[][] indices) {
        int[] lm = new int[m];
        int[] ln = new int[n];
        for(int i = 0;i < indices.length;i++) {
            ln[indices[i][0]]++;
            lm[indices[i][1]]++;
        }
        int lmodd = 0;
        int lmeven = 0;
        int lnodd = 0;
        int lneven = 0;
        for(int i = 0;i < m;i++) {
            if (lm[i] % 2 == 0) {
                lmeven++;
            } else {
                lmodd++;
            }
        }

        for(int i = 0;i < n;i++) {
            if (ln[i] % 2 == 0) {
                lneven++;
            } else {
                lnodd++;
            }
        }
        return lmodd * lneven + lmeven * lnodd;
    }
}
