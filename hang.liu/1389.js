var createTargetArray = function(nums, index) {
  var length = nums.length;
  var result = [];

  for (var i = 0; i < length; i++) {
    result.splice(index[i], 0, nums[i]);
  }

  return result;
};
