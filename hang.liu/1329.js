var diagonalSort = function(mat) {
  var m = mat.length;
  var n = mat[0].length;

  for (var k = 0; k < Math.min(m, n); k++) {
    var swap = false;
    for (var i = 0; i < m; i++) {
      for (var j = 0; j < n; j++) {
        if (i + 1 < m && j + 1 < n && mat[i + 1][j + 1] < mat[i][j]) {
          var temp = mat[i][j];
          mat[i][j] = mat[i + 1][j + 1];
          mat[i + 1][j + 1] = temp;
          swap = true;
        }
      }
    }
    if (!swap) break;
  }
  return mat;
};
