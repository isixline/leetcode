var sortArrayByParity = function(A) {
    var left = 0
    var right = A.length - 1
    
    while (left < right) {
        while(A[left] % 2 !== 1 && left < right) {
            left++
        }
        
        while(A[right] % 2 !== 0 && left < right) {
            right--
        }
        
        var temp = A[left]
        A[left] = A[right]
        A[right] = temp
    }
    
    return A
    
};