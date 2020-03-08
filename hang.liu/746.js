var minCostClimbingStairs = function(cost) {
  var result = cost.slice();
  result.unshift(0);

  var length = result.length;

  for (var i = 2; i < length; i++) {
    result[i] = result[i] + Math.min(result[i - 1], result[i - 2]);
  }

  return Math.min(result[length - 1], result[length - 2]);
};
