var decompressRLElist = function(nums) {
    var result = []
    for (var i = 0; i < nums.length; i += 2) {
        for (var j = 0; j < nums[i]; j++) {
            result.push(nums[i + 1])
        }
    }
    return result
};