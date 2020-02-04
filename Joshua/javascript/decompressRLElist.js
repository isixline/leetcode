var decompressRLElist = function(nums) {
    let decArr = new Array();
    
    for (var i = 0; i < nums.length; i+=2) {
        let v = nums[i];
        
        while(v > 0) {
            decArr.push(nums[i+1]);
            v--;
        }
    }
    
    return decArr;
};
