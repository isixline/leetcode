var mctFromLeafValues = function(arr) {
  var result = 0;
  while (arr.length > 1) {
    var current = 0;
    for (var i = 1; i < arr.length - 1; i++) {
      if (arr[i] * arr[i + 1] < arr[current] * arr[current + 1]) {
        current = i;
      }
    }
    result += arr[current] * arr[current + 1];
    arr.splice(current, 2, Math.max(arr[current], arr[current + 1]));
  }
  return result;
};
