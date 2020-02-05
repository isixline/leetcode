class Solution {
    public int minTimeToVisitAllPoints(int[][] points) {
        int length = 0;
        for(int i = 0;i < points.length - 1;i++) {
            length += max(points[i][0], points[i][1], points[i + 1][0], points[i + 1][1]);
        }
        return length;
    }
    int max(int x1, int y1, int x2, int y2) {
        int x = abs(x1 - x2);
        int y = abs(y1 - y2);
        return x > y  ? x : y;
    }
    int abs(int a) {
        return (a < 0) ? -a : a;
    }
}
