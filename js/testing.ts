// Var setup

function print(input) {
        return console.log(input);
} // I didn't like how console.log looked so I replaced it with print

function lZero(num) { 
    if (num >= 0) {return false} 
    else if (num <= 0) { return true }
}

print(lZero(-1));