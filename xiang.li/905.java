class Solution {
    public int[] sortArrayByParity(int[] A) {
        int i = 0;
        int j = A.length - 1;
        int temp;
        while(i < j){
            if(A[i] % 2 != 0) {
                temp = A[i];
                A[i] = A[j];
                A[j] = temp;
                j--;
            } else {
                i++;
            }
        }
        return A;
    }
}
