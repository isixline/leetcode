var queensAttacktheKing = function(queens, king) {
    var length = 8
    var result = [];
    var options = [-1, 0, 1];
  
    var optionsLength = options.length;
    for (var j = 0; j < optionsLength; j++) {
      for (var k = 0; k < optionsLength; k++) {
        for (var i = 1; i <= length; i++) {
          var temp = [king[0] + i * options[j], king[1] + i * options[k]];
          if (temp[0] < 0 || temp[0] >= length || temp[1] < 0 || temp[1] >= length) continue 
          if (queens.filter(item => item[0] === temp[0] && item[1] === temp[1]).length) {
            result.push(temp);
            break;
          }
        }
      }
    }
      
   return result
  };