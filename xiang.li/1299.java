class Solution {
    public int[] replaceElements(int[] arr) {
        int greatest = -1;
        int temp = -1;
        for(int i = arr.length - 1;i >= 0;i--){
            greatest = temp > greatest ? temp : greatest;
            temp = arr[i];
            arr[i] = greatest;
        }
        return arr;
    }
}
