var smallerNumbersThanCurrent = function(nums) {
  var result = nums
    .map((item, index) => ({ index, num: item, lessCount: 0 }))
    .sort((a, b) => a.num - b.num);

  for (var i = 1; i < result.length; i++) {
    if (result[i].num === result[i - 1].num) {
      result[i].lessCount = result[i - 1].lessCount;
    } else {
      result[i].lessCount = i;
    }
  }

  return result.sort((a, b) => a.index - b.index).map(item => item.lessCount);
};
