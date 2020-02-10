var flipAndInvertImage = function(A) {
    var result = []
    
    for (var item of A) {
        var temp = []
        
        for (var num of item) {
            temp.unshift(1 - num)
        }
        
        result.push(temp)
        
    } 
    
    return result
};