var minTimeToVisitAllPoints = function(points) {
    var result = 0
    for (var i = 1; i < points.length; i++) {
        result += minTimeToVisitPoint(points[i - 1], points[i])
    }
     
     return result
 };
 
 var minTimeToVisitPoint = function(point1, point2) {
     var xDiff = Math.abs(point1[0] - point2[0])
     var yDiff = Math.abs(point1[1] - point2[1])
     
     return xDiff + yDiff - Math.min(xDiff, yDiff)
 }