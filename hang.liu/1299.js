var replaceElements = function(arr) {
    arr.push(-1)
    
    for (var i = arr.length - 2; i > 0; i--) {
        arr[i] = Math.max(arr[i], arr[i + 1])
    }
    
    return arr.slice(1)
};