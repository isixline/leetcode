var sumZero = function(n) {
    var result = n % 2 === 0 ? [] : [0]
    
    n /= 2
    
    while(n >= 1) {
        result.push(n, 0 - n)
        n -= 1
    }
    
    return result
};