// Var setup

function print(input) {
        return console.log(input);
} // I didn't like how console.log looked so I replaced it with print

var i;
var testArray = [14, 35, 6, 1, 34, 54];
function minMax(arr) {
    let sortedArray = [];
	for (i = 0; i < arr.length; i++) {
        sortedArray[i] = Math.min(arr[i], arr[i+1]);

    } 
    return sortedArray[]
}

print(minMax(testArray));