var minFallingPathSum = function(A) {
  var m = A.length;
  var n = A[0].length;

  for (var i = 1; i < m; i++) {
    for (var j = 0; j < n; j++) {
      const temp = [];
      if (j - 1 >= 0) temp.push(A[i - 1][j - 1]);
      if (j + 1 < n) temp.push(A[i - 1][j + 1]);
      temp.push(A[i - 1][j]);
      A[i][j] += Math.min(...temp);
    }
  }

  return Math.min(...A[m - 1]);
};
