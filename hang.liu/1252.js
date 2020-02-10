var oddCells = function(n, m, indices) {
    var rows = new Array(n).fill(0)
    var cols = new Array(m).fill(0)
    
    for (var item of indices) {
        rows[item[0]] += 1
        cols[item[1]] += 1
    }
    
    var oddOfRows =  rows.reduce((pre, item) => pre + item % 2, 0)
    var oddOfCols =  cols.reduce((pre, item) => pre + item % 2, 0)
    
    return oddOfRows * m + oddOfCols * n - 2 * oddOfRows * oddOfCols
};