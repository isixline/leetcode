var sortArrayByParityII = function(A) {
    var length = A.length
    var evenIndex = 0
    var oddIndex = 1
    
    while(evenIndex <= length && oddIndex <= length) {
        while(evenIndex <= length && A[evenIndex] % 2 === 0) evenIndex += 2
        while(oddIndex <= length && A[oddIndex] % 2 !== 0 ) oddIndex += 2
        
        if (evenIndex <= length && oddIndex <= length) {
            var temp = A[evenIndex]
            A[evenIndex] = A[oddIndex]
            A[oddIndex] = temp
        }
       
    }
    
    return A
};