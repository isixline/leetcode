var kWeakestRows = function(mat, k) {
  return mat
    .map((item, index) => [index, item.reduce((total, num) => total + num, 0)])
    .sort((a, b) => a[1] - b[1] || a[0] - b[0])
    .slice(0, k)
    .map(item => item[0]);
};
