class Solution {
    public int[][] flipAndInvertImage(int[][] A) {
        for(int i = 0;i < A.length;i++) {
            for(int js = 0, je = A[0].length - 1;je >= js;js++, je--) {
                if(A[i][js] == A[i][je]) {
                    A[i][js] = A[i][je] = A[i][je] == 0 ? 1: 0;
                }
            }
        }
        return A;
    }
}
