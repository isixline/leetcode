var findNumbers = function(nums) {
    var result = 0
    
    for (var item of nums) {
        if ((item + '').length % 2 === 0) {
            result += 1
        }
    }
    
    return result
};