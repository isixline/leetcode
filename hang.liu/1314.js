var matrixBlockSum = function(mat, K) {
  var m = mat.length;
  var n = mat[0].length;

  var rangeSum = Array(m + 1)
    .fill(0)
    .map(x => Array(n + 1).fill(0));

  for (var i = 0; i < m; i++) {
    for (var j = 0; j < n; j++) {
      rangeSum[i + 1][j + 1] =
        rangeSum[i + 1][j] + rangeSum[i][j + 1] - rangeSum[i][j] + mat[i][j];
    }
  }

  var result = Array(m)
    .fill(0)
    .map(x => Array(n).fill(0));
  for (var i = 0; i < m; i++) {
    for (var j = 0; j < n; j++) {
      var r1 = Math.max(0, i - K);
      var c1 = Math.max(0, j - K);
      var r2 = Math.min(m, i + K + 1);
      var c2 = Math.min(n, j + K + 1);
      result[i][j] =
        rangeSum[r2][c2] -
        rangeSum[r2][c1] -
        rangeSum[r1][c2] +
        rangeSum[r1][c1];
    }
  }

  return result;
};
