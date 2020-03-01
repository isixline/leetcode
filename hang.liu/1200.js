var minimumAbsDifference = function(arr) {
  arr.sort((a, b) => a - b);

  var result = Array(arr.length - 1);

  var min = arr[1] - arr[0];

  for (var i = 0; i < arr.length - 1; i++) {
    var diff = arr[i + 1] - arr[i];
    result[i] = [diff, arr[i], arr[i + 1]];
    if (diff < min) min = diff;
  }

  return result.filter(item => item[0] === min).map(item => [item[1], item[2]]);
};
