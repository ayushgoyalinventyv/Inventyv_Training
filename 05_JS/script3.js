let sum=0;

first_func();

function first_func()
{
    console.log("First Function");
    let arr1=[Symbol(1),Symbol(2),Symbol(3),Symbol(4),Symbol(5)];
    let first_element = arr1.shift()

    second_func(first_element,arr1);
}


function second_func(first_element, rest_of_array)
{
    let arr2=[Symbol(6),Symbol(7),Symbol(8),Symbol(9),Symbol(10)];
    arr2=[first_element, ...arr2, ...rest_of_array];
    console.log(arr2);

    for (let i=0 ; i<arr2.length ; i++){
        sum=sum+Number(arr2[i].description);
    }
    console.log(sum);

    
}

const promise = new Promise((resolve, reject) => {
    if(sum>35){
        resolve("Promise Resolved");
    }
    else{
        reject("Promise Rejected");
    }
});


promise
.then((message) => {
    console.log(message);
})
.catch((message) => {
    console.log(message);
});




