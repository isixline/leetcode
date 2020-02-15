var heightChecker = function(heights) {
  var sortedHeights = heights.slice().sort((a, b) => a - b);

  var result = 0;

  for (var i = 0; i < heights.length; i++) {
    if (heights[i] !== sortedHeights[i]) result++;
  }

  return result;
};
